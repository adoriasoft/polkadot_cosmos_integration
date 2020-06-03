#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch,
weights::Weight,};
use frame_system::{self as system, ensure_signed};
use frame_support::sp_runtime::{
	print,
	transaction_validity::{
		TransactionValidity, ValidTransaction, InvalidTransaction, TransactionSource,
		TransactionPriority,
	},
};
 use frame_support::dispatch::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		//Simple unparametrized function, may be useful for testing calls to the pallet
		#[weight = 10]
		pub fn some_function(origin) {
			print("some_function")
		}
   
		 #[weight = 0]
		pub fn deliver_tx(origin, message: Vec<u8>) {
			print("Executing transaction, sent message:");
			let converted_message: &[u8] = &message;
			print(converted_message)

		}
		
		fn on_finalize() {
            print("Block is finalized");
		}
		
		fn on_initialize()  -> Weight{
            print("Block is initialized");
			0
		}		
	}	
}

//Unnecessary now, may be useful in future
/*impl<T: Trait> frame_support::unsigned::ValidateUnsigned for Module<T> {
	type Call = Call<T>;
	fn validate_unsigned(
		_source: TransactionSource,
		call: &Self::Call,
	) -> TransactionValidity {
		print("Validate unsigned");
		ValidTransaction::with_tag_prefix("test")
				.priority(1)
				.and_provides([b"submit_number_unsigned"])
				.longevity(3)
				.propagate(true)
				.build()
	}
}*/