//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
pub use sc_rpc_api::DenyUnsafe;
use sp_transaction_pool::TransactionPool;
use substrate_frame_rpc_system::{FullSystem, SystemApi};

/// Full client dependencies.
pub struct FullDeps<C, P> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
}

/// Instantiate all full RPC extensions.
pub fn create_full<P: TransactionPool + 'static>(
    deps: FullDeps<crate::service::FullClient, P>,
) -> jsonrpc_core::IoHandler<sc_rpc::Metadata> {
    let mut io = jsonrpc_core::IoHandler::default();
    io.extend_with(SystemApi::to_delegate(FullSystem::new(
        deps.client.clone(),
        deps.pool.clone(),
        deps.deny_unsafe,
    )));
    io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(
        deps.client.clone(),
    )));
    io
}
