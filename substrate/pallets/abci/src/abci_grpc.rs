#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::debug;
use frame_support::dispatch::Vec;
use sp_runtime::offchain::{http, http::Method};
use sp_std::str;

use alt_serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const ABCI_SERVER_URL: &[u8] = b"http://localhost:8082/abci/v1/";

#[serde(crate = "alt_serde")]
#[derive(Serialize, Deserialize)]
pub struct BlockMessage {
    pub height: u64,
}

#[serde(crate = "alt_serde")]
#[derive(Serialize, Deserialize)]
pub struct TxMessage {
    pub tx: Vec<u8>,
}

pub fn init_chain() {
    let url: &[u8] = &[ABCI_SERVER_URL, b"InitChain"].concat();
    let request_url = str::from_utf8(url).unwrap();

    let request = http::Request::get(request_url);

    let pending = request.send().unwrap();

    let response = pending.wait().unwrap();

    if response.code != 200 {
        debug::error!("Unexpected http request status code: {}", response.code);
    }
}

pub fn deliver_tx(tx_msg: &TxMessage) {
    let url: &[u8] = &[ABCI_SERVER_URL, b"DeliverTx"].concat();
    let request_url = str::from_utf8(url).unwrap();

    let serialized_data = serde_json::to_string(tx_msg).unwrap();

    let request = http::Request::default()
        .method(Method::Post)
        .url(request_url)
        .body([serialized_data.as_bytes()].to_vec())
        .add_header("Content-Type", "application/json");

    let pending = request.send().unwrap();

    let response = pending.wait().unwrap();

    if response.code != 200 {
        debug::error!("Unexpected http request status code: {}", response.code);
    }
}

pub fn check_tx(tx_msg: &TxMessage) {
    let url: &[u8] = &[ABCI_SERVER_URL, b"CheckTx"].concat();
    let request_url = str::from_utf8(url).unwrap();

    let serialized_data = serde_json::to_string(tx_msg).unwrap();

    let request = http::Request::default()
        .method(Method::Post)
        .url(request_url)
        .body([serialized_data.as_bytes()].to_vec())
        .add_header("Content-Type", "application/json");

    let pending = request.send().unwrap();

    let response = pending.wait().unwrap();

    if response.code != 200 {
        debug::error!("Unexpected http request status code: {}", response.code);
    }
}

pub fn on_initialize(blk_msg: &BlockMessage) {
    let url: &[u8] = &[ABCI_SERVER_URL, b"OnInitialize"].concat();
    let request_url = str::from_utf8(url).unwrap();

    let serialized_data = serde_json::to_string(blk_msg).unwrap();

    let request = http::Request::default()
        .method(Method::Post)
        .url(request_url)
        .body([serialized_data.as_bytes()].to_vec())
        .add_header("Content-Type", "application/json");

    let pending = request.send().unwrap();

    let response = pending.wait().unwrap();

    if response.code != 200 {
        debug::error!("Unexpected http request status code: {}", response.code);
    }
}

pub fn on_finilize(blk_msg: &BlockMessage) {
    let url: &[u8] = &[ABCI_SERVER_URL, b"OnFinilize"].concat();
    let request_url = str::from_utf8(url).unwrap();

    let serialized_data = serde_json::to_string(blk_msg).unwrap();

    let request = http::Request::default()
        .method(Method::Post)
        .url(request_url)
        .body([serialized_data.as_bytes()].to_vec())
        .add_header("Content-Type", "application/json");

    let pending = request.send().unwrap();

    let response = pending.wait().unwrap();

    if response.code != 200 {
        debug::error!("Unexpected http request status code: {}", response.code);
    }
}

pub fn commit(blk_msg: &BlockMessage) {
    let url: &[u8] = &[ABCI_SERVER_URL, b"Commit"].concat();
    let request_url = str::from_utf8(url).unwrap();

    let serialized_data = serde_json::to_string(blk_msg).unwrap();

    let request = http::Request::default()
        .method(Method::Post)
        .url(request_url)
        .body([serialized_data.as_bytes()].to_vec())
        .add_header("Content-Type", "application/json");

    let pending = request.send().unwrap();

    let response = pending.wait().unwrap();

    if response.code != 200 {
        debug::error!("Unexpected http request status code: {}", response.code);
    }
}

pub fn echo() {
    let url: &[u8] = &[ABCI_SERVER_URL, b"Echo"].concat();
    let request_url = str::from_utf8(url).unwrap();

    let request = http::Request::get(request_url);

    let pending = request.send().unwrap();

    let response = pending.wait().unwrap();

    if response.code != 200 {
        debug::error!("Unexpected http request status code: {}", response.code);
    }
}
