#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use frame_support::{
    debug, decl_module, dispatch::DispatchResult, dispatch::Vec, sp_runtime::print,
    sp_runtime::transaction_validity::TransactionSource, weights::Weight,
};
use frame_system::offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer};

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
            let res = Self::make_request();
            match res {
                Ok(results) => {
                    debug::native::info!("Results: {:?}", results.len());
                    for val in &results {
                        match val {
                            Ok(acc) => debug::info!("Submitted transaction: {:?}", acc),
                            Err(e) => debug::error!("Failed to submit transaction: {:?}", e),
                        }
                    }
                }
                Err(e) => {
                    debug::error!("Error: {}", e);
                }
            }
        }

        //Simple unparametrized function, may be useful for test calls to the pallet
        #[weight = 0]
        pub fn some_function(_origin) {
            debug::native::info!("Some function logic");
        }

        /// Transaction execution
        #[weight = 0]
        pub fn deliver_tx(_origin, message: Vec<u8>) -> DispatchResult{
            print("Executing transaction, received message:");
            let converted_message: &[u8] = &message;
            print(converted_message);
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn do_finalize() {
        print("Block is finilized");
    }

    pub fn do_initialize(_block_number: T::BlockNumber) {
        print("Block is initialized");
    }

    pub fn do_commit() { print("Block is commited") }

    pub fn do_check_tx(_source: TransactionSource, _message: &Vec<u8>) {
        print("Validate from pallet");
        let converted_message: &[u8] = &_message;
        print(converted_message);
    }

    pub fn make_request() -> Result<Vec<Result<T::AccountId, ()>>, &'static str> {
        let signer = Signer::<T, T::AuthorityId>::all_accounts();
        if !signer.can_sign() {
            return Err(
                "No local accounts available. Consider adding one via `author_insertKey` RPC.",
            )?;
		}
		// Todo: Make gRPC request
        let results = signer.send_signed_transaction(|_account| Call::some_function());
        Ok(results.iter().map(|(acc, res)| match res {
            Ok(_) => Ok(acc.id.clone()),
            Err(_) => Err(()),
        }).collect())
    }
}
