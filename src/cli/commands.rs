use sc_cli::CliConfiguration;
use sc_service::Configuration;
use std::fmt::Debug;
use std::io::Write;
use std::{fs, io};
use structopt::StructOpt;

pub use pallet_abci;

/// The `purge-abci-storage` command used to remove abci storage.
#[derive(Debug, StructOpt)]
pub struct PurgeAbciStorageCmd {
    #[allow(missing_docs)]
    #[structopt(flatten)]
    pub shared_params: sc_cli::SharedParams,
}

impl PurgeAbciStorageCmd {
    /// Run the purge command.
    pub fn run(&self, config: &Configuration) -> sc_cli::Result<()> {
        let chain_spec_id = config.chain_spec.id();

        println!("Remove storage of node with spec_id {:?}", chain_spec_id);

        let config_dir = config
            .base_path
            .as_ref()
            .ok_or_else(|| "base_path has not been set")
            .unwrap()
            .path()
            .to_path_buf()
            .join("chains")
            .join(chain_spec_id);
        let db_name = &pallet_abci::get_storage_name();
        let db_path = config_dir.join(&db_name);

        io::stdout().flush().expect("failed to flush stdout");

        match fs::remove_dir_all(&db_path) {
            Ok(_) => {
                println!("{:?} removed.", &db_name);
                Ok(())
            }
            Err(_) => {
                eprintln!("{:?} did not exist.", &db_path);
                Ok(())
            }
        }
    }
}

impl CliConfiguration for PurgeAbciStorageCmd {
    fn shared_params(&self) -> &sc_cli::SharedParams {
        &self.shared_params
    }
}
