mod types;

use jsonrpc_http_server::jsonrpc_core::{serde_json::json, Error, ErrorCode, IoHandler, Params};
use jsonrpc_http_server::ServerBuilder;
use node_template_runtime::cosmos_abci::ExtrinsicConstructionApi;
use node_template_runtime::opaque::Block;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::generic::BlockId;
use std::sync::Arc;

pub const DEFAULT_ABCI_RPC_URL: &str = "127.0.0.1:26657";
pub const FAILED_SETUP_CONNECTION_MSG: &str = "Failed to get abci instance.";
pub const FAILED_TO_DECODE_TX_MSG: &str = "Failde to decode tx.";

pub fn start_server(
    client_copy_commit: Arc<crate::service::FullClient>,
    client_copy_sync: Arc<crate::service::FullClient>,
    client_copy_async: Arc<crate::service::FullClient>,
) {
    let mut io = IoHandler::new();

    fn block_best_height(tx_value: Vec<u8>, client: Arc<crate::service::FullClient>) -> u32 {
        let info = client.info();
        let best_hash = info.best_hash;
        let at = BlockId::<Block>::hash(best_hash);
        client
            .runtime_api()
            .broadcast_deliver_tx(&at, &tx_value)
            .ok();

        info.best_number.into()
    };

    // Handlers.
    fn handle_error(e: std::boxed::Box<dyn std::error::Error>) -> Error {
        Error {
            code: ErrorCode::ServerError(1),
            message: e.to_string(),
            data: None,
        }
    }

    fn handle_ok_error(e: &str) -> sc_service::Result<jsonrpc_core::Value, Error> {
        Ok(json!({
            "error": e.to_string()
        }))
    }

    // Methods implementation.
    async fn fetch_abci_info(_: Params) -> sc_service::Result<jsonrpc_core::Value, Error> {
        let result = abci::get_abci_instance()
            .map_err(handle_error)?
            .info()
            .map_err(handle_error)?;

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
        let query_params: types::AbciSetOption = params.parse()?;
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
                    Err(_e) => handle_ok_error(_e),
                    Ok(abci_set_option_res_ok) => Ok(json!({
                        "response": {
                            "code": format!("{}", abci_set_option_res_ok.get_code()),
                            "log": format!("{}", abci_set_option_res_ok.get_log()),
                            "info": format!("{}", abci_set_option_res_ok.get_info())
                        }
                    })),
                }
            }
            Err(_e) => handle_ok_error(_e),
        }
    }

    async fn fetch_abci_query(params: Params) -> sc_service::Result<jsonrpc_core::Value, Error> {
        let query_params: types::AbciQueryParams = params.parse()?;
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
                                    value = Some(base64::encode(origin_value));
                                }
                            }
                            Err(_e) => {
                                value = Some(base64::encode(""));
                            }
                        }

                        Ok(json!({
                            "response": {
                                "log" : format!("{}", abci_query_res_ok.get_log()),
                                "height" : format!("{}", abci_query_res_ok.get_height()),
                                "index" : format!("{}", abci_query_res_ok.get_index()),
                                "code" : format!("{}", abci_query_res_ok.get_code()),
                                "key" : &key,
                                "value" : &value,
                                "proof" : &proof,
                            }
                        }))
                    }
                }
            }
            Err(_e) => handle_ok_error(_e),
        }
    }

    async fn fetch_abci_flush(_: Params) -> sc_service::Result<jsonrpc_core::Value, Error> {
        let abci_instance_res = abci::get_abci_instance()
            .ok()
            .ok_or(FAILED_SETUP_CONNECTION_MSG);

        match abci_instance_res {
            Ok(mut abci_instance_res_ok) => {
                let abci_flush_resp = abci_instance_res_ok
                    .flush()
                    .ok()
                    .ok_or("Failed to Flush().");
                match abci_flush_resp {
                    Ok(_) => Ok(json!({
                        "response": { }
                    })),
                    Err(_e) => handle_ok_error(_e),
                }
            }
            Err(_e) => handle_ok_error(_e),
        }
    }

    async fn abci_check_tx(params: Params) -> sc_service::Result<jsonrpc_core::Value, Error> {
        let query_params: types::AbciCheckTx = params.parse().unwrap();
        let tx = hex::decode(query_params.tx).unwrap_or(vec![]);
        let check_tx_type = query_params.check_tx_type;
        let abci_instance_res = abci::get_abci_instance()
            .ok()
            .ok_or(FAILED_SETUP_CONNECTION_MSG);

        match abci_instance_res {
            Ok(mut abci_instance_res_ok) => {
                let abci_check_tx_res = abci_instance_res_ok
                    .check_tx(tx, check_tx_type)
                    .ok()
                    .ok_or("Failed to CheckTx().");

                match abci_check_tx_res {
                    Ok(abci_check_tx_res_ok) => {
                        let origin_data = abci_check_tx_res_ok.get_data();
                        let mut data: Option<String> = None;

                        match std::str::from_utf8(&origin_data) {
                            Ok(data_res_ok) => {
                                let data_str = data_res_ok.to_string();
                                if data_str.chars().count() > 0 {
                                    data = Some(data_str);
                                }
                            }
                            Err(_e) => {}
                        }

                        Ok(json!({
                            "response": {
                                "code": abci_check_tx_res_ok.get_code(),
                                "info": abci_check_tx_res_ok.get_info(),
                                "log": abci_check_tx_res_ok.get_log(),
                                "data": data,
                                "gas_wanted": abci_check_tx_res_ok.get_gas_wanted(),
                                "gas_used": abci_check_tx_res_ok.get_gas_used(),
                                "codespace": abci_check_tx_res_ok.get_codespace()
                            }
                        }))
                    }
                    Err(_e) => handle_ok_error(_e),
                }
            }
            Err(_e) => handle_ok_error(_e),
        }
    }

    // IO methods mapping.
    io.add_method("abci_info", fetch_abci_info);

    io.add_method("abci_set_option", fetch_abci_set_option);

    io.add_method("abci_query", fetch_abci_query);

    io.add_method("abci_flush", fetch_abci_flush);

    io.add_method("abci_check_tx", abci_check_tx);

    io.add_method("broadcast_tx_async", move |params: Params| {
        let client = client_copy_async.clone();
        async move {
            let params: types::AbciTxCommitParams = params.parse()?;
            let tx_value = base64::decode(params.tx)
                .map_err(|_| handle_error(FAILED_TO_DECODE_TX_MSG.to_owned().into()))?;

            block_best_height(tx_value, client);

            Ok(json!({
                "code": 0,
                "data": "",
                "log": "",
                "codespace": "",
                "hash": ""
            }))
        }
    });

    io.add_method("broadcast_tx_sync", move |params: Params| {
        let client = client_copy_sync.clone();
        async move {
            let params: types::AbciTxCommitParams = params.parse()?;
            let tx_value = base64::decode(params.tx)
                .map_err(|_| handle_error(FAILED_TO_DECODE_TX_MSG.to_owned().into()))?;

            let result = abci::get_abci_instance()
                .map_err(handle_error)?
                .check_tx(tx_value.clone(), 0)
                .map_err(handle_error)?;

            block_best_height(tx_value, client);

            Ok(json!({
                "code": result.get_code(),
                "data": format!("{}", base64::encode(result.get_data())),
                "log": format!("{}", result.get_log()),
                "codespace": "",
                "hash": "",
            }))
        }
    });

    io.add_method("broadcast_tx_commit", move |params: Params| {
        let client = client_copy_commit.clone();
        async move {
            let params: types::AbciTxCommitParams = params.parse()?;
            let tx_value = base64::decode(params.tx)
                .map_err(|_| handle_error(FAILED_TO_DECODE_TX_MSG.to_owned().into()))?;

            let result = abci::get_abci_instance()
                .map_err(handle_error)?
                .check_tx(tx_value.clone(), 0)
                .map_err(handle_error)?;

            let best_height: u32 = block_best_height(tx_value, client);

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

    // Running.
    std::thread::spawn(move || {
        let server = ServerBuilder::new(io)
            .threads(3)
            .start_http(&DEFAULT_ABCI_RPC_URL.parse().unwrap())
            .unwrap();
        server.wait();
    });
}
