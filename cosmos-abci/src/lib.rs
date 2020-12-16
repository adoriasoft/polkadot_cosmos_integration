//! The pallet for interact with cosmos abci interface.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(unused_must_use)]
use frame_support::{
    codec::{Decode, Encode},
    debug, decl_module, decl_storage,
    dispatch::{DispatchResult, Vec},
    weights::Weight,
};
use frame_system::{
    self as system, ensure_none, ensure_signed,
    offchain::{AppCrypto, CreateSignedTransaction},
    RawOrigin,
};
use pallet_session as session;
use pallet_sudo as sudo;
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
    offchain::{
        storage::StorageValueRef,
        storage_lock::{BlockAndTime, StorageLock},
    },
    traits::{Convert, SaturatedConversion},
    transaction_validity::{
        InvalidTransaction, TransactionSource, TransactionValidity, ValidTransaction,
    },
    DispatchError, RuntimeDebug,
};
use sp_runtime_interface::runtime_interface;
use sp_std::prelude::*;

/// Balance type for pallet.
pub type Balance = u64;
/// Session index that define in pallet_session.
type SessionIndex = u32;
/// The optional ledger type.
type OptionalLedger<AccountId> = Option<(AccountId, Balance)>;

/// Priority for unsigned transaction.
pub const UNSIGNED_TXS_PRIORITY: u64 = 100;

/// The KeyType ID.
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"abci");

/// Type helpers.
pub mod utils;
//
/// Based on the above `KeyTypeId` we need to generate a pallet-specific crypto type wrapper.
/// We can utilize the supported crypto kinds (`sr25519`, `ed25519` and `ecdsa`) and augment
/// them with the pallet-specific identifier.
pub mod crypto {
    use crate::KEY_TYPE;
    use frame_support::codec::Decode;
    use sp_core::sr25519::Signature as Sr25519Signature;
    use sp_runtime::app_crypto::{app_crypto, sr25519};
    use sp_runtime::traits::Verify;
    use sp_runtime::{MultiSignature, MultiSigner};

    app_crypto!(sr25519, KEY_TYPE);

    #[derive(Decode, Default)]
    pub struct ABCIAuthId;
    /// Implemented for ocw-runtime.
    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for ABCIAuthId {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }

    /// Implemented for mock runtime in test.
    impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
        for ABCIAuthId
    {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }
}

/// The CosmosAbci trait.
pub trait CosmosAbci {
    fn check_tx(data: Vec<u8>) -> Result<u64, DispatchError>;
    fn deliver_tx(data: Vec<u8>) -> DispatchResult;
}

/// The pallet configuration trait.
pub trait Trait:
    CreateSignedTransaction<Call<Self>> + pallet_session::Trait + pallet_sudo::Trait
{
    type AuthorityId: AppCrypto<Self::Public, Self::Signature> + Default + Decode;
    type Call: From<Call<Self>>;
    type Subscription: SubscriptionManager;
}

/// The pallet Subscription manager trait.
pub trait SubscriptionManager {
    fn on_check_tx(data: Vec<u8>) -> DispatchResult;
    fn on_deliver_tx(data: Vec<u8>) -> DispatchResult;
}

impl SubscriptionManager for () {
    fn on_check_tx(_: Vec<u8>) -> DispatchResult { Ok(()) }
    fn on_deliver_tx(_: Vec<u8>) -> DispatchResult { Ok(()) }
}

impl<T: Trait> sp_runtime::BoundToRuntimeAppPublic for Module<T>
where
    <T as Trait>::AuthorityId: sp_runtime::RuntimeAppPublic,
{
    type Public = T::AuthorityId;
}

/// The ABCITxs struct that keept map of txs.
#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct ABCITxs {
    data_array: Vec<Vec<u8>>,
}

