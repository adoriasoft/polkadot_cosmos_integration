use jsonrpc_http_server::jsonrpc_core::{serde_json::json, IoHandler, Params};
use jsonrpc_http_server::ServerBuilder;

fn main() {
    let mut io = IoHandler::new();
    io.add_method("abci_query", |_params: Params| {
        println!("abci_query");
        async {
            let res = json!({
                // "account": json!({
                //     "type_url": "cosmos-sdk/BaseAccount",
                //     "value": json!(["an", "array"]),
                    // json!({
                    //     "address": "cosmos1fjjc22h4l6js58x4g03q4z0q67tqcdujycw5g5",
                    //     "public_key": "61rphyECFBOm5XiydVYhb2dKWcvRV5ymus4AqAfB86DBJKiO1V0=",
                    //     "sequence": "1",
                    // }),
                // }),
                "owner": "",
                "price": json!([json!({
                        "amount": "1",
                        "denom": "nametoken",
                    }),
                ]),
                "value": ""
            });
            Ok(res)
        }
    });
    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:26657".parse().unwrap())
        .unwrap();
    server.wait();
}
