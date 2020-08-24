use jsonrpc_core::*;
use jsonrpc_http_server::*;

fn main() {
	let mut io = IoHandler::new();
	io.add_method("say_hello", |_: Params| {
		Ok(Value::String("hello".to_string()))
    });
    
    io.add_method("/abci_query", |_: Params| {
        println!("abci_query");
        Ok(Value::String("hello".to_string()))
    });

	let _server = ServerBuilder::new(io)
	.start_http(&"127.0.0.1:26657".parse().unwrap())
	.expect("Unable to start RPC server");

_server.wait();
}