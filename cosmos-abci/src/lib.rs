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
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
    traits::{Convert, SaturatedConversion},
    transaction_validity::{
        InvalidTransaction, TransactionSource, TransactionValidity, ValidTransaction,
    },
    DispatchError, RuntimeDebug,
};
use sp_runtime_interface::runtime_interface;
use sp_std::{prelude::*, str};

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

/// Cosmos ABCI pallet utils.
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
    fn on_check_tx(_: Vec<u8>) -> DispatchResult {
        Ok(())
    }
    fn on_deliver_tx(_: Vec<u8>) -> DispatchResult {
        Ok(())
    }
}

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: SubscriptionManager),+> SubscriptionManager for ($($name,)+)
        {
            fn on_check_tx(data: Vec<u8>) -> DispatchResult {
                $($name::on_check_tx(data.clone())?;)+
                Ok(())
            }

            fn on_deliver_tx(data: Vec<u8>) -> DispatchResult {
                $($name::on_deliver_tx(data.clone())?;)+
                Ok(())
            }
        }
    };
}

tuple_impls! { A }
tuple_impls! { A B }
tuple_impls! { A B C }
tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
tuple_impls! { A B C D E F G H }
tuple_impls! { A B C D E F G H I }
tuple_impls! { A B C D E F G H I J }
tuple_impls! { A B C D E F G H I J K }
tuple_impls! { A B C D E F G H I J K L }
tuple_impls! { A B C D E F G H I J K L M }
tuple_impls! { A B C D E F G H I J K L M N }
tuple_impls! { A B C D E F G H I J K L M N O}
tuple_impls! { A B C D E F G H I J K L M N O P }

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
        CosmosAccounts get(fn cosmos_accounts): map hasher(blake2_128_concat) utils::CosmosAccountId => Option<T::AccountId> = None;
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
        fn insert_cosmos_account(origin, cosmos_account_id: Vec<u8>) -> DispatchResult {
            let origin_signed = ensure_signed(origin)?;
            <AccountLedger<T>>::insert(&origin_signed, Some((&origin_signed, 0)));
            <CosmosAccounts<T>>::insert(&cosmos_account_id, &origin_signed);
            Ok(())
        }

        // Remove Cosmos node account.
        #[weight = 0]
        fn remove_cosmos_account(origin, cosmos_account_id: Vec<u8>) -> DispatchResult {
            let _origin_signed = ensure_signed(origin)?;
            <CosmosAccounts<T>>::remove(&cosmos_account_id);
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
            Ok(_) => match abci_interface::commit() {
                Err(err) => {
                    panic!("Commit failed: {:?}", err);
                }
                _ => true,
            },
            Err(err) => {
                panic!("End block failed: {:?}", err);
            }
        }
    }

    pub fn update_keys_for_account(validator_id: T::AccountId, keys: T::Keys) {
        let proof = vec![];
        let _response =
            <session::Module<T>>::set_keys(RawOrigin::Signed(validator_id).into(), keys, proof);
    }

    pub fn on_new_session(new_index: SessionIndex) -> Option<Vec<T::AccountId>> {
        let next_cosmos_validators =
            abci_interface::get_cosmos_validators_from_storage(new_index.into()).unwrap();

        if !next_cosmos_validators.is_empty() {
            let mut new_substrate_validators: Vec<T::AccountId> = vec![];
            for cosmos_validator_id in &next_cosmos_validators {
                let substrate_account_id = <CosmosAccounts<T>>::get(cosmos_validator_id);
                if substrate_account_id.is_some() {
                    if let Some(full_substrate_account_id) = substrate_account_id {
                        new_substrate_validators.push(full_substrate_account_id);
                    } else {
                        sp_runtime::print(
                            "WARNING: Not able to found Substrate account to Cosmos for ID \n",
                        );
                        sp_runtime::print(str::from_utf8(cosmos_validator_id).unwrap());
                    }
                }
            }
            if !new_substrate_validators.is_empty() {
                debug::info!(
                    "Substrate validators on_new_session() {:?}",
                    new_substrate_validators
                );
                return Some(new_substrate_validators);
            }
        }
        None
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
        <T::Subscription as SubscriptionManager>::on_check_tx(data.clone())?;
        abci_interface::check_tx(data)
    }

    fn deliver_tx(data: Vec<u8>) -> DispatchResult {
        <T::Subscription as SubscriptionManager>::on_deliver_tx(data.clone())?;
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
    fn storage_write(key: Vec<u8>, value: Vec<u8>) -> Result<(), DispatchError> {
        abci_storage::get_abci_storage_instance()
            .map_err(|_| "failed to get abci storage instance")?
            .write(key, value)
            .map_err(|_| "failed to write some data into the abci storage")?;
        Ok(())
    }

    fn storage_get(key: Vec<u8>) -> Result<Option<Vec<u8>>, DispatchError> {
        let value = abci_storage::get_abci_storage_instance()
            .map_err(|_| "failed to get abci storage instance")?
            .get(key)
            .map_err(|_| "failed to get value from the abci storage")?;

        Ok(value)
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

    fn get_cosmos_validators_from_storage(height: i64) -> Result<Vec<Vec<u8>>, DispatchError> {
        match storage_get(height.to_ne_bytes().to_vec()) {
            Ok(validators_response) => match validators_response {
                Some(bytes) => {
                    let validators = pallet_abci::utils::deserialize_vec::<
                        pallet_abci::utils::SerializableValidatorUpdate,
                    >(&bytes)
                    .map_err(|_| "cannot deserialize ValidatorUpdate vector")?;
                    let mut res = Vec::new();
                    for val in validators {
                        res.push(val.key_data);
                    }
                    Ok(res)
                }
                None => Ok(Vec::new()),
            },
            Err(_err) => Ok(Vec::new()),
        }
    }

    fn begin_block(
        height: i64,
        hash: Vec<u8>,
        last_block_id: Vec<u8>,
        proposer_address: Vec<u8>,
    ) -> DispatchResult {
        let last_cosmos_validators =
            abci_interface::get_cosmos_validators_from_storage(height).unwrap();
        debug::info!(
            "Validators on begin_block(): {:?} with height {}",
            last_cosmos_validators,
            height
        );

        let byzantine_validators: Vec<pallet_abci::protos::Evidence> = last_cosmos_validators
            .iter()
            .map(|validator| {
                let address = pallet_abci::utils::get_validator_address(validator.clone()).unwrap();
                // TODO Do we need specify `type` and `power` from origin?
                pallet_abci::protos::Evidence {
                    r#type: "ed25519".to_owned(),
                    validator: Some(pallet_abci::protos::Validator {
                        power: 100,
                        address,
                    }),
                    height,
                    time: None,
                    total_voting_power: 100,
                }
            })
            .collect();
        let _result = pallet_abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")?
            .begin_block(
                height,
                hash,
                last_block_id,
                proposer_address,
                byzantine_validators,
            )
            .map_err(|_| "begin_block failed")?;

        Ok(())
    }

    fn end_block(height: i64) -> DispatchResult {
        let result = pallet_abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")?
            .end_block(height)
            .map_err(|_| "end_block failed")?;
        let bytes = pallet_abci::utils::serialize_vec(
            result
                .get_validator_updates()
                .iter()
                .map(|validator| {
                    let mut pub_key = vec![];
                    match &validator.pub_key {
                        Some(key) => pub_key = key.data.clone(),
                        None => {}
                    }
                    pallet_abci::utils::SerializableValidatorUpdate {
                        key_data: pub_key,
                        r#type: "ed25519".to_owned(),
                        power: validator.power,
                    }
                })
                .collect(),
        )
        .map_err(|_| "cannot serialize cosmos validators")?;

        abci_storage::get_abci_storage_instance()
            .map_err(|_| "failed to get abci storage instance")?
            .write(height.to_ne_bytes().to_vec(), bytes)
            .map_err(|_| "failed to write some data into the abci storage")?;

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
        let account_ledger: OptionalLedger<T::AccountId> =
            <Module<T>>::account_ledgers(&controller);
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
        Self::on_new_session(new_index)
    }

    fn end_session(_end_index: SessionIndex) {}

    fn start_session(_start_index: SessionIndex) {}
}

impl<T: Trait>
    pallet_session::historical::SessionManager<T::AccountId, utils::Exposure<T::AccountId, Balance>>
    for Module<T>
{
    fn new_session(
        new_index: SessionIndex,
    ) -> Option<Vec<(T::AccountId, utils::Exposure<T::AccountId, Balance>)>> {
        let new_substrate_validators = Self::on_new_session(new_index);
        if let Some(validators) = new_substrate_validators {
            return Some(
                validators
                    .iter()
                    .map(|validator| {
                        (
                            validator.clone(),
                            utils::Exposure {
                                total: 0,
                                own: 0,
                                others: vec![],
                            },
                        )
                    })
                    .collect(),
            );
        }
        None
    }

    fn end_session(_end_index: SessionIndex) {}

    fn start_session(_start_index: SessionIndex) {}
}
