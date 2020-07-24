use abci_proto::abci_client::AbciClient;
use abci_proto::EmptyMessage;
use tendermint_proto::abci_application_client::AbciApplicationClient;
use tendermint_proto::RequestEcho;
use std::time::Duration;
use tokio::runtime::Runtime;

pub mod abci_proto {
    tonic::include_proto!("abci");
}

pub mod tendermint_proto {
    tonic::include_proto!("types");
}

pub fn send_test_method(abci_endpoint: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut rt = Runtime::new()?;

    // Get server URL from ENV variable and translate it into static str
    let endpoint: &'static str = Box::leak(abci_endpoint.into_boxed_str());
    let client = AbciClient::connect(endpoint);
    let mut client = rt.block_on(async move {
        tokio::time::timeout(Duration::from_secs(1), client)
            .await
            .expect("failed to set timeout for future")
    })?;

    let request = tonic::Request::new(EmptyMessage {});

    let req = client.init_chain(request);
    let response = rt.block_on(async move {
        tokio::time::timeout(Duration::from_secs(1), req)
            .await
            .expect("failed to set timeout for future")
    })?;

    println!("RESPONSE={:?}", response);
    Ok(())
}

pub fn send_tendermint_method(abci_endpoint: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut rt = Runtime::new()?;

    // Get server URL from ENV variable and translate it into static str
    let endpoint: &'static str = Box::leak(abci_endpoint.into_boxed_str());
    let client = AbciApplicationClient::connect(endpoint);
    let mut client = rt.block_on(async move {
        tokio::time::timeout(Duration::from_secs(1), client)
            .await
            .expect("failed to set timeout for future")
    })?;

    let request = tonic::Request::new(RequestEcho {
        // tx: "Hello".as_bytes().to_vec(),
        message: "Hello".to_owned(),
    });

    let req = client.echo(request);
    let response = rt.block_on(async move {
        tokio::time::timeout(Duration::from_secs(1), req)
            .await
            .expect("failed to set timeout for future")
    })?;

    println!("RESPONSE={:?}", response);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_logic() {
        let result = send_test_method("http://0.0.0.0:8081".to_owned());
        // let result = send_test_method("tcp://127.0.0.1:26658".to_owned());
        println!("result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_tendermint_req() {
        // let result = send_test_method("http://0.0.0.0:8081".to_owned());
        let result = send_tendermint_method("tcp://127.0.0.1:26658".to_owned());
        println!("result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }
}
