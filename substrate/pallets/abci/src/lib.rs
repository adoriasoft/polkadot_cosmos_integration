pub mod protos;

use lazy_static::lazy_static;
use owning_ref::MutexGuardRefMut;
use protos::abci_application_client;
use std::{sync::Mutex, time::Duration};
use tokio::runtime::Runtime;

lazy_static! {
    static ref ABCI_CLIENT: Mutex<Option<Client>> = Mutex::new(None);
}

pub const DEFAULT_ABCI_URL: &str = "tcp://localhost:26658";

type AbciResult<T> = Result<T, Box<dyn std::error::Error>>;
type AbciClient = abci_application_client::AbciApplicationClient<tonic::transport::Channel>;

pub struct Client {
    rt: Runtime,
    client: AbciClient,
}

pub fn connect_or_get_connection<'ret>(
    abci_endpoint: &str,
) -> AbciResult<MutexGuardRefMut<'ret, Option<Client>, Client>> {
    let mut client = ABCI_CLIENT.lock()?;
    if client.is_none() {
        *client = Some(Client::connect(abci_endpoint)?);
    }
    // Here we create a ref to the inner value of the mutex guard.
    // Unwrap should never panic as we set it previously.
    let res = MutexGuardRefMut::new(client).map_mut(|mg| mg.as_mut().unwrap());
    Ok(res)
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

    /// Type: 0 - New, 1 - Recheck
    pub fn check_tx(&mut self, tx: Vec<u8>, r#type: i32) -> AbciResult<()> {
        let request = tonic::Request::new(protos::RequestCheckTx { tx, r#type });

        let future = self.client.check_tx(request);
        let response = self.rt.block_on(async move {
            tokio::time::timeout(Duration::from_secs(1), future)
                .await
                .expect("failed to set timeout for future")
        })?;

        println!("RESPONSE={:?}", response);
        Ok(())
    }

    pub fn deliver_tx(&mut self, tx: Vec<u8>) -> AbciResult<()> {
        let request = tonic::Request::new(protos::RequestDeliverTx { tx });

        let future = self.client.deliver_tx(request);
        let response = self.rt.block_on(async move {
            tokio::time::timeout(Duration::from_secs(1), future)
                .await
                .expect("failed to set timeout for future")
        })?;

        println!("RESPONSE={:?}", response);
        Ok(())
    }

    pub fn init_chain(&mut self, chain_id: String, app_state_bytes: Vec<u8>) -> AbciResult<()> {
        let request = tonic::Request::new(protos::RequestInitChain {
            chain_id: chain_id,
            app_state_bytes: app_state_bytes,
            time: None,
            validators: Vec::new(),
            consensus_params: None,
        });

        let future = self.client.init_chain(request);
        let response = self.rt.block_on(async move {
            tokio::time::timeout(Duration::from_secs(1), future)
                .await
                .expect("failed to set timeout for future")
        })?;

        println!("RESPONSE={:?}", response);
        Ok(())
    }

    pub fn begin_block(&mut self, hash: Vec<u8>) -> AbciResult<()> {
        let last_commit_info = protos::LastCommitInfo {
            round: 0,
            votes: Vec::new(),
        };

        let request = tonic::Request::new(protos::RequestBeginBlock {
            hash: hash,
            header: None,
            last_commit_info: Some(last_commit_info),
            byzantine_validators: Vec::new(),
        });

        let future = self.client.begin_block(request);
        let response = self.rt.block_on(async move {
            tokio::time::timeout(Duration::from_secs(1), future)
                .await
                .expect("failed to set timeout for future")
        })?;

        println!("RESPONSE={:?}", response);
        Ok(())
    }

    pub fn end_block(&mut self, height: i64) -> AbciResult<()> {
        let request = tonic::Request::new(protos::RequestEndBlock { height: height });

        let future = self.client.end_block(request);
        let response = self.rt.block_on(async move {
            tokio::time::timeout(Duration::from_secs(1), future)
                .await
                .expect("failed to set timeout for future")
        })?;

        println!("RESPONSE={:?}", response);
        Ok(())
    }

    pub fn commit(&mut self) -> AbciResult<()> {
        let request = tonic::Request::new(protos::RequestCommit {});

        let future = self.client.commit(request);
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
    fn test_abci_echo() {
        let result = connect_or_get_connection(DEFAULT_ABCI_URL)
            .unwrap()
            .echo("Hello there".to_owned());
        println!("echo result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_abci_deliver_tx() {
        let result = connect_or_get_connection(DEFAULT_ABCI_URL)
            .unwrap()
            .deliver_tx(Vec::new());
        println!("deliver_tx result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_abci_check_tx() {
        let result = connect_or_get_connection(DEFAULT_ABCI_URL)
            .unwrap()
            .check_tx(Vec::new(), 0);
        println!("check_tx result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_abci_init_chain() {
        let result = connect_or_get_connection(DEFAULT_ABCI_URL)
            .unwrap()
            .init_chain("chain_id".to_owned(), Vec::new());
        println!("init_chain result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_abci_begin_block() {
        let result = connect_or_get_connection(DEFAULT_ABCI_URL)
            .unwrap()
            .begin_block(Vec::new());
        println!("begin_block result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_abci_end_block() {
        let result = connect_or_get_connection(DEFAULT_ABCI_URL)
            .unwrap()
            .end_block(10);
        println!("end_block result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_abci_commit() {
        let result = connect_or_get_connection(DEFAULT_ABCI_URL)
            .unwrap()
            .commit();
        println!("commit result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }
}
