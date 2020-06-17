#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::Vec;
use sp_runtime::offchain::{http, Duration};
use sp_std::str;

pub const ABCI_SERVER_URL : &[u8] = b"localhost:8082/abci/v1";

pub fn CheckTx(_data: &Vec<u8>) {
}

pub fn Echo() {
    let method_name : &[u8] = b"Echo";
    let request_url = match str::from_utf8(&[ABCI_SERVER_URL, method_name].concat()) {
        Ok(request_url) => request_url,
        Err(_) => Err(),
    };

    let request = http::Request::get(request_url);
    request.send();
}