decl_storage! {
    trait Store for Module<T: Trait> as ABCITxStorage {
        ABCITxStorage get(fn abci_tx): map hasher(blake2_128_concat) T::BlockNumber => ABCITxs;
        CosmosAccounts get(fn cosmos_accounts): map hasher(blake2_128_concat) utils::CosmosAccountId => T::AccountId;// Option<T::AccountId> = None;
        AccountLedger get(fn account_ledgers): map hasher(blake2_128_concat) T::AccountId => OptionalLedger<T::AccountId>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Block initialization.
        fn on_initialize(block_number: T::BlockNumber) -> Weight {
            0
        }

        // Block finalization.
        fn on_finalize(block_number: T::BlockNumber) {
        }

        // Simple tx.
        #[weight = 0]
        fn handle_origin_account(origin) -> Result<(), DispatchError> {
            let origin_signed = ensure_signed(origin)?;
            <AccountLedger<T>>::insert(&origin_signed, Some((&origin_signed, 0)));
            let cosmos_account = (&origin_signed, cosmos_account_id.clone());
            <CosmosAccounts<T>>::insert(cosmos_account_id.clone(), &origin_signed);
            // todo
            // Save cosmos node accounts into rocks_db storage.
            Ok(())
        }

        // Transaction dispatch.
        #[weight = 0]
        pub fn abci_transaction(origin, data: Vec<u8>) -> DispatchResult {
            let _ = ensure_none(origin)?;

            Self::call_abci_transaction(data)?;
            Ok(())
        }

        // Offchain worker logic.
        fn offchain_worker(block_number: T::BlockNumber) {
            if block_number.saturated_into() as i64 != 0 {
                // hash of the current block
                let block_hash = <system::Module<T>>::block_hash(block_number);
                // hash of the previous block
                let parent_hash = <system::Module<T>>::parent_hash();
                // hash of the extrinsics root
                let extrinsics_root = <system::Module<T>>::extrinsics_root();
                Self::call_offchain_worker(block_number, block_hash, parent_hash, extrinsics_root);
            }
        }
    }
}

/// Implementation of additional methods for pallet configuration trait.
impl<T: Trait> Module<T> {
    // The abci transaction call.
    pub fn call_abci_transaction(data: Vec<u8>) -> DispatchResult {
        let block_number = <system::Module<T>>::block_number();
        let mut abci_txs: ABCITxs = <ABCITxStorage<T>>::get(block_number);
        abci_txs.data_array.push(data);
        <ABCITxStorage<T>>::insert(block_number, abci_txs);
        Ok(())
    }

    // Called on offchain worker executive.
    pub fn call_offchain_worker(
        block_number: T::BlockNumber,
        block_hash: T::Hash,
        parent_hash: T::Hash,
        extrinsics_root: T::Hash,
    ) {
        debug::info!("call_offchain_worker(), block_number: {:?}", block_number);

        Self::call_on_init_chain();

        Self::call_on_initialize(block_number, block_hash, parent_hash, extrinsics_root);

        let abci_txs: ABCITxs = <ABCITxStorage<T>>::get(block_number);
        for abci_tx in abci_txs.data_array {
            debug::info!("call_offchain_worker(), abci_tx: {:?}", abci_tx);
            let _ = <Self as CosmosAbci>::deliver_tx(abci_tx)
                .map_err(|e| debug::error!("deliver_tx() error: {:?}", e))
                .unwrap();
        }
        Self::call_on_finalize(block_number);
    }

    pub fn call_on_init_chain() {
        let storage = StorageValueRef::persistent(b"abci-local-storage:init_chain_info");

        if let Some(Some(init_chain_info)) = storage.get::<bool>() {
            if !init_chain_info {
                abci_interface::init_chain().unwrap();
            }
        } else {
            abci_interface::init_chain().unwrap();
        }

        if let Ok(_guard) =
            StorageLock::<BlockAndTime<Self>>::new(b"abci-local-storage:lock").try_lock()
        {
            storage.set(&true);
        }
    }

    // Called on block initialize.
    pub fn call_on_initialize(
        block_number: T::BlockNumber,
        block_hash: T::Hash,
        parent_hash: T::Hash,
        extrinsics_root: T::Hash,
    ) -> bool {
        if let Err(err) = abci_interface::begin_block(
            block_number.saturated_into() as i64,
            block_hash.as_ref().to_vec(),
            parent_hash.as_ref().to_vec(),
            extrinsics_root.as_ref().to_vec(),
        ) {
            panic!("Begin block failed: {:?}", err);
        }
        true
    }

    /// Called on block finalize.
    pub fn call_on_finalize(block_number: T::BlockNumber) -> bool {
        match abci_interface::end_block(block_number.saturated_into() as i64) {
            Ok(_) => {
                match abci_interface::commit() {
                    Err(err) => {
                        panic!("Commit failed: {:?}", err);
                    }
                    _ => true,
                }
            }
            Err(err) => {
                panic!("End block failed: {:?}", err);
            }
        }
    }

