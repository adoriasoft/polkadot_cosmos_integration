use alt_serde::Serialize;

pub const ABCI_SERVER_URL: &str = "http://localhost:8082/abci/v1/";

pub fn get_method(method_name: &str) -> core::result::Result<(), &'static str> {
    let request_url = format!("{}{}", ABCI_SERVER_URL, method_name);
    let body = reqwest::blocking::get(&request_url)
        .map_err(|_| "Failed to send request")?
        .text()
        .map_err(|_| "Failed to get response")?;
    println!("body = {:?}", body);
    Ok(())
}

pub fn post_method<T: Serialize>(method_name: &str, _msg: &T) -> core::result::Result<(), &'static str> {
    let request_url = format!("{}{}", ABCI_SERVER_URL, method_name);
    let body = reqwest::blocking::get(&request_url)
        .map_err(|_| "Failed to send request")?
        .text()
        .map_err(|_| "Failed to get response")?;
    println!("body = {:?}", body);
    Ok(())
}
