#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
mod std_logic;
mod abci_grpc;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use frame_support::{
    debug, decl_module, decl_storage, dispatch::DispatchResult, dispatch::Vec,
    sp_runtime::transaction_validity::TransactionSource, weights::Weight,
};
use frame_system::{
    ensure_signed,
    offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer},
};
use sp_runtime::traits::SaturatedConversion;
use sp_runtime_interface::runtime_interface;
// use sc_executor::{native_executor_instance, NativeExecutor};
use sp_std::prelude::*;

pub mod crypto {
    use sp_core::crypto::KeyTypeId;
    use sp_runtime::{
        app_crypto::{app_crypto, sr25519},
        traits::Verify,
        MultiSignature,
    };
    pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"abci");
    app_crypto!(sr25519, KEY_TYPE);

    pub struct AuthId;
    impl frame_system::offchain::AppCrypto<<MultiSignature as Verify>::Signer, MultiSignature>
        for AuthId
    {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }
}

/// The pallet's configuration trait.
pub trait Trait: CreateSignedTransaction<Call<Self>> {
    /// The identifier type for an offchain worker.
    type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
    /// The overarching dispatch call type.
    type Call: From<Call<Self>>;
}

decl_storage! {
    trait Store for Module<T: Trait> as AbciModule {
        Requests get(fn requests): Vec<abci_grpc::TxMessage>;
    }
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Block initialization
        fn on_initialize(_now: T::BlockNumber) -> Weight {
            Self::do_initialize(_now);
            return 0;
        }

        /// Block finalization
        fn on_finalize() {
            let tmp = my_interface::get_hello_world();
            debug::native::info!("Hello: {:?}", tmp);
            Self::do_finalize();
        }

        fn offchain_worker(now: T::BlockNumber) {
            debug::native::info!("Hello from offchain workers!");
            match Self::offchain_logic(now) {
                Ok(results) => {
                    debug::native::info!("Results: {:?}", results.len());
                    for val in &results {
                        match val {
                            Ok(acc) => debug::info!("Submitted transaction from: {:?}", acc),
                            Err(e) => debug::error!("Failed to submit transaction: {:?}", e),
                        }
                    }
                }
                Err(e) => {
                    debug::error!("Error: {}", e);
                }
            }
        }

        #[weight = 0]
        pub fn deliver_tx(origin, tx: abci_grpc::TxMessage) -> DispatchResult {
            ensure_signed(origin)?;
            debug::info!("Received deliver tx request");
            <Requests>::mutate(|x| x.push(tx));
            Ok(())
        }

        #[weight = 0]
        pub fn finish_deliver_tx(origin, results: Vec<abci_grpc::TxMessage>) -> DispatchResult {
            ensure_signed(origin)?;
            debug::native::info!("Finish deliver tx: {:?}", results);
            <Requests>::mutate(|x| x.retain(|r| {
                results.iter().position(|res| res == r).is_none()
            }));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn do_finalize() {
        debug::native::info!("Block is finilized");
    }

    pub fn do_initialize(block_number: T::BlockNumber) {
        debug::native::info!("Block is initialized: {:?}", block_number);
    }

    pub fn do_commit() {
        debug::native::info!("Block is commited");
    }

    pub fn do_check_tx(_source: TransactionSource) {
        debug::native::info!("Validate from pallet");
    }

    pub fn offchain_logic(
        now: T::BlockNumber,
    ) -> Result<Vec<Result<T::AccountId, ()>>, &'static str> {
        let blk_msg = abci_grpc::BlockMessage {
            height: now.saturated_into::<u64>(),
        };

        abci_grpc::on_initialize(&blk_msg)?;

        let requests = Self::requests();
        for tx_msg in &requests {
            abci_grpc::check_tx(tx_msg)?;
            abci_grpc::deliver_tx(tx_msg)?;
        }

        abci_grpc::on_finilize(&blk_msg)?;
        abci_grpc::commit(&blk_msg)?;

        if requests.len() > 0 {
            Self::submit_result(requests)
        } else {
            Ok(vec![])
        }
    }

    pub fn submit_result(
        result: Vec<abci_grpc::TxMessage>,
    ) -> Result<Vec<Result<T::AccountId, ()>>, &'static str> {
        let signer = Signer::<T, T::AuthorityId>::all_accounts();
        if !signer.can_sign() {
            return Err(
                "No local accounts available. Consider adding one via `author_insertKey` RPC.",
            )?;
        }
        let results = signer.send_signed_transaction(|_| Call::finish_deliver_tx(result.clone()));
        Ok(results
            .iter()
            .map(|(acc, res)| match res {
                Ok(_) => Ok(acc.id.clone()),
                Err(_) => Err(()),
            })
            .collect())
    }
}

#[runtime_interface]
trait MyInterface {
    fn get_hello_world() -> Vec<u8> {
        // println!("Hello world from: {}", data);
        crate::std_logic::get_something()
    }
}

// native_executor_instance!(
//     pub MyExecutor,
//     substrate_test_runtime::api::dispatch,
//     substrate_test_runtime::native_version,
//     (my_interface::HostFunctions, my_interface::HostFunctions),
// );
