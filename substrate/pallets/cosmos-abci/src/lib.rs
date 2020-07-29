#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use alt_serde::{Deserialize, Serialize};
use codec::{Decode, Encode};
use frame_support::{
    debug, decl_module, decl_error, dispatch::DispatchResult, dispatch::Vec, weights::Weight,
};
use frame_system::ensure_signed;
// use sp_runtime::traits::SaturatedConversion;
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
            // abci_interface::on_initialize(&BlockMessage { height: now.saturated_into() as u64 });
            return 0;
        }

        /// Block finalization
        fn on_finalize(now: T::BlockNumber) {
            // abci_interface::on_finalize(&BlockMessage { height: now.saturated_into() as u64 });
        }

        #[weight = 0]
        pub fn deliver_tx(origin, tx: TxMessage) -> DispatchResult {
            ensure_signed(origin)?;
            debug::info!("Received deliver tx request");
            // match abci_interface::deliver_tx(&tx) {
            //     true => Ok(()),
            //     false => Err(<Error<T>>::CosmosError.into()),
            // }
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn check_tx(tx: Vec<u8>) -> DispatchResult {
        abci_interface::check_tx(tx)
    }
}

#[cfg(feature = "std")]
pub fn get_server_url() -> String {
    match std::env::var("ABCI_SERVER_URL") {
        Ok(val) => val,
        Err(_) => abci::DEFAULT_ABCI_URL.to_owned(),
    }
}

#[runtime_interface]
pub trait AbciInterface {


    fn echo() -> DispatchResult {
        let result = abci::ABCI_Client.lock().unwrap()
            .echo("Hello from runtime interface".to_owned())
            .map_err(|_| "echo failed")?;

        Ok(result)
    }

    fn check_tx(tx: Vec<u8>) -> DispatchResult {
        let result = abci::ABCI_Client.lock().unwrap()
        .check_tx(tx, 0)
        .map_err(|_| "check_tx failed")?;

        Ok(result)
    }

    fn deliver_tx(tx: Vec<u8>) -> DispatchResult {
        let result = abci::ABCI_Client.lock().unwrap()
        .deliver_tx(tx)
        .map_err(|_| "deliver_tx failed")?;
        
        Ok(result)
    }

    fn init_chain(chain_id: String, app_state_bytes: Vec<u8>) -> DispatchResult {
        let result = abci::ABCI_Client.lock().unwrap()
        .init_chain(chain_id, app_state_bytes)
        .map_err(|_| "init_chain failed")?;

        Ok(result)
    }

    fn begin_block(hash: Vec<u8>) -> DispatchResult {
        let result = abci::ABCI_Client.lock().unwrap()
        .begin_block(hash)
        .map_err(|_| "begin_block failed")?;

        Ok(result)
    }

    fn end_block(height: i64) -> DispatchResult {
        let result = abci::ABCI_Client.lock().unwrap()
        .end_block(height)
        .map_err(|_| "end_block failed")?;

        Ok(result)
    }

    fn commit() -> DispatchResult {
        let result = abci::ABCI_Client.lock().unwrap()
        .commit()
        .map_err(|_| "commit failed")?;

        Ok(result)
    }
}
