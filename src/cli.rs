use sc_cli::RunCmd;
use structopt::StructOpt;
pub mod commands;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub subcommand: Option<Subcommand>,
    /// ABCI Genesis state path
    #[structopt(
        long = "abci_genesis_state_path",
        about = "ABCI Genesis state path",
        default_value = ""
    )]
    pub path_to_genesis: String,
    /// Path to ABCI server
    #[structopt(
        long = "abci_server_url",
        about = "Path to ABCI server",
        default_value = ""
    )]
    pub abci_server_url: String,
    /// Path to ABCI RPC server
    #[structopt(
        long = "abci_rpc_url",
        about = "Path to ABCI RPC server",
        default_value = ""
    )]
    pub abci_rpc_url: String,
    #[structopt(flatten)]
    pub run: RunCmd,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    /// Build a chain specification.
    BuildSpec(sc_cli::BuildSpecCmd),

    /// Validate blocks.
    CheckBlock(sc_cli::CheckBlockCmd),

    /// Export blocks.
    ExportBlocks(sc_cli::ExportBlocksCmd),

    /// Export the state of a given block into a chain spec.
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks.
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Remove the whole chain.
    PurgeChain(commands::PurgeChainWithStorageCmd),

    /// Revert the chain to a previous state.
    Revert(sc_cli::RevertCmd),

    /// The custom benchmark subcommmand benchmarking runtime pallets.
    #[structopt(name = "benchmark", about = "Benchmark runtime pallets.")]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),
}
