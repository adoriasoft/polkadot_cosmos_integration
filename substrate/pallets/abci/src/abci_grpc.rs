#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::{Vec};
use frame_support::debug;
use sp_runtime::offchain::{http, http::Method};
use sp_std::str;
use frame_support::assert_ok;

use lite_json::json::{JsonValue, NumberValue};

pub const ABCI_SERVER_URL : &[u8] = b"http://localhost:8082/abci/v1/";

fn convert(data : &Vec<u8>) -> String {
    let mut res = "[".to_string();
    let data_len = data.len();
    for (i, el) in data.iter().enumerate() {
        if i != data.len() - 1 {
            res.push_str(&format!("{},", el));
        }
        else {
            res.push_str(&format!("{}", el));
        }
    }
    res.push_str("]");
    return res;
}

#[test]
fn convert_test() {
    let data : Vec<u8> = vec![1 , 2 ,3 ,4];
    let str_data = convert(&data);

    assert_eq!(str_data, "[1,2,3,4]");
}


pub struct BlockMessage {
    pub height : u64,
}

pub struct TxMessage {
    pub tx : Vec<u8>,
}

impl BlockMessage {
    pub fn serializeToJson(&self) -> Vec<u8> {
        let json = format!(r#""height" : {}"#, self.height);
        return json.into_bytes();
    }
}

impl TxMessage {
    pub fn serializeToJson(&self) -> Vec<u8> {
        let json = format!(r#""tx" : {}"#, convert(&self.tx));
        return json.into_bytes();
    }
}


pub fn InitChain() {
    let url : &[u8] = &[ABCI_SERVER_URL,  b"InitChain"].concat();
    let request_url = str::from_utf8(url).unwrap();

    let request = http::Request::get(request_url);

    let pending = request.send().unwrap();

    let response = pending.wait().unwrap();

    if response.code != 200 {
        debug::error!("Unexpected http request status code: {}", response.code);
    }
}

pub fn DeliverTx(tx_msg: TxMessage) {
    let url : &[u8] = &[ABCI_SERVER_URL,  b"DeliverTx"].concat();
    let request_url = str::from_utf8(url).unwrap();

    let request = http::Request::default()
        .method(Method::Post)
        .url(request_url)
        .body(vec![tx_msg.serializeToJson()])
        .add_header("Content-Type", "application/json");

    let pending = request.send().unwrap();

    let response = pending.wait().unwrap();

    if response.code != 200 {
        debug::error!("Unexpected http request status code: {}", response.code);
    }
}

pub fn CheckTx(tx_msg: TxMessage) {
    let url : &[u8] = &[ABCI_SERVER_URL,  b"CheckTx"].concat();
    let request_url = str::from_utf8(url).unwrap();

    let request = http::Request::default()
        .method(Method::Post)
        .url(request_url)
        .body(vec![tx_msg.serializeToJson()])
        .add_header("Content-Type", "application/json");

    let pending = request.send().unwrap();

    let response = pending.wait().unwrap();

    if response.code != 200 {
        debug::error!("Unexpected http request status code: {}", response.code);
    }
}

pub fn OnInitialize(blockMessage: BlockMessage) {
    let url : &[u8] = &[ABCI_SERVER_URL,  b"OnInitialize"].concat();
    let request_url = str::from_utf8(url).unwrap();

    let request = http::Request::default()
        .method(Method::Post)
        .url(request_url)
        .body(vec![blockMessage.serializeToJson()])
        .add_header("Content-Type", "application/json");

    let pending = request.send().unwrap();

    let response = pending.wait().unwrap();

    if response.code != 200 {
        debug::error!("Unexpected http request status code: {}", response.code);
    }
}

pub fn OnFinilize(blockMessage: BlockMessage) {
    let url : &[u8] = &[ABCI_SERVER_URL,  b"OnFinilize"].concat();
    let request_url = str::from_utf8(url).unwrap();

    let request = http::Request::default()
        .method(Method::Post)
        .url(request_url)
        .body(vec![blockMessage.serializeToJson()])
        .add_header("Content-Type", "application/json");

    let pending = request.send().unwrap();

    let response = pending.wait().unwrap();

    if response.code != 200 {
        debug::error!("Unexpected http request status code: {}", response.code);
    }
}

pub fn Commit(blockMessage: BlockMessage) {
    let url : &[u8] = &[ABCI_SERVER_URL,  b"Commit"].concat();
    let request_url = str::from_utf8(url).unwrap();

    let request = http::Request::default()
        .method(Method::Post)
        .url(request_url)
        .body(vec![blockMessage.serializeToJson()])
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