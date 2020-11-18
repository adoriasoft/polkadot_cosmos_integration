//! A module that expose start_server() method for running Substrate RPC server from node.

/// Declare types module.
mod types;

use jsonrpc_http_server::jsonrpc_core::{serde_json::json, Error, ErrorCode, IoHandler, Params};
use jsonrpc_http_server::ServerBuilder;
use node_template_runtime::opaque::Block;
use node_template_runtime::pallet_cosmos_abci::ExtrinsicConstructionApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::generic::BlockId;
use std::sync::Arc;

/// Default ABCI RPC url.
pub const DEFAULT_ABCI_RPC_URL: &str = "127.0.0.1:26657";
/// Error message for failed connection.
pub const FAILED_SETUP_CONNECTION_MSG: &str = "Failed to get abci instance.";
/// Error message for decode tx failed.
pub const FAILED_TO_DECODE_TX_MSG: &str = "Failde to decode tx.";

/// Method for getting RPC server url form active env.
pub fn get_abci_rpc_url() -> String {
    match std::env::var("ABCI_RPC_SERVER_URL") {
        Ok(val) => val,
        Err(_) => DEFAULT_ABCI_RPC_URL.to_owned(),
    }
}

/// Method for start RPC server.
pub fn start_server(client: Arc<crate::service::FullClient>) {
    let mut io = IoHandler::new();

    /** Method for broadcasting abci tx value and return block best_number. */
    fn broadcast_abci_tx(tx_value: Vec<u8>, client: Arc<crate::service::FullClient>) -> u32 {
        let info = client.info();
        let best_hash = info.best_hash;
        let at = BlockId::<Block>::hash(best_hash);
        client.runtime_api().broadcast_abci_tx(&at, tx_value).ok();
        info.best_number
    };

    /** Handle and map RPC server error. */
    fn handle_error(e: std::boxed::Box<dyn std::error::Error>) -> Error {
        Error {
            code: ErrorCode::ServerError(1),
            message: e.to_string(),
            data: None,
        }
    }

    /** Handle and dispatch not critical RPC server error. */
    fn handle_ok_error(e: &str) -> sc_service::Result<jsonrpc_core::Value, Error> {
        Ok(json!({
            "error": e.to_string()
        }))
    }

    /** Substrate RPC info() method. */
    async fn fetch_abci_info(_: Params) -> sc_service::Result<jsonrpc_core::Value, Error> {
        let result = pallet_abci::get_abci_instance()
            .map_err(handle_error)?
            .info()
            .map_err(handle_error)?;

        Ok(json!({
            "response": {
                "data": result.get_data(),
                "version": result.get_version(),
                "app_version": result.get_app_version().to_string()
            }
        }))
    }

    /** Substrate RPC set_option() method. */
    async fn fetch_abci_set_option(
        params: Params,
    ) -> sc_service::Result<jsonrpc_core::Value, Error> {
        let query_params: types::AbciSetOption = params.parse()?;
        let key: &str = &query_params.key;
        let value: &str = &query_params.value;
        let abci_instance_res = pallet_abci::get_abci_instance()
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
                            "code": abci_set_option_res_ok.get_code().to_string(),
                            "log": abci_set_option_res_ok.get_log(),
                            "info": abci_set_option_res_ok.get_info()
                        }
                    })),
                }
            }
            Err(_e) => handle_ok_error(_e),
        }
    }

    /** Substrate RPC query() method. */
    async fn fetch_abci_query(params: Params) -> sc_service::Result<jsonrpc_core::Value, Error> {
        let query_params: types::AbciQueryParams = params.parse()?;
        let abci_instance_res = pallet_abci::get_abci_instance()
            .ok()
            .ok_or(FAILED_SETUP_CONNECTION_MSG);

        match abci_instance_res {
            Ok(mut abci_instance_res_ok) => {
                let data = hex::decode(query_params.data).unwrap_or_default();
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
                                "log" : abci_query_res_ok.get_log(),
                                "height" : abci_query_res_ok.get_height().to_string(),
                                "index" : abci_query_res_ok.get_index().to_string(),
                                "code" : abci_query_res_ok.get_code().to_string(),
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

    /** Substrate RPC flush() method. */
    async fn fetch_abci_flush(_: Params) -> sc_service::Result<jsonrpc_core::Value, Error> {
        let abci_instance_res = pallet_abci::get_abci_instance()
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

    /** Substrate RPC check_tx() method. */
    async fn abci_check_tx(params: Params) -> sc_service::Result<jsonrpc_core::Value, Error> {
        let query_params: types::AbciCheckTx = params.parse().unwrap();
        let tx = hex::decode(query_params.tx).unwrap_or_default();
        let abci_instance_res = pallet_abci::get_abci_instance()
            .ok()
            .ok_or(FAILED_SETUP_CONNECTION_MSG);

        match abci_instance_res {
            Ok(mut abci_instance_res_ok) => {
                let abci_check_tx_res = abci_instance_res_ok
                    .check_tx(tx)
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

    io.add_method("abci_info", fetch_abci_info);

    io.add_method("abci_set_option", fetch_abci_set_option);

    io.add_method("abci_query", fetch_abci_query);

    io.add_method("abci_flush", fetch_abci_flush);

    io.add_method("abci_check_tx", abci_check_tx);

    let client_tx_async_copy = client.clone();
    io.add_method("broadcast_tx_async", move |params: Params| {
        let client = client_tx_async_copy.clone();
        async move {
            let params: types::AbciTxBroadcastParams = params.parse()?;
            let tx_value = base64::decode(params.tx)
                .map_err(|_| handle_error(FAILED_TO_DECODE_TX_MSG.to_owned().into()))?;

            broadcast_abci_tx(tx_value, client);

            Ok(json!({
                "code": 0,
                "data": "",
                "log": "",
                "codespace": "",
                "hash": ""
            }))
        }
    });

    let client_tx_sync_copy = client.clone();
    io.add_method("broadcast_tx_sync", move |params: Params| {
        let client = client_tx_sync_copy.clone();
        async move {
            let params: types::AbciTxBroadcastParams = params.parse()?;
            let tx_value = base64::decode(params.tx)
                .map_err(|_| handle_error(FAILED_TO_DECODE_TX_MSG.to_owned().into()))?;

            let result = pallet_abci::get_abci_instance()
                .map_err(handle_error)?
                .check_tx(tx_value.clone())
                .map_err(handle_error)?;

            broadcast_abci_tx(tx_value, client);

            Ok(json!({
                "code": result.get_code(),
                "data": base64::encode(result.get_data()),
                "log": result.get_log(),
                "codespace": "",
                "hash": "",
            }))
        }
    });

    let client_commit_copy = client;
    io.add_method("broadcast_tx_commit", move |params: Params| {
        let client = client_commit_copy.clone();
        async move {
            let params: types::AbciTxCommitParams = params.parse()?;
            let tx_value = base64::decode(params.tx)
                .map_err(|_| handle_error(FAILED_TO_DECODE_TX_MSG.to_owned().into()))?;

            let result = pallet_abci::get_abci_instance()
                .map_err(handle_error)?
                .check_tx(tx_value.clone())
                .map_err(handle_error)?;

            let best_height: u32 = broadcast_abci_tx(tx_value, client);

            Ok(json!({
                "height": (best_height + 1).to_string(),
                "hash": "",
                "deliver_tx": {
                    "log": result.get_log(),
                    "data": base64::encode(result.get_data()),
                    "code": result.get_code().to_string()
                },
                "check_tx": {
                    "log": result.get_log(),
                    "data": base64::encode(result.get_data()),
                    "code": result.get_code().to_string()
                }
            }))
        }
    });

    std::thread::spawn(move || {
        let server = ServerBuilder::new(io)
            .threads(3)
            .start_http(&get_abci_rpc_url().as_str().parse().unwrap())
            .unwrap();
        server.wait();
    });
}
