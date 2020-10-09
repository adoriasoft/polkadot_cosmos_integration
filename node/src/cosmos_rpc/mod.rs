mod types;

use jsonrpc_http_server::jsonrpc_core::{serde_json::json, Error, IoHandler, Params};
use jsonrpc_http_server::ServerBuilder;
use node_template_runtime::cosmos_abci::ExtrinsicConstructionApi;
use node_template_runtime::opaque::Block;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::generic::BlockId;
use std::sync::Arc;

pub const DEFAULT_ABCI_RPC_URL: &str = "127.0.0.1:26657";
pub const FAILED_SETUP_CONNECTION_MSG: &str = "Failed to get abci instance.";

pub fn start_server(client: Arc<crate::service::FullClient>) {
    let mut io = IoHandler::new();

    fn handle_error(
        e: std::boxed::Box<dyn std::error::Error>,
    ) -> sc_service::Result<jsonrpc_core::Value, Error> {
        Ok(json!({ "error": e.to_string()}))
    }

    async fn fetch_abci_info(_: Params) -> sc_service::Result<jsonrpc_core::Value, Error> {
        let result = abci::get_abci_instance()
            .map_err(handle_error)
            .unwrap()
            .info()
            .map_err(handle_error)
            .unwrap();
        // todo Must it be save anywhere?
        // let last_block_app_hash = result.get_last_block_app_hash();
        // let last_block_height = result.get_last_block_height();

        Ok(json!({
            "response": {
                "data": format!("{}", result.get_data()),
                "version": format!("{}", result.get_version()),
                "app_version": format!("{}", result.get_app_version())
            }
        }))
    }

    async fn fetch_abci_set_option(
        params: Params,
    ) -> sc_service::Result<jsonrpc_core::Value, Error> {
        let query_params: types::AbciSetOption = params.parse().unwrap();
        let key: &str = &query_params.key;
        let value: &str = &query_params.value;
        let abci_instance_res = abci::get_abci_instance()
            .ok()
            .ok_or(FAILED_SETUP_CONNECTION_MSG);

        match abci_instance_res {
            Ok(mut abci_instance_res_ok) => {
                let abci_set_option_res = abci_instance_res_ok
                    .set_option(key, value)
                    .ok()
                    .ok_or("Failed to SetOption().");

                match abci_set_option_res {
                    Err(e) => Ok(json!({ "error": e })),
                    Ok(abci_set_option_res_ok) => Ok(json!({
                        "response": {
                            "code": format!("{}", abci_set_option_res_ok.get_code()),
                            "log": format!("{}", abci_set_option_res_ok.get_log()),
                            "info": format!("{}", abci_set_option_res_ok.get_info())
                        }
                    })),
                }
            }
            Err(e) => Ok(json!({ "error": e })),
        }
    }

    async fn query(params: Params) -> sc_service::Result<jsonrpc_core::Value, Error> {
        let query_params: types::ABCIQueryParams = params.parse().unwrap();
        let abci_instance_res = abci::get_abci_instance()
            .ok()
            .ok_or(FAILED_SETUP_CONNECTION_MSG);

        match abci_instance_res {
            Ok(mut abci_instance_res_ok) => {
                let data = hex::decode(query_params.data).unwrap_or(vec![]);
                let mut path = query_params.path;

                if path.chars().count() == 0 {
                    path = "/".to_string();
                }

                let height = query_params.height.parse::<i64>().unwrap_or(0);
                let abci_query_res = abci_instance_res_ok
                    .query(path, data, height, query_params.prove)
                    .ok()
                    .ok_or("Failed to Query().");

                match abci_query_res {
                    Err(e) => Ok(json!({ "error": e })),
                    Ok(abci_query_res_ok) => {
                        let origin_key = &abci_query_res_ok.get_key();
                        let origin_proof = &abci_query_res_ok.get_proof();
                        let origin_value = &abci_query_res_ok.get_value();

                        let mut proof: Option<String> = None;
                        let mut key: Option<String> = None;
                        let mut value: Option<String> = None;

                        match origin_proof {
                            Some(proof_res_ok) => {
                                proof = Some(format!("{:?}", proof_res_ok));
                            }
                            None => {}
                        }

                        match std::str::from_utf8(origin_key) {
                            Ok(key_res_ok) => {
                                let key_str = key_res_ok.to_string();
                                if key_str.chars().count() > 0 {
                                    key = Some(key_str);
                                }
                            }
                            Err(_e) => {}
                        }

                        match std::str::from_utf8(origin_value) {
                            Ok(value_res_ok) => {
                                let value_str = value_res_ok.to_string();
                                if value_str.chars().count() > 0 {
                                    value = Some(value_str);
                                }
                            }
                            Err(_e) => {}
                        }

                        Ok(json!({
                            "response": {
                                "log" : format!("{}", abci_query_res_ok.get_log()),
                                "height" : format!("{}", abci_query_res_ok.get_height()),
                                "key" : key,
                                "value" : value,
                                "index" : format!("{}", abci_query_res_ok.get_index()),
                                "code" : format!("{}", abci_query_res_ok.get_code()),
                                "proof" : proof,
                            }
                        }))
                    }
                }
            }
            Err(e) => Ok(json!({ "error": e })),
        }
    }

    io.add_method("abci_info", fetch_abci_info);

    io.add_method("abci_set_option", fetch_abci_set_option);

    io.add_method("abci_query", query);

    io.add_method("broadcast_tx_commit", move |params: Params| {
        let client = client.clone();
        async move {
            let params: types::ABCITxCommitParams = params.parse().unwrap();
            let tx_value = base64::decode(params.tx).unwrap();
            let result = abci::get_abci_instance()
                .map_err(|_| "failed to setup connection")
                .unwrap()
                .check_tx(tx_value.clone(), 0)
                .map_err(|_| "query failed")
                .unwrap();

            let info = client.info();
            let best_hash = info.best_hash;
            let best_height: u32 = info.best_number.into();
            let at = BlockId::<Block>::hash(best_hash);
            client
                .runtime_api()
                .broadcast_deliver_tx(&at, &tx_value)
                .ok();
            Ok(json!({
                "height": (best_height + 1).to_string(),
                "hash": "",
                "deliver_tx": {
                    "log": format!("{}", result.get_log()),
                    "data": format!("{}", base64::encode(result.get_data().clone())),
                    "code": format!("{}", result.get_code())
                },
                "check_tx": {
                    "log": format!("{}", result.get_log()),
                    "data": format!("{}", base64::encode(result.get_data())),
                    "code": format!("{}", result.get_code())
                }
            }))
        }
    });

    std::thread::spawn(move || {
        let server = ServerBuilder::new(io)
            .threads(3)
            .start_http(&DEFAULT_ABCI_RPC_URL.parse().unwrap())
            .unwrap();
        server.wait();
    });
}
