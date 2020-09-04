pub mod types;

use types::*;

use jsonrpc_http_server::jsonrpc_core::{serde_json::json, IoHandler, Params};
use jsonrpc_http_server::ServerBuilder;

pub fn start_server() {
    let mut io = IoHandler::new();
    io.add_method("abci_query", |params: Params| {
        println!("abci_query");
        let query_params: ABCIQueryParams = params.parse().unwrap();
        println!("params path: {}, data: {}, height: {}, prove: {}",
         query_params.path, query_params.data, query_params.height, query_params.prove);

         let result : abci::protos::ResponseQuery = abci::connect_or_get_connection(&abci::get_server_url())
         .map_err(|_| "failed to setup connection").unwrap()
         .query(
            query_params.path,
            query_params.data.as_bytes().to_vec(),
            query_params.height.parse::<i64>().unwrap(),
            query_params.prove)
         .map_err(|_| "query failed").unwrap();

         println!("abci query result: {:?}", result);

        async {
            let res = json!({
                "error" : "",
                 "result": json!({
                     "response" : json!({
                        "log" : format!("{}", result.log), 
                        "height" : format!("{}", result.height),
                        "proof" : format!("{:?}", result.proof),
                        "value" : format!("{:?}", result.value), 
                        "key" : format!("{:?}", result.key), 
                        "index" : format!("{}", result.index),
                        "code" : format!("{}", result.code), 
                     }),
                 }), 
                "id" : 0,
                "jsonrpc" : "2.0", 
            });
            Ok(())
        }
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:26657".parse().unwrap())
        .unwrap();
    server.wait();
}