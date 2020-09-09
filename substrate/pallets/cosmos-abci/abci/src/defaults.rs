pub fn get_server_url() -> String {
    match std::env::var("ABCI_SERVER_URL") {
        Ok(val) => val,
        Err(_) => DEFAULT_ABCI_URL.to_owned(),
    }
}

pub const DEFAULT_ABCI_URL: &str = "tcp://localhost:26658";