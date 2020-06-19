#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::{Vec};
use frame_support::debug;
use sp_runtime::offchain::{http,};
use sp_std::str;

use alt_serde::{Deserialize, Serialize,};

pub const ABCI_SERVER_URL : &[u8] = b"http://localhost:8082/abci/v1/";

#[serde(crate = "alt_serde")]
#[derive(Serialize, Deserialize)]
pub struct BlockMessage {
    pub height : u64,
}

#[serde(crate = "alt_serde")]
#[derive(Serialize, Deserialize)]
pub struct TxMessage {
    pub tx : Vec<u8>,
}

fn get_method(method_name: &str) {
    let url : &[u8] = &[ABCI_SERVER_URL,  method_name.as_bytes()].concat();
    let request_url = str::from_utf8(url).unwrap();

    let request = http::Request::get(request_url);

    let pending = request.send().unwrap();

    let response = pending.wait().unwrap();

    if response.code != 200 {
        debug::error!("Unexpected http request status code: {}", response.code);
    }

}

fn post_method<T : Serialize>(msg: &T, method_name: &str) {
    let url : &[u8] = &[ABCI_SERVER_URL,  method_name.as_bytes()].concat();
    let request_url = str::from_utf8(url).unwrap();

    let serialized_data = serde_json::to_string(msg).unwrap();

    let request = http::Request::post(request_url, [serialized_data.as_bytes()].to_vec())
        .add_header("Content-Type", "application/json");

    let pending = request.send().unwrap();

    let response = pending.wait().unwrap();

    if response.code != 200 {
        debug::error!("Unexpected http request status code: {}", response.code);
    }
}

pub fn InitChain() {
    get_method("InitChain");
}

pub fn DeliverTx(tx_msg: &TxMessage) {
    post_method(tx_msg, "DeliverTx");
}

pub fn CheckTx(tx_msg: &TxMessage) {
    post_method(tx_msg, "CheckTx")
}

pub fn OnInitialize(blk_msg: &BlockMessage) {
    post_method(blk_msg, "OnInitialize");
}

pub fn OnFinilize(blk_msg: &BlockMessage) {
    post_method(blk_msg, "OnFinilize");
}

pub fn Commit(blk_msg: &BlockMessage) {
    post_method(blk_msg, "Commit");
}

pub fn Echo() {
    get_method("Echo");
}