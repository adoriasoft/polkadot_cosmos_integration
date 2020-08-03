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

use sp_runtime::print;

fn get_server_url() -> String {
    match std::env::var("ABCI_SERVER_URL") {
        Ok(val) => val,
        Err(_) => abci::DEFAULT_ABCI_URL.to_owned(),
    }
}

fn get_abci_app_state() -> sc_cli::Result<String> {
    std::env::var("ABCI_APP_STATE")
        .map_err(|_| sc_cli::Error::Other("Failed to get abci app state".into()))
}

fn init_chain() -> sc_cli::Result<()> {
    let app_state = get_abci_app_state()?;
    abci::connect_or_get_connection(&get_server_url())
        .map_err(|err| sc_cli::Error::Other(err.to_string()))?
        .init_chain("test-chain-id".to_owned(), app_state.as_bytes().to_vec())
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
        print("Load initial state");
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
