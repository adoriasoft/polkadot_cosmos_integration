#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use frame_support::{debug, decl_module, dispatch::DispatchResult, dispatch::Vec, weights::Weight};
use frame_system::ensure_signed;
use sp_runtime::{traits::SaturatedConversion, DispatchError};
use sp_runtime_interface::runtime_interface;
use sp_std::prelude::*;

pub trait CosmosAbci {
    fn check_tx(tx: Vec<u8>) -> Result<u64, DispatchError>;
    fn deliver_tx(tx: Vec<u8>) -> DispatchResult;
}

/// The pallet's configuration trait.
pub trait Trait: frame_system::Trait {
    /// The overarching dispatch call type.
    type Call: From<Call<Self>>;
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Block initialization
        fn on_initialize(now: T::BlockNumber) -> Weight {
            match abci_interface::begin_block(
                "test-chain-id",
                now.saturated_into() as i64,
                vec![],
                vec![],
            ) {
                Err(err) => {
                    // We have to panic, as if cosmos will not have some blocks - it will fail.
                    panic!("Begin block failed: {:?}", err);
                },
                _ => {},
            }
            return 0;
        }

        /// Block finalization
        fn on_finalize(now: T::BlockNumber) {
            match abci_interface::end_block(now.saturated_into() as i64) {
                Ok(_) => {
                    match abci_interface::commit() {
                        Err(err) => {
                            // We have to panic, as if cosmos will not have some blocks - it will fail.
                            panic!("Commit failed: {:?}", err);
                        },
                        _ => {},
                    }
                },
                Err(err) => {
                    // We have to panic, as if cosmos will not have some blocks - it will fail.
                    panic!("End block failed: {:?}", err);
                },
            }
        }

        #[weight = 0]
        pub fn deliver_tx(origin, tx: Vec<u8>) -> DispatchResult {
            ensure_signed(origin)?;
            debug::info!("Received deliver tx request");
            <Self as CosmosAbci>::deliver_tx(tx)?;
            Ok(())
        }
    }
}

impl<T: Trait> CosmosAbci for Module<T> {
    fn check_tx(tx: Vec<u8>) -> Result<u64, DispatchError> {
        abci_interface::check_tx(tx)
    }

    fn deliver_tx(tx: Vec<u8>) -> DispatchResult {
        abci_interface::deliver_tx(tx)
    }
}

#[runtime_interface]
pub trait AbciInterface {
    fn echo(msg: &str) -> DispatchResult {
        let result = abci::connect_or_get_connection(&abci::get_server_url())
            .map_err(|_| "failed to setup connection")?
            .echo(msg.to_owned())
            .map_err(|_| "echo failed")?;
        debug::info!("Result: {:?}", result);
        Ok(())
    }

    fn check_tx(tx: Vec<u8>) -> Result<u64, DispatchError> {
        let result = abci::connect_or_get_connection(&abci::get_server_url())
            .map_err(|_| "failed to setup connection")?
            .check_tx(tx, 0)
            .map_err(|_| "check_tx failed")?;
        debug::info!("Result: {:?}", result);
        // If GasWanted is greater than GasUsed, we will increase the priority
        // Todo: Make it more logical
        let dif = result.gas_wanted - result.gas_used;
        Ok(dif as u64)
    }

    fn deliver_tx(tx: Vec<u8>) -> DispatchResult {
        let result = abci::connect_or_get_connection(&abci::get_server_url())
            .map_err(|_| "failed to setup connection")?
            .deliver_tx(tx)
            .map_err(|_| "deliver_tx failed")?;
        debug::info!("Result: {:?}", result);
        Ok(())
    }

    fn init_chain(chain_id: &str, app_state_bytes: Vec<u8>) -> DispatchResult {
        let result = abci::connect_or_get_connection(&abci::get_server_url())
            .map_err(|_| "failed to setup connection")?
            .init_chain(chain_id.to_owned(), app_state_bytes)
            .map_err(|_| "init_chain failed")?;
        debug::info!("Result: {:?}", result);
        Ok(())
    }

    fn begin_block(
        chain_id: &str,
        height: i64,
        hash: Vec<u8>,
        proposer_address: Vec<u8>,
    ) -> DispatchResult {
        let result = abci::connect_or_get_connection(&abci::get_server_url())
            .map_err(|_| "failed to setup connection")?
            .begin_block(chain_id.to_owned(), height, hash, proposer_address)
            .map_err(|_| "begin_block failed")?;
        debug::info!("Result: {:?}", result);
        Ok(())
    }

    fn end_block(height: i64) -> DispatchResult {
        let result = abci::connect_or_get_connection(&abci::get_server_url())
            .map_err(|_| "failed to setup connection")?
            .end_block(height)
            .map_err(|_| "end_block failed")?;
        debug::info!("Result: {:?}", result);
        Ok(())
    }

    fn commit() -> DispatchResult {
        let result = abci::connect_or_get_connection(&abci::get_server_url())
            .map_err(|_| "failed to setup connection")?
            .commit()
            .map_err(|_| "commit failed")?;
        debug::info!("Result: {:?}", result);
        Ok(())
    }
}
