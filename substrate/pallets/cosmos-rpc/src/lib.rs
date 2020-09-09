mod types;

use jsonrpc_http_server::jsonrpc_core::{serde_json::json, Error, IoHandler, Params, Value};
use jsonrpc_http_server::ServerBuilder;
use crate::types::*;

pub const DEFAULT_ABCI_RPC_URL: &str = "127.0.0.1:26657";

pub fn start_server(uri: &str) {
    let mut io = IoHandler::new();

    io.add_method("abci_query", handle_abci_query);
    io.add_method("broadcast_tx_commit", handle_broadcast_tx_commit);

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&uri.parse().unwrap())
        .unwrap();
    server.wait();
}

async fn handle_abci_query(params: Params) -> Result<Value, Error> {
    let query_params: ABCIQueryParams = params.parse().unwrap();
    println!(
        "params path: {}, data: {}, height: {}, prove: {}",
        query_params.path, query_params.data, query_params.height, query_params.prove
    );
    let result: abci::protos::ResponseQuery =
        abci::connect_or_get_connection(&abci::get_server_url())
            .map_err(|_| "failed to setup connection")
            .unwrap()
            .query(
                query_params.path,
                hex::decode(query_params.data).expect("Decoding failed"),
                query_params.height.parse::<i64>().unwrap(),
                query_params.prove,
            )
            .map_err(|_| "query failed")
            .unwrap();
    println!("abci query result: {:?}", result);
    Ok(json!({
        "response": {
            "log" : format!("{}", result.log),
            "height" : format!("{}", result.height),
            "proof" : null,
            "value" : format!("{}", base64::encode(result.value)),
            "key" : format!("{}", std::str::from_utf8(&result.key).unwrap()),
            "index" : format!("{}", result.index),
            "code" : format!("{}", result.code),
        }
    }))
}

async fn handle_broadcast_tx_commit(params: Params) -> Result<Value, Error> {
    let params: ABCITxCommitParams = params.parse().unwrap();
    println!("params tx: {}", params.tx);
    let result = abci::connect_or_get_connection(&abci::get_server_url())
        .map_err(|_| "failed to setup connection")
        .unwrap()
        .check_tx(params.tx.as_bytes().to_vec(), 0)
        .map_err(|_| "query failed")
        .unwrap();
    println!("abci check_tx result: {:?}", result);
    Ok(json!({
        "height": "26682",
        "hash": "75CA0F856A4DA078FC4911580360E70CEFB2EBEE",
        "deliver_tx": {
            "log": "",
            "data": "",
            "code": "0"
        },
        "check_tx": {
            "log": format!("{}", result.log),
            "data": format!("{}", base64::encode(result.data)),
            "code": format!("{}", result.code)
        }
    }))
}
