//! The pallet for interact with cosmos abci interface.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_assignments)]
#![warn(unused_must_use)]
use frame_support::{
    codec::{Decode, Encode},
    debug, decl_module, decl_storage,
    dispatch::{DispatchResult, Vec},
    weights::Weight,
};
use frame_system::{
    self as system, ensure_none, ensure_signed, offchain::CreateSignedTransaction, RawOrigin,
};
use pallet_babe;
#[cfg(feature = "aura")]
use pallet_grandpa::fg_primitives;
use pallet_session as session;
#[cfg(feature = "babe")]
#[allow(unused_imports)]
use sp_consensus_babe;
use sp_core::{crypto::KeyTypeId, Hasher};
#[allow(unused_imports)]
use sp_runtime::{
    traits::{Convert, SaturatedConversion, Zero},
    transaction_validity::{
        InvalidTransaction, TransactionSource, TransactionValidity, ValidTransaction,
    },
    DispatchError, RuntimeDebug,
};
use sp_runtime_interface::runtime_interface;
use sp_std::{cmp::PartialEq, convert::TryInto, prelude::*, str};

/// Declare `crypto_transform` module.
pub mod crypto_transform;
/// Declare `utils` module.
pub mod utils;

/// Balance type for pallet.
pub type Balance = u64;
/// Session index that define in pallet_session.
type SessionIndex = u32;
/// The optional ledger type.
type OptionalLedger<AccountId> = Option<(AccountId, Balance)>;

pub const COSMOS_ACCOUNT_DEFAULT_PUB_KEY_TYPE: &str = "ed25519";
/// Priority for unsigned transaction.
pub const UNSIGNED_TXS_PRIORITY: u64 = 100;
pub const SESSION_BLOCKS_PERIOD: u32 = 2;

/// The KeyType ID.
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"abci");
/// Based on the above `KeyTypeId` we need to generate a pallet-specific crypto type wrapper.
/// We can utilize the supported crypto kinds (`sr25519`, `ed25519` and `ecdsa`) and augment
/// them with the pallet-specific identifier.
pub mod crypto {
    use crate::KEY_TYPE;
    use sp_runtime::app_crypto::{app_crypto, sr25519};

    app_crypto!(sr25519, KEY_TYPE);
}

/// The CosmosAbci trait.
pub trait CosmosAbci {
    fn check_tx(data: Vec<u8>) -> Result<u64, DispatchError>;
    fn deliver_tx(data: Vec<u8>) -> DispatchResult;
}

/// The pallet configuration trait.
pub trait Trait:
    CreateSignedTransaction<Call<Self>>
    + pallet_session::Trait
    + pallet_sudo::Trait
    + pallet_grandpa::Trait
    + pallet_babe::Trait
{
    type AuthorityId: Decode + sp_runtime::RuntimeAppPublic + Default;
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
tuple_impls! { A B C D E F G H I J K L M N O }
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
        CosmosAccounts get(fn cosmos_accounts): map hasher(blake2_128_concat) Vec<u8> => Option<T::AccountId> = None;
        AccountLedger get(fn account_ledgers): map hasher(blake2_128_concat) T::AccountId => OptionalLedger<T::AccountId>;
        SubstrateAccounts get(fn substrate_accounts): map hasher(blake2_128_concat) <T as session::Trait>::ValidatorId => Option<utils::CosmosAccount> = None;
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

        // Insert Cosmos node account.
        #[weight = 0]
        fn insert_cosmos_account(
            origin,
            cosmos_account_pub_key: Vec<u8>,
        ) -> DispatchResult {
            let origin_signed = ensure_signed(origin)?;
            <AccountLedger<T>>::insert(&origin_signed, Some((&origin_signed, 0)));
            let convertable = <T as pallet_session::Trait>::ValidatorIdOf::convert(origin_signed.clone())
                .unwrap();
            <CosmosAccounts<T>>::insert(&cosmos_account_pub_key, &origin_signed);
            <SubstrateAccounts<T>>::insert(&convertable, utils::CosmosAccount {
                pub_key: cosmos_account_pub_key,
                power: 0,
            });
            Ok(())
        }

        // Remove Cosmos node account.
        #[weight = 0]
        fn remove_cosmos_account(origin) -> DispatchResult {
            let convertable = <T as session::Trait>::ValidatorIdOf::convert(ensure_signed(origin)?)
                .unwrap();
            if let Some(cosmos_account) = <SubstrateAccounts<T>>::get(&convertable) {
                <CosmosAccounts<T>>::remove(&cosmos_account.pub_key);
            }
            <SubstrateAccounts<T>>::remove(&convertable);
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
            if let Some(bytes) = abci_interface::storage_get(b"abci_current_height".to_vec()).unwrap() {
                let mut height: u32 = u32::from_ne_bytes(bytes.as_slice().try_into().unwrap());
                while height != block_number.saturated_into() as u32 {
                    height += 1;
                    if height !=0 {
                        let block_hash = <system::Module<T>>::block_hash(T::BlockNumber::from(height));
                        let parent_hash = <system::Module<T>>::block_hash(T::BlockNumber::from(height - 1));
                        // TODO: fix it, calculate the original extrinsics_root of the block
                        let extrinsic_data = <system::Module<T>>::extrinsic_data(0);
                        let extrinsics_root = T::Hashing::hash(extrinsic_data.as_slice());

                        Self::call_offchain_worker(T::BlockNumber::from(height), block_hash, parent_hash, extrinsics_root);
                    }
                }
            }

            abci_interface::storage_write(b"abci_current_height".to_vec(),
            (block_number.saturated_into() as u32).to_ne_bytes().to_vec()).unwrap();
        }
    }
}

