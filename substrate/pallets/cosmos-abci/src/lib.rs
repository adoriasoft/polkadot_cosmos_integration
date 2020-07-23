#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(feature = "std")]
mod request;
#[cfg(test)]
mod tests;

use alt_serde::{Deserialize, Serialize};
use codec::{Decode, Encode};
use frame_support::{
    debug, decl_module, decl_error, dispatch::DispatchResult, dispatch::Vec,
    sp_runtime::transaction_validity::TransactionSource, weights::Weight,
};
use frame_system::ensure_signed;
use sp_runtime::traits::SaturatedConversion;
use sp_runtime_interface::{pass_by::PassByCodec, runtime_interface};
use sp_std::prelude::*;

#[serde(crate = "alt_serde")]
#[derive(Encode, Decode, Serialize, Deserialize, PassByCodec)]
pub struct BlockMessage {
    pub height: u64,
}

#[serde(crate = "alt_serde")]
#[derive(Encode, Decode, Default, Clone, Debug, PartialEq, Serialize, Deserialize, PassByCodec)]
pub struct TxMessage {
    pub tx: Vec<u8>,
}

/// The pallet's configuration trait.
pub trait Trait: frame_system::Trait {
    /// The overarching dispatch call type.
    type Call: From<Call<Self>>;
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Cosmos returned an error
        CosmosError,
    }
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Block initialization
        fn on_initialize(now: T::BlockNumber) -> Weight {
            abci_interface::on_initialize(&BlockMessage { height: now.saturated_into() as u64 });
            return 0;
        }

        /// Block finalization
        fn on_finalize(now: T::BlockNumber) {
            abci_interface::on_finalize(&BlockMessage { height: now.saturated_into() as u64 });
        }

        #[weight = 0]
        pub fn deliver_tx(origin, tx: TxMessage) -> DispatchResult {
            ensure_signed(origin)?;
            debug::info!("Received deliver tx request");
            match abci_interface::deliver_tx(&tx) {
                true => Ok(()),
                false => Err(<Error<T>>::CosmosError.into()),
            }
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn do_init_chain() {
        abci_interface::init_chain();
    }

    pub fn do_commit(height: u64) {
        abci_interface::commit(&BlockMessage { height });
    }

    pub fn do_check_tx(_source: TransactionSource, tx: Vec<u8>) {
        abci_interface::check_tx(&TxMessage { tx });
    }
}

#[runtime_interface]
pub trait AbciInterface {
    fn init_chain() -> bool {
        crate::request::get_method("InitChain").is_ok()
    }

    fn deliver_tx(_tx_msg: &TxMessage) -> bool {
        abci::send_test_method(crate::request::get_server_url());
        true
        // crate::request::post_method("DeliverTx", tx_msg).is_ok()
    }

    fn check_tx(tx_msg: &TxMessage) -> bool {
        crate::request::post_method("CheckTx", tx_msg).is_ok()
    }

    fn on_initialize(blk_msg: &BlockMessage) -> bool {
        crate::request::post_method("OnInitialize", blk_msg).is_ok()
    }

    fn on_finalize(blk_msg: &BlockMessage) -> bool {
        crate::request::post_method("OnFinilize", blk_msg).is_ok()
    }

    fn commit(blk_msg: &BlockMessage) -> bool {
        crate::request::post_method("Commit", blk_msg).is_ok()
    }
}
