#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{decl_module, weights::Weight, dispatch::DispatchResult, sp_runtime::print};
use frame_system::{self as system};
use frame_support::sp_runtime::{
	transaction_validity::{TransactionSource},
};
use frame_support::dispatch::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {}

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

		//Simple unparametrized function, may be useful for test calls to the pallet
		#[weight = 10]
		pub fn some_function(_origin) {
			print("some_function")
		}

		/// Transaction execution
		#[weight = 0]
		pub fn deliver_tx(_origin, _message: Vec<u8>) -> DispatchResult{
			print("Executing transaction, received message:");
			let converted_message: &[u8] = &_message;
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

	pub fn check_tx(_source: TransactionSource, _message: &Vec<u8>) {
		print("Validate from pallet");
		let converted_message: &[u8] = &_message;
		print(converted_message);
	}
}