    pub fn update_keys_for_account(validator_id: T::AccountId) {
        let proof = vec![];
        let set_keys_response = <session::Module<T>>::set_keys(
            RawOrigin::Signed(validator_id.clone()).into(),
            T::Keys::default(),
            proof,
        );
        match set_keys_response {
            Ok(_) => {
                debug::info!("Set new keys for validator {:?}", validator_id);
            }
            Err(err) => {
                debug::info!("Set keys for validator error {:?}", err);
            }
        }
    }
}

/// The implementation of ValidateUnsigned trait for module.
impl<T: Trait> frame_support::unsigned::ValidateUnsigned for Module<T> {
    type Call = Call<T>;

    fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
        let valid_tx = |provide| {
            ValidTransaction::with_tag_prefix("cosmos-abci")
                .priority(UNSIGNED_TXS_PRIORITY)
                .and_provides([&provide])
                .longevity(3)
                .propagate(true)
                .build()
        };

        match call {
            Call::abci_transaction(_number) => valid_tx(b"submit_abci_transaction".to_vec()),
            _ => InvalidTransaction::Call.into(),
        }
    }
}

/// The implementation for CosmosAbci trait for pallet.
impl<T: Trait> CosmosAbci for Module<T> {
    fn check_tx(data: Vec<u8>) -> Result<u64, DispatchError> {
        <Self as SubscriptionManager>::on_check_tx(data)?;
        abci_interface::check_tx(data)
    }

    fn deliver_tx(data: Vec<u8>) -> DispatchResult {
        <Self as SubscriptionManager>::on_deliver_tx(data)?;
        abci_interface::deliver_tx(data)
    }
}

sp_api::decl_runtime_apis! {
    /// ExtrinsicConstructionApi trait for define broadcast_abci_tx method.
    pub trait ExtrinsicConstructionApi {
        fn broadcast_abci_tx(data: Vec<u8>);
    }
}

/// AbciInterface trait with runtime_interface macro.
#[runtime_interface]
pub trait AbciInterface {
    fn init_chain() -> Result<(), DispatchError> {
        let genesis =
            pallet_abci::utils::parse_cosmos_genesis_file(&pallet_abci::utils::get_abci_genesis())
                .map_err(|_| "failed to get cosmos genesis file")?;

        pallet_abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")?
            .init_chain(
                genesis.time_seconds,
                genesis.time_nanos,
                &genesis.chain_id,
                genesis.pub_key_types,
                genesis.max_bytes,
                genesis.max_gas,
                genesis.max_age_num_blocks,
                genesis.max_age_duration,
                genesis.app_state_bytes,
            )
            .map_err(|_| "init chain failed")?;

        Ok(())
    }

    fn check_tx(data: Vec<u8>) -> Result<u64, DispatchError> {
        let result = pallet_abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")?
            .check_tx(data)
            .map_err(|_| "check_tx failed")?;

        if result.get_code() != 0 {
            Err(sp_runtime::DispatchError::Module {
                index: u8::MIN,
                error: result.get_code() as u8,
                message: Some("Invalid tx data."),
            })
        } else {
            let dif = result.get_gas_wanted() - result.get_gas_used();
            Ok(dif as u64)
        }
    }

    fn deliver_tx(data: Vec<u8>) -> DispatchResult {
        let _result = pallet_abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")?
            .deliver_tx(data)
            .map_err(|_| "deliver_tx failed")?;
        Ok(())
    }

    fn begin_block(
        height: i64,
        hash: Vec<u8>,
        last_block_id: Vec<u8>,
        proposer_address: Vec<u8>,
    ) -> DispatchResult {
        let _result = pallet_abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")?
            .begin_block(height, hash, last_block_id, proposer_address)
            .map_err(|_| "begin_block failed")?;
        Ok(())
    }

    fn end_block(height: i64) -> DispatchResult {
        let _result = pallet_abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")?
            .end_block(height)
            .map_err(|_| "end_block failed")?;
        // debug::info!("Result: {:?}", result);
        let cosmos_node_validators = _result.get_validator_updates();
        debug::info!("Cosmos validators {:?}", cosmos_node_validators);
        // todo
        // Save cosmos node validators into storage.
        Ok(())
    }

