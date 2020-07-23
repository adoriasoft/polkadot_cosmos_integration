use alt_serde::Serialize;
use isahc::prelude::*;
use sp_std::str;
use std::time::Duration;

pub const ABCI_SERVER_URL: &str = "http://localhost:8082/abci/v1/";

fn get_server_url() -> String {
    match std::env::var("ABCI_SERVER_URL") {
        Ok(val) => val,
        Err(_) => ABCI_SERVER_URL.to_owned(),
    }
}

pub fn get_method(method_name: &str) -> core::result::Result<(), &'static str> {
    let request_url = format!("{}{}", &get_server_url(), method_name);
    let res = Request::get(&request_url)
        .header("Content-Type", "application/json")
        .timeout(Duration::from_secs(1))
        .body("")
        .map_err(|_| "Failed to add body to request")?
        .send()
        .map_err(|_| "Failed to send request")?;
    let status = res.status().as_u16();
    if status != 200 {
        Err("Unexpected HTTP request status code")
    } else {
        Ok(())
    }
}

pub fn post_method<T: Serialize>(
    method_name: &str,
    msg: &T,
) -> core::result::Result<(), &'static str> {
    let request_url = format!("{}{}", &get_server_url(), method_name);
    let val = serde_json::to_string(&msg).map_err(|_| "Failed to convert msg to json string")?;
    let res = Request::post(&request_url)
        .header("Content-Type", "application/json")
        .timeout(Duration::from_secs(1))
        .body(val)
        .map_err(|_| "Failed to add body to request")?
        .send()
        .map_err(|_| "Failed to send request")?;
    let status = res.status().as_u16();
    if status != 200 {
        Err("Unexpected HTTP request status code")
    } else {
        Ok(())
    }
}

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use tokio::runtime::Runtime;

pub fn send_hello_world_method() {
    println!("Making request to Greeter server");

    let mut rt = Runtime::new().expect("failed to obtain a new RunTime object");

    // Get server URL from ENV variable and translate it into static str
    let endpoint: &'static str = Box::leak(get_server_url().into_boxed_str());
    let client = GreeterClient::connect(endpoint);
    let mut client = rt
        .block_on(async move {
            tokio::time::timeout(Duration::from_secs(1), client)
                .await
                .expect("failed to get result from timeout")
        })
        .expect("failed to successfully run the future on RunTime");

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let req = client.say_hello(request);
    let response = rt
        .block_on(async move {
            tokio::time::timeout(Duration::from_secs(1), req)
                .await
                .expect("failed to get result from timeout")
        })
        .expect("failed to successfully run the future on RunTime");

    println!("RESPONSE={:?}", response);
}
