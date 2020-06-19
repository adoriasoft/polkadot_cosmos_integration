#![cfg_attr(not(feature = "std"), no_std)]

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
use sp_std::prelude::*;
use sp_runtime::traits::SaturatedConversion;

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
        Requests get(fn requests): Vec<u32>;
        Results get(fn results): Vec<u32>;
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
        pub fn deliver_tx(origin, id: u32) -> DispatchResult {
            ensure_signed(origin)?;
            debug::info!("Received deliver tx request #{}", id);
            <Requests>::mutate(|x| x.push(id));
            Ok(())
        }

        #[weight = 0]
        pub fn finish_deliver_tx(origin, results: Vec<u32>) -> DispatchResult {
            ensure_signed(origin)?;
            debug::native::info!("Finish deliver tx: {:?}", results);
            <Requests>::mutate(|x| *x = vec![]);
            <Results>::mutate(|x| x.extend(results));
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

    pub fn do_check_tx(_source: TransactionSource, message: &u32) {
        debug::native::info!("Validate from pallet: {:?}", message);
    }

    pub fn offchain_logic(now: T::BlockNumber) -> Result<Vec<Result<T::AccountId, ()>>, &'static str> {
        // Test values
        let blk_msg = abci_grpc::BlockMessage { height: now.saturated_into::<u64>() };
        let tx_msg = abci_grpc::TxMessage { tx: vec![33, 33, 33, 33] };

        abci_grpc::init_chain()?;

        abci_grpc::on_initialize(&blk_msg)?;

        abci_grpc::check_tx(&tx_msg)?;
        abci_grpc::deliver_tx(&tx_msg)?;

        abci_grpc::on_finilize(&blk_msg)?;
        abci_grpc::commit(&blk_msg)?;

        abci_grpc::echo()?;

        Self::submit_result(vec![1,2,3])
    }

    pub fn submit_result(res: Vec<u32>) -> Result<Vec<Result<T::AccountId, ()>>, &'static str> {
        let signer = Signer::<T, T::AuthorityId>::all_accounts();
        if !signer.can_sign() {
            return Err(
                "No local accounts available. Consider adding one via `author_insertKey` RPC.",
            )?;
        }
        let results = signer.send_signed_transaction(|_| Call::finish_deliver_tx(res.clone()));
        Ok(results
            .iter()
            .map(|(acc, res)| match res {
                Ok(_) => Ok(acc.id.clone()),
                Err(_) => Err(()),
            })
            .collect())
    }
}