    fn commit() -> DispatchResult {
        let _result = pallet_abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")?
            .commit()
            .map_err(|_| "commit failed")?;
        Ok(())
    }
}

impl<T: Trait> sp_runtime::offchain::storage_lock::BlockNumberProvider for Module<T> {
    type BlockNumber = T::BlockNumber;
    fn current_block_number() -> Self::BlockNumber {
        <frame_system::Module<T>>::block_number()
    }
}

impl<T: Trait> Convert<T::AccountId, Option<T::AccountId>> for utils::StashOf<T> {
    fn convert(controller: T::AccountId) -> Option<T::AccountId> {
        let account_ledger: OptionalLedger<T::AccountId> = <Module<T>>::account_ledger(&controller);
        match account_ledger {
            Some(_ledger) => Some(_ledger.0),
            None => Some(controller),
        }
    }
}

impl<T: Trait> Convert<T::AccountId, Option<utils::Exposure<T::AccountId, Balance>>>
    for utils::ExposureOf<T>
{
    fn convert(_validator: T::AccountId) -> Option<utils::Exposure<T::AccountId, Balance>> {
        Some(utils::Exposure {
            total: 0,
            own: 0,
            others: vec![],
        })
    }
}

impl<T: Trait> pallet_session::SessionManager<T::AccountId> for Module<T> {
    fn new_session(new_index: SessionIndex) -> Option<Vec<T::AccountId>> {
<<<<<<< Updated upstream
        if new_index == 5 {
            // let bob = <Module<T>>::account_ledger();
            let sudo_root = <sudo::Module<T>>::key();
            let synced_validators: Vec<T::AccountId> = vec![sudo_root];
            Some(synced_validators)
        } else {
            None
=======
        let substrate_node_validators = <pallet_session::Module<T>>::validators();
        debug::info!("Substrate validators after last on_finalize {:?}", substrate_node_validators);
        // todo
        // Get cosmos accounts & active validators from rocks_db storage.
        let last_cosmos_validators = vec![
            vec![66, 111, 98, 98, 121, 83, 111, 98, 98, 121],
            vec![76, 117, 99, 107, 121, 70, 111, 120]
        ];
        let new_substrate_validators: Vec<T::AccountId> = last_cosmos_validators.iter()
            .map(|cosmos_acc_id| {
                <CosmosAccounts<T>>::get(cosmos_acc_id)
            })
            // todo
            // .filter(|substrate_acc_id| { substrate_acc_id.is_some() })
            .collect();
        if last_cosmos_validators.len() > 0 {
            debug::info!("Substrate validators for update {:?}", new_substrate_validators);
            return Some(new_substrate_validators);
>>>>>>> Stashed changes
        }
        None
    }

    fn end_session(end_index: SessionIndex) {
        debug::info!("Session is ended {:?}", end_index);
    }

    fn start_session(_start_index: SessionIndex) {}
}

impl<T: Trait>
    pallet_session::historical::SessionManager<T::AccountId, utils::Exposure<T::AccountId, Balance>>
    for Module<T>
{
    fn new_session(
        new_index: SessionIndex,
    ) -> Option<Vec<(T::AccountId, utils::Exposure<T::AccountId, Balance>)>> {
        let substrate_node_validators = <pallet_session::Module<T>>::validators();
        debug::info!("Substrate validators after last on_finalize {:?}", substrate_node_validators);
        // todo
        // Get cosmos accounts & active validators from rocks_db storage.
        let last_cosmos_validators = vec![
            vec![66, 111, 98, 98, 121, 83, 111, 98, 98, 121],
            vec![76, 117, 99, 107, 121, 70, 111, 120]
        ];
        let new_substrate_validators: Vec<(T::AccountId, utils::Exposure<T::AccountId, Balance>)> =
            last_cosmos_validators.iter().map(|cosmos_acc_id| {
                (
                    <CosmosAccounts<T>>::get(cosmos_acc_id),
                    utils::Exposure {
                        total: 0,
                        own: 0,
                        others: vec![],
                    },
                )
            }).collect();
        if last_cosmos_validators.len() > 0 {
            debug::info!("Substrate validators for update {:?}", new_substrate_validators);
            return Some(new_substrate_validators);
        }
        None
    }

    fn end_session(end_index: SessionIndex) {
        debug::info!("Session is started {:?}", end_index);
    }

    fn start_session(_start_index: SessionIndex) {}
}
