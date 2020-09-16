//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod cosmos_rpc;
mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;

fn main() -> sc_cli::Result<()> {
	command::run()
}
