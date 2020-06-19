#![cfg_attr(not(feature = "std"), no_std)]

mod abci_grpc;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use frame_support::{
    debug, decl_module, decl_storage, dispatch::DispatchResult, dispatch::Vec, sp_runtime::print,
    sp_runtime::transaction_validity::TransactionSource, weights::Weight,
};
use frame_system::{
    ensure_signed,
    offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer},
};
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

        fn offchain_worker(_now: T::BlockNumber) {
            debug::native::info!("Hello from offchain workers!");

            // Test values
            let blk_msg = abci_grpc::BlockMessage { height: 123 };
            let tx_msg = abci_grpc::TxMessage { tx: vec![33, 33, 33, 33] };

            abci_grpc::init_chain();

            abci_grpc::on_initialize(&blk_msg);

            abci_grpc::check_tx(&tx_msg);
            abci_grpc::deliver_tx(&tx_msg);

            abci_grpc::on_finilize(&blk_msg);
            abci_grpc::commit(&blk_msg);

            abci_grpc::echo();
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
}
