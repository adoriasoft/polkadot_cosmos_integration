pub mod protos;

use protos::{abci_application_client};
use std::time::Duration;
use tokio::runtime::Runtime;

use std::time::SystemTime;

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
        let request = tonic::Request::new(protos::RequestEcho { message });

        let future = self.client.echo(request);
        let response = self.rt.block_on(async move {
            tokio::time::timeout(Duration::from_secs(1), future)
                .await
                .expect("failed to set timeout for future")
        })?;

        println!("RESPONSE={:?}", response);
        Ok(())
    }

    // t: 0 - New, 1 - Recheck
    pub fn check_tx(&mut self, tx: Vec<u8>, t: i32) -> AbciResult<()> {
        let request = tonic::Request::new(protos::RequestCheckTx{tx: tx, r#type: t});

        let future = self.client.check_tx(request);
        let response = self.rt.block_on(async move {
            tokio::time::timeout(Duration::from_secs(1), future)
                .await
                .expect("failed to set timeout for future")
        })?;
                
        Ok(())
    }

    pub fn deliver_tx(&mut self, tx: Vec<u8>) -> AbciResult<()> {
        let request = tonic::Request::new(protos::RequestDeliverTx{tx});

        let future = self.client.deliver_tx(request);
        let response = self.rt.block_on(async move {
            tokio::time::timeout(Duration::from_secs(1), future)
                .await
                .expect("failed to set timeout for future")
        })?;
                
        Ok(())
    }

    pub fn init_chain(&mut self, chain_id: String, app_state_bytes: Vec<u8>) -> AbciResult<()> {
        let now = SystemTime::now();

        let request = tonic::Request::new(protos::RequestInitChain{chain_id: chain_id,
            app_state_bytes: app_state_bytes,
            time: None, validators: Vec::new(),
            consensus_params: None 
        });

        let future = self.client.init_chain(request);
        let response = self.rt.block_on(async move {
            tokio::time::timeout(Duration::from_secs(1), future)
                .await
                .expect("failed to set timeout for future")
        })?;
                
        Ok(())
    }

    pub fn begin_block(&mut self, hash: Vec<u8>, header: protos::Header) -> AbciResult<()> {
        let last_commit_info = protos::LastCommitInfo{
            round: 0,
            votes: Vec::new()
            };

        let request = tonic::Request::new(protos::RequestBeginBlock{hash: hash,
            header: Some(header),
            last_commit_info: Some(last_commit_info),
            byzantine_validators: Vec::new()
        });

        let future = self.client.begin_block(request);
        let response = self.rt.block_on(async move {
            tokio::time::timeout(Duration::from_secs(1), future)
                .await
                .expect("failed to set timeout for future")
        })?;
                
        Ok(())
    }

    pub fn end_block(&mut self, height: i64) -> AbciResult<()> {
        let request = tonic::Request::new(protos::RequestEndBlock{height: height});

        let future = self.client.end_block(request);
        let response = self.rt.block_on(async move {
            tokio::time::timeout(Duration::from_secs(1), future)
                .await
                .expect("failed to set timeout for future")
        })?;
                
        Ok(())
    }

    pub fn commit(&mut self) -> AbciResult<()> {
        let request = tonic::Request::new(protos::RequestCommit{});

        let future = self.client.commit(request);
        let response = self.rt.block_on(async move {
            tokio::time::timeout(Duration::from_secs(1), future)
                .await
                .expect("failed to set timeout for future")
        })?;
                
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

    #[test]
    fn test_abci_deliver_tx() {
        let mut client = Client::connect(DEFAULT_ABCI_URL).unwrap();
        let result = client.deliver_tx(Vec::new());
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_abci_check_tx() {
        let mut client = Client::connect(DEFAULT_ABCI_URL).unwrap();
        let result = client.check_tx(Vec::new(), 0);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_abci_init_chain() {
        let mut client = Client::connect(DEFAULT_ABCI_URL).unwrap();
        let result = client.init_chain("chain_id".to_owned(), Vec::new());
        assert_eq!(result.is_ok(), true);
    }

    // #[test]
    // fn test_abci_begin_block() {
    //     let mut client = Client::connect(DEFAULT_ABCI_URL).unwrap();
    //     let header = protos::Header{};

    //     let result = client.begin_block(Vec::new(), header);
    //     assert_eq!(result.is_ok(), true);
    // }

    #[test]
    fn test_abci_end_block() {
        let mut client = Client::connect(DEFAULT_ABCI_URL).unwrap();
        let result = client.end_block(10);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_abci_commit() {
        let mut client = Client::connect(DEFAULT_ABCI_URL).unwrap();
        let result = client.commit();
        assert_eq!(result.is_ok(), true);
    }

    
}
