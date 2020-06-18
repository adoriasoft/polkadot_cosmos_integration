#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::{Vec};
use frame_support::debug;
use sp_runtime::offchain::{http, http::Method};
use sp_std::str;

use lite_json::json::JsonValue;

pub const ABCI_SERVER_URL : &[u8] = b"http://localhost:8082/abci/v1/";



pub fn DeliverTx(data: Vec<&[u8]>) {
    let url : &[u8] = &[ABCI_SERVER_URL,  b"DeliverTx"].concat();
    let request_url = str::from_utf8(url).unwrap();

    let request = http::Request::default()
        .method(Method::Post)
        .url(request_url)
        .body(data)
        .add_header("Content-Type", "application/json");

    let pending = request.send().unwrap();

    let response = pending.wait().unwrap();

    if response.code != 200 {
        debug::error!("Unexpected http request status code: {}", response.code);
    }
}

pub fn Echo() {
    let url : &[u8] = &[ABCI_SERVER_URL,  b"Echo"].concat();
    let request_url = str::from_utf8(url).unwrap();

    let request = http::Request::get(request_url);

    let pending = request.send().unwrap();

    let response = pending.wait().unwrap();

    if response.code != 200 {
           debug::error!("Unexpected http request status code: {}", response.code);
    }
}