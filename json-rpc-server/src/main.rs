use std::sync::Arc;
use std::{thread, time};

use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::{hyper, ServerBuilder, RequestMiddleware, RequestMiddlewareAction};
use jsonrpc_pubsub::{PubSubHandler, PubSubMetadata, Session, Subscriber, SubscriptionId};

#[derive(Default, Clone)]
struct Meta;

impl Metadata for Meta {}

impl PubSubMetadata for Meta {
    fn session(&self) -> Option<Arc<Session>> { None }
}

struct Middleware;

impl RequestMiddleware for Middleware {
    fn on_request(&self, request: hyper::Request<hyper::Body>) -> RequestMiddlewareAction {
        println!("{:?}", request);
        RequestMiddlewareAction::Proceed {
            should_continue_on_invalid_cors: true,
            request,
        }
    }
}

fn main() {
    let mut io = PubSubHandler::new(MetaIoHandler::default());
    io.add_method("say_hello", |_params: Params| {
        Ok(Value::String("hello".to_string()))
    });

    io.add_method("/abci_query", |_: Params| {
        println!("abci_query");
        Ok(Value::String("hello".to_string()))
    });

    io.add_subscription(
        "/abci_query",
        (
            "abci_query",
            |params: Params, _, subscriber: Subscriber| {
                println!("HERE");

                if params != Params::None {
                    subscriber
                        .reject(Error {
                            code: ErrorCode::ParseError,
                            message: "Invalid parameters. Subscription rejected.".into(),
                            data: None,
                        })
                        .unwrap();
                    return;
                }

                thread::spawn(move || {
                    let sink = subscriber.assign_id(SubscriptionId::Number(5)).unwrap();
                    // or subscriber.reject(Error {} );
                    // or drop(subscriber)

                    loop {
                        thread::sleep(time::Duration::from_millis(100));
                        let res = sink.notify(Params::Array(vec![Value::Number(10.into())]));
                        println!("Subscription has ended, finishing. {:?}", res);
                        break;
                    }
                });
            },
        ),
        ("abci_query_end", |_id: SubscriptionId, _| {
            println!("Closing subscription");
            futures::future::ok(Value::Bool(true))
        }),
    );

    let server = ServerBuilder::with_meta_extractor(io, |_: &hyper::Request<hyper::Body>| Meta)
    .start_http(&"127.0.0.1:26657".parse().unwrap())
    .expect("Unable to start RPC server");

    server.wait();
}
