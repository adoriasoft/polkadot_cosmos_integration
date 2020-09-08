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

use crate::chain_spec;
use crate::cli::Cli;
use crate::service;
use sc_cli::SubstrateCli;
use serde_json::Value;
use std::{fs, path::PathBuf};

fn from_json_file() -> sc_cli::Result<String> {
    let path: PathBuf = std::env::var("ABCI_APP_STATE_PATH")
        .map_err(|_| sc_cli::Error::Other("Failed to get app state file path".into()))?
        .into();
    let app_state = fs::read_to_string(&path)
        .map_err(|_| sc_cli::Error::Other("Error opening app state file".into()))?;
    Ok(app_state)
}

fn get_abci_genesis() -> String {
    let app_state = match from_json_file() {
        Ok(v) => v,
        _ => std::env::var("ABCI_GENESIS_STATE")
            .map_err(|_| sc_cli::Error::Other("Failed to get abci genesis state file".into()))
            .unwrap(),
    };
    app_state
}

fn init_chain() -> sc_cli::Result<()> {
    let genesis: Value = serde_json::from_str(&get_abci_genesis()).unwrap();

    let time = genesis["genesis_time"].as_str().unwrap();

    let mut pub_key_types: Vec<String> = Vec::new();

    for key_type in genesis["consensus_params"]["validator"]["pub_key_types"]
        .as_array()
        .unwrap()
    {
        pub_key_types.push(key_type.as_str().unwrap().to_string());
    }

    abci::connect_or_get_connection(&abci::get_server_url())
        .map_err(|err| sc_cli::Error::Other(err.to_string()))?
        .init_chain(
            genesis["chain_id"].as_str().unwrap().to_string(),
            genesis["app_state"].to_string().as_bytes().to_vec(),
            genesis["consensus_params"]["block"]["max_bytes"]
                .as_str()
                .unwrap()
                .parse::<i64>()
                .unwrap(),
            genesis["consensus_params"]["block"]["max_gas"]
                .as_str()
                .unwrap()
                .parse::<i64>()
                .unwrap(),
            genesis["consensus_params"]["evidence"]["max_age_num_blocks"]
                .as_str()
                .unwrap()
                .parse::<i64>()
                .unwrap(),
            genesis["consensus_params"]["evidence"]["max_age_duration"]
                .as_str()
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            pub_key_types,
        )
        .map_err(|err| sc_cli::Error::Other(err.to_string()))?;
    Ok(())
}

impl SubstrateCli for Cli {
    fn impl_name() -> &'static str {
        "Substrate Node"
    }

    fn impl_version() -> &'static str {
        env!("SUBSTRATE_CLI_IMPL_VERSION")
    }

    fn description() -> &'static str {
        env!("CARGO_PKG_DESCRIPTION")
    }

    fn author() -> &'static str {
        env!("CARGO_PKG_AUTHORS")
    }

    fn support_url() -> &'static str {
        "support.anonymous.an"
    }

    fn copyright_start_year() -> i32 {
        2017
    }

    fn executable_name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        Ok(match id {
            "dev" => Box::new(chain_spec::development_config()),
            "" | "local" => Box::new(chain_spec::local_testnet_config()),
            path => Box::new(chain_spec::ChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        })
    }
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        Some(subcommand) => {
            let runner = cli.create_runner(subcommand)?;
            runner.run_subcommand(subcommand, |config| Ok(new_full_start!(config).0))
        }
        None => {
            let runner = cli.create_runner(&cli.run)?;
            // Todo: Move to service.rs and add chain id param
            init_chain()?;
            runner.run_node(
                service::new_light,
                service::new_full,
                node_template_runtime::VERSION,
            )
        }
    }
}
