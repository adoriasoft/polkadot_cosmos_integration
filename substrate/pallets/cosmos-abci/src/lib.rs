#![cfg_attr(not(feature = "std"), no_std)]
#[warn(unused_must_use)]

use frame_support::{debug, decl_module, dispatch::DispatchResult, dispatch::Vec, weights::Weight};
use frame_system::{
    ensure_signed,
    offchain::{AppCrypto, CreateSignedTransaction},
};
use sp_core::crypto::KeyTypeId;
use sp_runtime::{traits::SaturatedConversion, DispatchError};
use sp_runtime_interface::runtime_interface;
use sp_std::prelude::*;

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

impl <T: Trait> Module<T> {

    pub fn call_on_initialize(block_number: T::BlockNumber) -> u64 {
        let value: i64 = abci_interface::get_on_initialize_variable();
        let block_number_current: i64 = block_number.saturated_into() as i64;
        debug::info!("on_initialize() processing, block number: {:?}", block_number_current);
        debug::info!("on_initialize() value: {:?}", value);
    
        if value > block_number_current {
            return 0;
        }

        match abci_interface::begin_block(
            block_number_current,
            vec![],
            vec![],
        ) {
            Err(err) => {
                // We have to panic, as if cosmos will not have some blocks - it will fail.
                panic!("Begin block failed: {:?}", err);
            },
            _ => {},
        }

        abci_interface::increment_on_initialize_variable();
        return 0;
    }

    pub fn call_on_finalize(block_number: T::BlockNumber) -> bool {
        debug::info!("on_finalize() processing, block number: {:?}", block_number);
        let block_number_current: i64 = block_number.saturated_into() as i64;
        match abci_interface::end_block(block_number_current) {
            Ok(_) => {
                match abci_interface::commit() {
                    Err(err) => {
                        // We have to panic, as if cosmos will not have some blocks - it will fail.
                        panic!("Commit failed: {:?}", err);
                    },
                    _ => {
                        true
                    },
                }
            },
            Err(err) => {
                // We have to panic, as if cosmos will not have some blocks - it will fail.
                panic!("End block failed: {:?}", err);
            },
        }
    }
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Block initialization
        fn on_initialize(now: T::BlockNumber) -> Weight {
            Self::call_on_initialize(now)
        }

        /// Block finalization
        fn on_finalize(now: T::BlockNumber) {
           Self::call_on_finalize(now);
        }

        #[weight = 0]
        pub fn deliver_tx(origin, data: Vec<u8>) -> DispatchResult {
            ensure_signed(origin)?;
            debug::info!("Received deliver tx request");
            <Self as CosmosAbci>::deliver_tx(data)?;
            Ok(())
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
        fn sign_and_send_deliver_tx(data: &Vec<u8>);
    }
}

#[runtime_interface]
pub trait AbciInterface {
    fn get_on_initialize_variable() -> i64 {
        abci::get_on_initialize_variable()
    }

    fn increment_on_initialize_variable() {
        abci::increment_on_initialize_variable();
    }

    fn echo(msg: &str) -> DispatchResult {
        let _result = abci::connect_or_get_connection(&abci::get_server_url())
            .map_err(|_| "failed to setup connection")?
            .echo(msg.to_owned())
            .map_err(|_| "echo failed")?;
        // debug::info!("Result: {:?}", result);
        Ok(())
    }

    fn check_tx(data: Vec<u8>) -> Result<u64, DispatchError> {
        let result = abci::connect_or_get_connection(&abci::get_server_url())
            .map_err(|_| "failed to setup connection")?
            .check_tx(data, 0)
            .map_err(|_| "check_tx failed")?;
        // debug::info!("Result: {:?}", result);
        // If GasWanted is greater than GasUsed, we will increase the priority
        // Todo: Make it more logical
        let dif = result.gas_wanted - result.gas_used;
        Ok(dif as u64)
    }

    fn deliver_tx(data: Vec<u8>) -> DispatchResult {
        let _result = abci::connect_or_get_connection(&abci::get_server_url())
            .map_err(|_| "failed to setup connection")?
            .deliver_tx(data)
            .map_err(|_| "deliver_tx failed")?;
        // debug::info!("Result: {:?}", result);
        Ok(())
    }

    fn begin_block(height: i64, hash: Vec<u8>, proposer_address: Vec<u8>) -> DispatchResult {
        let _result = abci::connect_or_get_connection(&abci::get_server_url())
            .map_err(|_| "failed to setup connection")?
            .begin_block(height, hash, proposer_address)
            .map_err(|_| "begin_block failed")?;
        // debug::info!("Result: {:?}", result);
        Ok(())
    }

    fn end_block(height: i64) -> DispatchResult {
        let _result = abci::connect_or_get_connection(&abci::get_server_url())
            .map_err(|_| "failed to setup connection")?
            .end_block(height)
            .map_err(|_| "end_block failed")?;
        // debug::info!("Result: {:?}", result);
        Ok(())
    }

    fn commit() -> DispatchResult {
        let _result = abci::connect_or_get_connection(&abci::get_server_url())
            .map_err(|_| "failed to setup connection")?
            .commit()
            .map_err(|_| "commit failed")?;
        // debug::info!("Result: {:?}", result);
        Ok(())
    }

    fn query(path: &str, data: Vec<u8>, height: i64, prove: bool) -> DispatchResult {
        let _result = abci::connect_or_get_connection(&abci::get_server_url())
            .map_err(|_| "failed to setup connection")?
            .query(path.to_owned(), data, height, prove)
            .map_err(|_| "query failed")?;
        // debug::info!("Result: {:?}", result);
        Ok(())
    }
}
