use sc_cli::CliConfiguration;
use sc_service::Configuration;
use std::fmt::Debug;
use std::{fs, io, path};
use structopt::StructOpt;

pub use pallet_abci;

/// The `purge-abci-storage` command used to remove abci storage.
#[derive(Debug, StructOpt)]
pub struct PurgeAbciStorageCmd {
    #[allow(missing_docs)]
    #[structopt(flatten)]
    pub shared_params: sc_cli::SharedParams,
    #[structopt(short = "y")]
	pub yes: bool,
}

fn remove_rocks_db(db_name: &str, db_path: path::PathBuf) {
    match fs::remove_dir_all(&db_path) {
        Ok(_) => {
            println!("{:?} removed.", &db_name);
        },
        Err(_) => {
            println!("{:?} did not exist.", &db_path);
        }
    }
}

impl PurgeAbciStorageCmd {
    /// Run the purge command.
    pub fn run(&self, config: &Configuration) -> sc_cli::Result<()> {
        let chain_spec_id = config.chain_spec.id();
        let shared_params = self.shared_params();
        let database_params = self.database_params().unwrap_or(&sc_cli::DatabaseParams {
            database: None,
            database_cache_size: None,
        });
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

        if !self.yes {
            println!("Are you sure to remove {:?}? [y/N]: ", &db_path);

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            match input.chars().nth(0) {
                Some('y') | Some('Y') => {
                    remove_rocks_db(rocks_db_name, db_path);
                },
                _ => {
                    println!("Aborted");
                },
            }
        } else {
            remove_rocks_db(rocks_db_name, db_path);
        }

        let purge_chain_cmd = sc_cli::PurgeChainCmd {
            yes: false,
            shared_params: sc_cli::SharedParams {
                dev: shared_params.dev,
                chain: None,
                base_path: None,
                log: shared_params.log.clone(),
            },
            database_params: sc_cli::DatabaseParams {
                database: database_params.database.clone(),
                database_cache_size: database_params.database_cache_size.clone(),
            },
        };

        let _o = purge_chain_cmd.run(config.database.clone());

        Ok(())
    }
}

impl CliConfiguration for PurgeAbciStorageCmd {
    fn shared_params(&self) -> &sc_cli::SharedParams {
        &self.shared_params
    }
}
