use alt_serde::Serialize;
use sp_std::str;
use isahc::prelude::*;
use std::time::Duration;

pub const ABCI_SERVER_URL: &str = "http://localhost:8082/abci/v1/";

pub fn get_method(method_name: &str) -> core::result::Result<(), &'static str> {
    let request_url = format!("{}{}", ABCI_SERVER_URL, method_name);
    let res = isahc::get(&request_url)
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
    let request_url = format!("{}{}", ABCI_SERVER_URL, method_name);
    let val = serde_json::to_string(&msg).map_err(|_| "Failed to convert msg to json string")?;
    let res = Request::post(&request_url)
        .header("Content-Type", "application/json")
        .timeout(Duration::from_secs(1))
        .body(val).map_err(|_| "Failed to add body to request")?
        .send().map_err(|_| "Failed to send request")?;
    let status = res.status().as_u16();
    if status != 200 {
        Err("Unexpected HTTP request status code")
    } else {
        Ok(())
    }
}
