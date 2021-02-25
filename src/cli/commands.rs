use sc_cli::CliConfiguration;
use sc_service::Configuration;
use std::fmt::Debug;
use std::{fs, path};
use std::io::{self, Write};
use structopt::StructOpt;

pub use pallet_abci;

/// The `purge-abci-storage` command used to remove abci storage.
#[derive(Debug, StructOpt)]
pub struct PurgeChainWithStorageCmd {
    #[allow(missing_docs)]
    #[structopt(flatten)]
    pub shared_params: sc_cli::SharedParams,
    #[allow(missing_docs)]
    #[structopt(flatten)]
    pub database_params: sc_cli::DatabaseParams,
    #[structopt(short = "y")]
    pub yes: bool,
}

fn remove_rocks_db(db_name: &str, db_path: path::PathBuf) {
    match fs::remove_dir_all(&db_path) {
        Ok(_) => {
            println!("{:?} removed.", &db_name);
        }
        Err(_) => {
            println!("{:?} did not exist.", &db_path);
        }
    }
}

impl PurgeChainWithStorageCmd {
    /// Run the purge command.
    pub fn run(&self, config: &Configuration) -> sc_cli::Result<()> {
        let mut confirm_removal = self.yes;
        let chain_spec_id = config.chain_spec.id();
        let shared_params = self.shared_params();
        let database_params = self.database_params().unwrap();
        let config_dir = config
            .base_path
            .as_ref()
            .ok_or_else(|| "base_path has not been set")
            .unwrap()
            .path()
            .to_path_buf()
            .join("chains")
            .join(chain_spec_id);
        let rocks_db_name = &pallet_abci::get_storage_name();
        let db_path = config_dir.join(&rocks_db_name);

        if self.yes {
            remove_rocks_db(rocks_db_name, db_path);
        } else {
            print!("Are you sure to reset state? [y/N]: ");
            std::io::stdout().flush().expect("failed to flush stdout");

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            match input.chars().next() {
                Some('y') | Some('Y') => {
                    remove_rocks_db(rocks_db_name, db_path);
                    confirm_removal = true;
                }
                _ => {
                    println!("Aborted");
                }
            }
        }

        if confirm_removal {
            let purge_chain_cmd = sc_cli::PurgeChainCmd {
                yes: confirm_removal,
                shared_params: sc_cli::SharedParams {
                    dev: shared_params.dev,
                    chain: None,
                    base_path: None,
                    log: shared_params.log.clone(),
                },
                database_params: sc_cli::DatabaseParams {
                    database: database_params.database,
                    database_cache_size: database_params.database_cache_size,
                },
            };

            let result = purge_chain_cmd.run(config.database.clone());

            return result;
        }

        Ok(())
    }
}

impl CliConfiguration for PurgeChainWithStorageCmd {
    fn shared_params(&self) -> &sc_cli::SharedParams {
        &self.shared_params
    }
    fn database_params(&self) -> Option<&sc_cli::DatabaseParams> {
        Some(&self.database_params)
    }
}
