#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{decl_module, weights::Weight, dispatch::DispatchResult, sp_runtime::print};
use frame_system::{self as system};

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
		fn on_initialize(now: T::BlockNumber) -> Weight {
			Self::do_initialize(now);
			return 0;
		}
				
   		/// Block finalization
		fn on_finalize() {
			Self::do_finalize();    
		}
	}
}

impl<T: Trait> Module<T> {
	
	fn do_finalize() -> DispatchResult {
		print("Block is finilized");
		Ok(())
	}

	fn do_initialize(block_number: T::BlockNumber) -> DispatchResult {
		print("Block is initialized");
		Ok(())
	}
}
