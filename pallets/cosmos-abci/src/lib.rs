#![cfg_attr(not(feature = "std"), no_std)]
#[warn(unused_must_use)]
use frame_support::{
    codec::{Decode, Encode},
    debug, decl_module, decl_storage,
    dispatch::{DispatchResult, Vec},
    weights::Weight,
};
use frame_system::{
    self as system, ensure_none,
    offchain::{AppCrypto, CreateSignedTransaction},
};
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
    traits::SaturatedConversion,
    transaction_validity::{
        InvalidTransaction, TransactionSource, TransactionValidity, ValidTransaction,
    },
    DispatchError, RuntimeDebug,
};
use sp_runtime_interface::runtime_interface;
use sp_std::prelude::*;

/// The type to sign and send transactions.
pub const UNSIGNED_TXS_PRIORITY: u64 = 100;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"abci");
/// Based on the above `KeyTypeId` we need to generate a pallet-specific crypto type wrapper.
/// We can utilize the supported crypto kinds (`sr25519`, `ed25519` and `ecdsa`) and augment
/// them with the pallet-specific identifier.
pub mod crypto {
    use crate::KEY_TYPE;
    use sp_core::sr25519::Signature as Sr25519Signature;
    use sp_runtime::app_crypto::{app_crypto, sr25519};
    use sp_runtime::{traits::Verify, MultiSignature, MultiSigner};

    app_crypto!(sr25519, KEY_TYPE);

    pub struct ABCIAuthId;
    // implemented for ocw-runtime
    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for ABCIAuthId {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }

    // implemented for mock runtime in test
    impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
        for ABCIAuthId
    {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }
}

pub trait CosmosAbci {
    fn check_tx(data: Vec<u8>) -> Result<u64, DispatchError>;
    fn deliver_tx(data: Vec<u8>) -> DispatchResult;
    fn query(path: &str, data: Vec<u8>, height: i64, prove: bool) -> DispatchResult;
}

/// The pallet's configuration trait.
pub trait Trait: CreateSignedTransaction<Call<Self>> {
    /// The identifier type for an offchain worker.
    type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
    /// The overarching dispatch call type.
    type Call: From<Call<Self>>;
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct ABCITxs {
    data_array: Vec<Vec<u8>>,
}

decl_storage! {
    trait Store for Module<T: Trait> as ABCITxStorage {
        ABCITxStorage get(fn abci_tx): map hasher(blake2_128_concat) T::BlockNumber => ABCITxs
    }
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Block initialization
        fn on_initialize(block_number: T::BlockNumber) -> Weight {
            0
        }

        /// Block finalization
        fn on_finalize(block_number: T::BlockNumber) {
        }

        #[weight = 0]
        pub fn abci_transaction(origin, data: Vec<u8>) -> DispatchResult {
            let _ = ensure_none(origin)?;

            Self::call_abci_transaction(data)?;
            Ok(())
        }

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

impl<T: Trait> Module<T> {
    pub fn call_abci_transaction(data: Vec<u8>) -> DispatchResult {
        let block_number = <system::Module<T>>::block_number();
        let mut abci_txs: ABCITxs = <ABCITxStorage<T>>::get(block_number);
        abci_txs.data_array.push(data.clone());
        <ABCITxStorage<T>>::insert(block_number, abci_txs);
        Ok(())
    }

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
            <Self as CosmosAbci>::deliver_tx(abci_tx)
                .map_err(|e| debug::error!("deliver_tx() error: {:?}", e));
        }

        Self::call_on_finalize(block_number);
    }

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
            // We have to panic, as if cosmos will not have some blocks - it will fail.
            panic!("Begin block failed: {:?}", err);
        }
        return true;
    }

    pub fn call_on_finalize(block_number: T::BlockNumber) -> bool {
        match abci_interface::end_block(block_number.saturated_into() as i64) {
            Ok(_) => {
                match abci_interface::commit() {
                    Err(err) => {
                        // We have to panic, as if cosmos will not have some blocks - it will fail.
                        panic!("Commit failed: {:?}", err);
                    }
                    _ => true,
                }
            }
            Err(err) => {
                // We have to panic, as if cosmos will not have some blocks - it will fail.
                panic!("End block failed: {:?}", err);
            }
        }
    }
}

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

impl<T: Trait> CosmosAbci for Module<T> {
    fn check_tx(data: Vec<u8>) -> Result<u64, DispatchError> {
        abci_interface::check_tx(data)
    }

    fn deliver_tx(data: Vec<u8>) -> DispatchResult {
        abci_interface::deliver_tx(data)
    }

    fn query(path: &str, data: Vec<u8>, height: i64, prove: bool) -> DispatchResult {
        abci_interface::query(path, data, height, prove)
    }
}

sp_api::decl_runtime_apis! {
    pub trait ExtrinsicConstructionApi {
        fn broadcast_abci_tx(data: &Vec<u8>);
    }
}

#[runtime_interface]
pub trait AbciInterface {
    fn echo(msg: &str) -> DispatchResult {
        let _result = abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")?
            .echo(msg.to_owned())
            .map_err(|_| "echo failed")?;
        // debug::info!("Result: {:?}", result);
        Ok(())
    }

    fn check_tx(data: Vec<u8>) -> Result<u64, DispatchError> {
        let result = abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")?
            .check_tx(data, 0)
            .map_err(|_| "check_tx failed")?;

        // TODO remove it after fix
        let dif = result.get_gas_wanted() - result.get_gas_used();
        Ok(dif as u64)

        // TODO uncomment after fix
        // if result.get_code() != 0 {
        //     Err(sp_runtime::DispatchError::Module {
        //         index: u8::MIN,
        //         error: result.get_code() as u8,
        //         message: Some("Invalid tx data."),
        //     })
        // } else {
        //     let dif = result.get_gas_wanted() - result.get_gas_used();
        //     Ok(dif as u64)
        // }
    }

    fn deliver_tx(data: Vec<u8>) -> DispatchResult {
        let _result = abci::get_abci_instance()
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
        let _result = abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")?
            .begin_block(height, hash, last_block_id, proposer_address)
            .map_err(|_| "begin_block failed")?;
        Ok(())
    }

    fn end_block(height: i64) -> DispatchResult {
        let _result = abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")?
            .end_block(height)
            .map_err(|_| "end_block failed")?;
        // debug::info!("Result: {:?}", result);
        Ok(())
    }

    fn commit() -> DispatchResult {
        let _result = abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")?
            .commit()
            .map_err(|_| "commit failed")?;
        // debug::info!("Result: {:?}", result);
        Ok(())
    }

    fn query(path: &str, data: Vec<u8>, height: i64, prove: bool) -> DispatchResult {
        let _result = abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")?
            .query(path.to_owned(), data, height, prove)
            .map_err(|_| "query failed")?;
        // debug::info!("Result: {:?}", result);
        Ok(())
    }
}
