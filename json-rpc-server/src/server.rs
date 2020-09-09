pub mod types;

use types::*;

use jsonrpc_http_server::jsonrpc_core::{serde_json::json, IoHandler, Params};
use jsonrpc_http_server::ServerBuilder;

pub fn start_server() {
    let mut io = IoHandler::new();
    io.add_method("abci_query", |params: Params| async {
        println!("abci_query");
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
        
        // TODO: parse result.proof and if it is qual to None in the json proof field put null
        // TODO: if key len == 0 put null in the json key field
        let res = json!({
            "response": {
                "log" : format!("{}", result.log),
                "height" : format!("{}", result.height),
                "proof" : null,
                "value" : format!("{}", base64::encode(result.value)),
                "key" : null ,
                "index" : format!("{}", result.index),
                "code" : format!("{}", result.code),
            }
        });
        Ok(res)
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:26657".parse().unwrap())
        .unwrap();
    server.wait();
}
