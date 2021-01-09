// This file is part of Substrate.

// Copyright (C) 2017-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::cli::{Cli, Subcommand};
use crate::{chain_spec, service};
use node_template_runtime::Block;
use sc_cli::{ChainSpec, Role, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;

fn init_chain() -> sc_cli::Result<()> {
    let key = b"init_chain_info".to_vec();
    let value = b"init_chain_info".to_vec();

    let mut abci_storage = abci_storage::get_abci_storage_instance()
        .map_err(|_| "failed to get abci storage instance")?;

    match abci_storage
        .get(key.clone())
        .map_err(|_| "failed to get value from the abci storage")?
    {
        // Just check that in storage exists some value to the following key
        Some(_) => {}
        None => {
            let genesis = pallet_abci::utils::parse_cosmos_genesis_file(
                &pallet_abci::utils::get_abci_genesis(),
            )
            .map_err(|_| "failed to get cosmos genesis file")?;
            let init_chain_response = pallet_abci::get_abci_instance()
                .map_err(|_| "failed to setup connection")?
                .init_chain(
                    genesis.time_seconds,
                    genesis.time_nanos,
                    &genesis.chain_id,
                    genesis.pub_key_types,
                    genesis.max_bytes,
                    genesis.max_gas,
                    genesis.max_age_num_blocks,
                    genesis.max_age_duration,
                    genesis.app_state_bytes,
                    vec![]
                )
                .map_err(|_| "init chain failed")?;
            let bytes = pallet_abci::utils::serialize_vec(
                init_chain_response
                    .get_validators()
                    .iter()
                    .map(|validator| {
                        let mut pub_key = vec![];
                        match &validator.pub_key {
                            Some(key) => pub_key = key.data.clone(),
                            None => {}
                        }
                        pallet_abci::utils::SerializableValidatorUpdate {
                            key_data: pub_key,
                            r#type: "ed25519".to_owned(),
                            power: validator.power,
                        }
                    })
                    .collect(),
            )
            .map_err(|_| "cannot serialize cosmos validators")?;
            abci_storage
                .write(0_i64.to_ne_bytes().to_vec(), bytes)
                .map_err(|_| "failed to write validators into the abci storage")?;
            abci_storage
                .write(key, value)
                .map_err(|_| "failed to write some data into the abci storage")?;
        }
    }

    Ok(())
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Substrate Node".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "support.anonymous.an".into()
    }

    fn copyright_start_year() -> i32 {
        2017
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        Ok(match id {
            "dev" => Box::new(chain_spec::development_config()?),
            "" | "local" => Box::new(chain_spec::local_testnet_config()?),
            path => Box::new(chain_spec::ChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        })
    }

    fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        &node_template_runtime::VERSION
    }
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
    // Init Abci instance
    let abci_server_url = &pallet_abci::get_server_url();
    pallet_abci::set_abci_instance(Box::new(
        pallet_abci::grpc::AbciinterfaceGrpc::connect(abci_server_url)
            .map_err(|_| "failed to connect")
            .unwrap(),
    ))
    .map_err(|_| "failed to set abci instance")
    .unwrap();

    abci_storage::set_abci_storage_instance(Box::new(
        abci_storage::rocksdb::AbciStorageRocksdb::init("abci_storage_rocksdb")
            .map_err(|_| "failed to open abci storage")
            .unwrap(),
    ))
    .map_err(|_| "failed to set abci storage instance")
    .unwrap();

    let cli = Cli::from_args();
    match cli.subcommand {
        Some(Subcommand::BuildSpec(ref cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(ref cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::ExportBlocks(ref cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, config.database), task_manager))
            })
        }
        Some(Subcommand::ExportState(ref cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, config.chain_spec), task_manager))
            })
        }
        Some(Subcommand::ImportBlocks(ref cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::PurgeChain(ref cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.database))
        }
        Some(Subcommand::Revert(ref cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    backend,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, backend), task_manager))
            })
        }
        Some(Subcommand::Benchmark(ref cmd)) => {
            if cfg!(feature = "runtime-benchmarks") {
                let runner = cli.create_runner(cmd)?;

                runner.sync_run(|config| cmd.run::<Block, service::Executor>(config))
            } else {
                Err("Benchmarking wasn't enabled when building the node. \
				You can enable it with `--features runtime-benchmarks`."
                    .into())
            }
        }
        None => {
            let runner = cli.create_runner(&cli.run)?;
            init_chain()?;
            runner.run_node_until_exit(|config| match config.role {
                Role::Light => service::new_light(config),
                _ => service::new_full(config),
            })
        }
    }
}
