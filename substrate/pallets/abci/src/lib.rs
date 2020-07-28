pub mod protos;

use protos::{abci_application_client, RequestEcho};
use std::time::Duration;
use tokio::runtime::Runtime;

pub const DEFAULT_ABCI_URL: &str = "tcp://localhost:26658";

type AbciResult<T> = Result<T, Box<dyn std::error::Error>>;
type AbciClient = abci_application_client::AbciApplicationClient<tonic::transport::Channel>;

pub struct Client {
    rt: Runtime,
    client: AbciClient,
}

impl Client {
    pub fn connect(abci_endpoint: &str) -> AbciResult<Self> {
        let mut rt = Runtime::new()?;
        let future = connect(abci_endpoint);
        let client = rt.block_on(async move {
            tokio::time::timeout(Duration::from_secs(1), future)
                .await
                .expect("failed to set timeout for future")
        })?;
        Ok(Client { rt, client })
    }

    pub fn echo(&mut self, message: String) -> AbciResult<()> {
        let request = tonic::Request::new(RequestEcho { message });

        let future = self.client.echo(request);
        let response = self.rt.block_on(async move {
            tokio::time::timeout(Duration::from_secs(1), future)
                .await
                .expect("failed to set timeout for future")
        })?;

        println!("RESPONSE={:?}", response);
        Ok(())
    }
}

async fn connect(abci_endpoint: &str) -> AbciResult<AbciClient> {
    // Get server URL from ENV variable and translate it into static str
    let endpoint: &'static str = Box::leak(abci_endpoint.into());
    let client = AbciClient::connect(endpoint).await?;
    Ok(client)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_abci_client_connection() {
        let client = Client::connect(DEFAULT_ABCI_URL);
        assert_eq!(client.is_ok(), true);
    }

    #[test]
    fn test_abci_echo() {
        let mut client = Client::connect(DEFAULT_ABCI_URL).unwrap();
        let result = client.echo("Hello there".to_owned());
        println!("result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }
}
