mod tx;
mod types;

use std::sync::Arc;
use node_template_runtime::opaque::Block;
use sp_transaction_pool::TransactionPool;
use jsonrpc_http_server::jsonrpc_core::{serde_json::json, IoHandler, Params};
use jsonrpc_http_server::ServerBuilder;

pub const DEFAULT_ABCI_RPC_URL: &str = "127.0.0.1:26657";

pub fn start_server<P: TransactionPool<Block = Block> + 'static>(
    client: Arc<crate::service::FullClient>,
    pool: Arc<P>,
) {
    let mut io = IoHandler::new();

    let clone_client = client.clone();
    let clone_pool = pool.clone();
    io.add_method("abci_query", move |params: Params| {
        let client = clone_client.clone();
        let pool = clone_pool.clone();
        async move {
            let query_params: types::ABCIQueryParams = params.parse().unwrap();
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
            // TODO: parse result.proof and if it is qual to None in the json proof field put null
            // TODO: if key len == 0 put null in the json key field
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
    });
    io.add_method("broadcast_tx_commit", move |params: Params| {
        let client = client.clone();
        let pool = pool.clone();
        async move {
            let params: types::ABCITxCommitParams = params.parse().unwrap();
            println!("params tx: {}", params.tx);
            let tx_value = base64::decode(params.tx).unwrap();
            let result = abci::connect_or_get_connection(&abci::get_server_url())
                .map_err(|_| "failed to setup connection")
                .unwrap()
                .check_tx(tx_value.clone(), 0)
                .map_err(|_| "query failed")
                .unwrap();
            println!("abci check_tx result: {:?}", result);
            let _deliver_tx_result = tx::deliver_tx(client.clone(), pool.clone(), tx_value).await;
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
    });

    std::thread::spawn(move || {
        let server = ServerBuilder::new(io)
            .threads(3)
            .start_http(&DEFAULT_ABCI_RPC_URL.parse().unwrap())
            .unwrap();
        server.wait();
    });
}