/// Implementation of additional methods for pallet configuration trait.
impl<T: Trait> Module<T> {
    fn get_corresponding_height(session_index: SessionIndex) -> u32 {
        if session_index > 2 {
            (session_index - 2) * SESSION_BLOCKS_PERIOD
        } else {
            0
        }
    }

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
        Self::call_on_initialize(block_number, block_hash, parent_hash, extrinsics_root);

        let abci_txs: ABCITxs = <ABCITxStorage<T>>::get(block_number);
        for abci_tx in abci_txs.data_array {
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
        let mut active_cosmos_validators = Vec::<utils::CosmosAccount>::new();

        for validator in <session::Module<T>>::validators() {
            if let Some(value) = <SubstrateAccounts<T>>::get(validator) {
                active_cosmos_validators.push(value);
            };
        }

        if let Err(err) = abci_interface::begin_block(
            block_number.saturated_into() as i64,
            block_hash.as_ref().to_vec(),
            parent_hash.as_ref().to_vec(),
            extrinsics_root.as_ref().to_vec(),
            active_cosmos_validators,
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

    pub fn update_keys_for_account(
        validator_id: T::AccountId,
        keys: T::Keys,
        proof: Vec<u8>,
    ) -> DispatchResult {
        let _response =
            <session::Module<T>>::set_keys(RawOrigin::Signed(validator_id).into(), keys, proof);
        Ok(())
    }

    pub fn assign_weights(changed: bool) {
        let mut authorities_with_updated_weight: fg_primitives::AuthorityList = Vec::new();
        let validators = <session::Module<T>>::validators();
        for validator in validators {
            if let Some(value) = <SubstrateAccounts<T>>::get(validator) {
                let mut substrate_account_id: &[u8] =
                    &<CosmosAccounts<T>>::get(value.pub_key).encode();
                match sp_finality_grandpa::AuthorityId::decode(&mut substrate_account_id) {
                    Ok(authority_id_value) => {
                        weighted_aura_authorities.push((authority_id_value, value.power as u64));
                    }
                    Err(_) => {}
                }
            };
        }

        if let Some((further_wait, median)) = <pallet_grandpa::Module<T>>::stalled() {
            <pallet_grandpa::Module<T>>::schedule_change(
                authorities_with_updated_weight,
                further_wait,
                Some(median),
            )
            .unwrap();
        } else if changed {
            <pallet_grandpa::Module<T>>::schedule_change(
                authorities_with_updated_weight,
                Zero::zero(),
                None,
            )
            .unwrap();
        }
    }

    pub fn on_new_session(new_session_index: SessionIndex) -> Option<Vec<T::ValidatorId>> {
        // Sessions starts after end_block() with number 2.
        // For some reason two first sessions is skipped.

        let current_substarte_validators = <session::Module<T>>::validators();

        let corresponding_height = Self::get_corresponding_height(new_session_index);
        let next_cosmos_validators =
            abci_interface::get_cosmos_validators(corresponding_height.into()).unwrap();

        if !next_cosmos_validators.is_empty() {
            let mut new_substrate_validators: Vec<T::ValidatorId> = vec![];
            for cosmos_validator in &next_cosmos_validators {
                if let Some(substrate_account_id) =
                    <CosmosAccounts<T>>::get(&cosmos_validator.pub_key)
                {
                    // update cosmos validator in the substrate storage
                    let convertable =
                        <T as pallet_session::Trait>::ValidatorIdOf::convert(substrate_account_id)
                            .unwrap();
                    new_substrate_validators.push(convertable.clone());
                    <SubstrateAccounts<T>>::insert(convertable, cosmos_validator);
                } else {
                    sp_runtime::print(
                        "WARNING: Not able to found Substrate account to Cosmos for ID : ",
                    );
                    sp_runtime::print(&*hex::encode(&cosmos_validator.pub_key));
                }
            }

            if !new_substrate_validators.is_empty()
                && current_substarte_validators != new_substrate_validators
            {
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

    fn get_cosmos_validators(height: i64) -> Result<Vec<utils::CosmosAccount>, DispatchError> {
        match abci_storage::get_abci_storage_instance()
            .map_err(|_| "failed to get abci storage instance")?
            .get(height.to_ne_bytes().to_vec())
            .map_err(|_| "failed to get value from the abci storage")?
        {
            Some(bytes) => {
                let validators = pallet_abci::utils::deserialize_vec::<
                    pallet_abci::protos::ValidatorUpdate,
                >(&bytes)
                .map_err(|_| "cannot deserialize ValidatorUpdate vector")?;

                let mut response = Vec::new();
                for val in validators {
                    if let Some(key) = val.pub_key {
                        response.push(utils::CosmosAccount {
                            pub_key: key.data,
                            power: val.power,
                        });
                    }
                }
                Ok(response)
            }
            None => Ok(Vec::new()),
        }
    }

    fn check_tx(data: Vec<u8>) -> Result<u64, DispatchError> {
        let result = pallet_abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")?
            .check_tx(data)
            .map_err(|_| "check_tx failed")?;

        if result.get_code() != 0 {
            Err(DispatchError::Module {
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
        current_cosmos_validators: Vec<utils::CosmosAccount>,
    ) -> DispatchResult {
        let cosmos_validators: Vec<pallet_abci::protos::VoteInfo> = current_cosmos_validators
            .iter()
            .map(|validator| {
                let address = crypto_transform::get_address_from_pub_key(
                    &validator.pub_key,
                    crypto_transform::PubKeyTypes::Ed25519,
                );

                pallet_abci::protos::VoteInfo {
                    validator: Some(pallet_abci::protos::Validator {
                        address,
                        power: validator.power,
                    }),
                    // TODO Check if validator is author of last block or does not.
                    signed_last_block: true,
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
                cosmos_validators,
            )
            .map_err(|_| "begin_block failed")?;

        Ok(())
    }

    fn end_block(height: i64) -> DispatchResult {
        let result = pallet_abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")?
            .end_block(height)
            .map_err(|_| "end_block failed")?;
        let cosmos_validators_updates = result.get_validator_updates();

        let mut current_cosmos_validators = vec![];

        // take validators from the previous block
        if let Some(previous_validators_bytes) = abci_storage::get_abci_storage_instance()
            .map_err(|_| "failed to get abci storage instance")?
            .get((height - 1).to_ne_bytes().to_vec())
            .map_err(|_| "failed to write some data into the abci storage")?
        {
            current_cosmos_validators =
                pallet_abci::utils::deserialize_vec(&previous_validators_bytes)
                    .map_err(|_| "cannot deserialize cosmos validators")?;
        }

        for validator_update in cosmos_validators_updates {
            if validator_update.power == 0 {
                // remove this validator for the current list
                current_cosmos_validators.retain(
                    |x: &pallet_abci::protos::ValidatorUpdate| -> bool {
                        x.pub_key != validator_update.pub_key
                    },
                );
            }
            current_cosmos_validators.push(validator_update);
        }

        let bytes = pallet_abci::utils::serialize_vec(current_cosmos_validators)
            .map_err(|_| "cannot serialize cosmos validators")?;

        // save it in the storage
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

impl<T: Trait> pallet_session::SessionManager<T::ValidatorId> for Module<T> {
    fn new_session(new_index: SessionIndex) -> Option<Vec<T::ValidatorId>> {
        Self::on_new_session(new_index)
    }

    fn end_session(_end_index: SessionIndex) {}

    fn start_session(_start_index: SessionIndex) {}
}

impl<T: Trait> pallet_session::ShouldEndSession<T::BlockNumber> for Module<T> {
    fn should_end_session(_: T::BlockNumber) -> bool {
        true
    }
}

impl<T: Trait> pallet_session::OneSessionHandler<T::AccountId> for Module<T>
where
    <T as Trait>::AuthorityId: sp_runtime::RuntimeAppPublic,
{
    type Key = T::AuthorityId;

    fn on_new_session<'a, I: 'a>(_changed: bool, _validators: I, _queued_validators: I)
    where
        I: Iterator<Item = (&'a T::AccountId, Self::Key)>,
    {
        // Self::assign_weights(changed);
    }

    fn on_genesis_session<'a, I: 'a>(_validators: I)
    where
        I: Iterator<Item = (&'a T::AccountId, Self::Key)>,
    {
    }

    fn on_disabled(_i: usize) {}
}
