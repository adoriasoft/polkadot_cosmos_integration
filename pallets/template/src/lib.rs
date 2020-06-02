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

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.
	// The overarching event type.
	//type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		#[weight = 10_000]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			// Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			//Something::put(something);

			// Here we are raising the Something event
			//Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}


		      
		 #[weight = 0]
		pub fn say_hi(origin) {
			// Ensure that the caller is a regular keypair account
			let caller = ensure_signed(origin)?;
            
			// Print a message
			print("Hi Worldqqq1");

		}
		
		fn on_finalize() {
			// at the end of the block, we can safely include the new VRF output
			// from this block into the under-construction randomness. If we've determined
			// that this block was the first in a new epoch, the changeover logic has
			// already occurred at this point, so the under-construction randomness
			// will only contain outputs from the right epoch.
            
            print("Block is finalized");
		}
		
		  /// Block initializing

		fn on_initialize()  -> Weight{
			// at the end of the block, we can safely include the new VRF output
			// from this block into the under-construction randomness. If we've determined
			// that this block was the first in a new epoch, the changeover logic has
			// already occurred at this point, so the under-construction randomness
			// will only contain outputs from the right epoch.
            
            print("Block is initialized");
			0
		}
		/* #[weight = 0]
		fn validate_transaction(origin,
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
		) -> TransactionValidity {
			 print("Validating traansaction!");
			 Executive::validate_transaction(tx)
		}
		*/
		
		
		
	}
	
}

impl<T: Trait> frame_support::unsigned::ValidateUnsigned for Module<T> {
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
}
/*
impl runtime_api::TaggedTransactionQueue<Block> for Module<T> {
	type Call = Call<T>;

	fn validate_transaction(__runtime_api_at_param__: &BlockId<Block>, 
        source: TransactionSource, 
        tx: <Block as BlockT>::Extrinsic) -> TransactionValidity {
		Executive::validate_transaction(tx)
	}
}*/
