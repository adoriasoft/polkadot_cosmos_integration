mod defaults;
pub mod protos;

pub use defaults::*;

use lazy_static::lazy_static;
use owning_ref::MutexGuardRefMut;
use protos::abci_application_client;
use std::{
    future::Future,
    sync::Mutex,
    time::{Duration, SystemTime},
};
use tokio::{runtime::Runtime, task::block_in_place};

lazy_static! {
    static ref ABCI_CLIENT: Mutex<Option<Client>> = Mutex::new(None);
    static ref ABCI_CHAIN_ID: Mutex<Option<String>> = Mutex::new(None);
}

// TODO: find better solution for the assync problem https://adoriasoft.atlassian.net/browse/PCI-108
// ----
lazy_static! {
    static ref ON_INITIALIZE_VARIABLE: Mutex<Option<i64>> = Mutex::new(None);
}

pub fn get_on_initialize_variable() -> i64 {
    let mut value = ON_INITIALIZE_VARIABLE.lock().unwrap();
    if value.is_none() {
        *value = Some(0);
    }
    let res = *value;
    return res.unwrap();
}

pub fn increment_on_initialize_variable() {
    let mut value = ON_INITIALIZE_VARIABLE.lock().unwrap();
    if value.is_none() {
        *value = Some(0);
    }
    let temp = value.unwrap();
    *value = Some(temp + 1)
}

// ----

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

pub fn defines_chain_id(chain_id: String) {
    let mut stored_chain_id = ABCI_CHAIN_ID.lock().unwrap();
    *stored_chain_id = Some(chain_id);
}

pub fn get_chain_id<'ret>() -> AbciResult<MutexGuardRefMut<'ret, Option<String>, String>> {
    let mut chain_id = ABCI_CHAIN_ID.lock()?;
    if chain_id.is_none() {
        *chain_id = Some("default-chain-id".to_string());
    }
    // Here we create a ref to the inner value of the mutex guard.
    // Unwrap should never panic as we set it previously.
    let res = MutexGuardRefMut::new(chain_id).map_mut(|mg| mg.as_mut().unwrap());
    Ok(res)
}

impl Client {
    pub fn connect(abci_endpoint: &str) -> AbciResult<Self> {
        let mut rt = Runtime::new()?;
        let future = async {
            // Translates str into static str
            let endpoint: &'static str = Box::leak(abci_endpoint.into());
            AbciClient::connect(endpoint).await
        };
        let client = rt.block_on(future)?;
        Ok(Client { rt, client })
    }

    pub fn echo(&mut self, message: String) -> AbciResult<protos::ResponseEcho> {
        let request = tonic::Request::new(protos::RequestEcho { message });
        let future = self.client.echo(request);
        let response = wait(&self.rt, future)?;
        Ok(response.into_inner())
    }

    /// Type: 0 - New, 1 - Recheck
    pub fn check_tx(&mut self, tx: Vec<u8>, r#type: i32) -> AbciResult<protos::ResponseCheckTx> {
        let request = tonic::Request::new(protos::RequestCheckTx { tx, r#type });
        let future = self.client.check_tx(request);
        let response = wait(&self.rt, future)?;
        Ok(response.into_inner())
    }

    pub fn deliver_tx(&mut self, tx: Vec<u8>) -> AbciResult<protos::ResponseDeliverTx> {
        let request = tonic::Request::new(protos::RequestDeliverTx { tx });
        let future = self.client.deliver_tx(request);
        let response = wait(&self.rt, future)?;
        Ok(response.into_inner())
    }

    // You should define ABCI_CHAIN_ID (defines_chain_id() function) variable before execute this function
    pub fn init_chain(
        &mut self,
        app_state_bytes: Vec<u8>,
        max_bytes: i64,
        max_gas: i64,
        max_age_num_blocks: i64,
        max_age_duration: u64,
        pub_key_types: Vec<String>,
    ) -> AbciResult<protos::ResponseInitChain> {
        let request = tonic::Request::new(protos::RequestInitChain {
            time: Some(SystemTime::now().into()),
            chain_id: get_chain_id().unwrap().to_string(),
            consensus_params: Some(protos::ConsensusParams {
                block: Some(protos::BlockParams {
                    max_bytes: max_bytes,
                    max_gas: max_gas,
                }),
                evidence: Some(protos::EvidenceParams {
                    max_age_num_blocks: max_age_num_blocks,
                    max_age_duration: Some(Duration::from_micros(max_age_duration).into()),
                }),
                validator: Some(protos::ValidatorParams {
                    pub_key_types: pub_key_types,
                }),
            }),
            validators: vec![],
            app_state_bytes: app_state_bytes,
        });
        let future = self.client.init_chain(request);
        let response = wait(&self.rt, future)?;
        Ok(response.into_inner())
    }

    pub fn begin_block(
        &mut self,
        height: i64,
        hash: Vec<u8>,
        proposer_address: Vec<u8>,
    ) -> AbciResult<protos::ResponseBeginBlock> {
        let chain_id: String = get_chain_id().unwrap().to_string();
        let request = tonic::Request::new(protos::RequestBeginBlock {
            hash,
            header: Some(protos::Header {
                version: None,
                chain_id: chain_id,
                height,
                time: Some(SystemTime::now().into()),
                last_block_id: None,
                last_commit_hash: vec![],
                data_hash: vec![],
                validators_hash: vec![],
                next_validators_hash: vec![],
                consensus_hash: vec![],
                app_hash: vec![],
                last_results_hash: vec![],
                evidence_hash: vec![],
                proposer_address,
            }),
            last_commit_info: None,
            byzantine_validators: vec![],
        });
        let future = self.client.begin_block(request);
        let response = wait(&self.rt, future)?;
        Ok(response.into_inner())
    }

    pub fn end_block(&mut self, height: i64) -> AbciResult<protos::ResponseEndBlock> {
        let request = tonic::Request::new(protos::RequestEndBlock { height });
        let future = self.client.end_block(request);
        let response = wait(&self.rt, future)?;
        Ok(response.into_inner())
    }

    pub fn commit(&mut self) -> AbciResult<protos::ResponseCommit> {
        let request = tonic::Request::new(protos::RequestCommit {});
        let future = self.client.commit(request);
        let response = wait(&self.rt, future)?;
        Ok(response.into_inner())
    }

    pub fn query(
        &mut self,
        path: String,
        data: Vec<u8>,
        height: i64,
        prove: bool,
    ) -> AbciResult<protos::ResponseQuery> {
        let request = tonic::Request::new(protos::RequestQuery {
            path,
            data,
            height,
            prove,
        });
        let future = self.client.query(request);
        let response = wait(&self.rt, future)?;
        Ok(response.into_inner())
    }
}

fn wait<F: Future>(rt: &Runtime, future: F) -> F::Output {
    let handle = rt.handle().clone();
    block_in_place(move || handle.block_on(future))
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
            .deliver_tx(vec![]);
        println!("deliver_tx result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_abci_check_tx() {
        let result = connect_or_get_connection(DEFAULT_ABCI_URL)
            .unwrap()
            .check_tx(vec![], 0);
        println!("check_tx result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_abci_query() {
        let result = connect_or_get_connection(DEFAULT_ABCI_URL).unwrap().query(
            "/a/b/c".to_owned(),
            "IHAVENOIDEA".as_bytes().to_vec(),
            0,
            false,
        );
        println!("query result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    // TODO: fix test
    // #[test]
    // fn test_abci_begin_block() {
    //     let height = 1;
    //     let result = connect_or_get_connection(DEFAULT_ABCI_URL)
    //         .unwrap()
    //         .init_chain(
    //             "test-chain-id".to_owned(),
    //             DEFAULT_ABCI_APP_STATE.as_bytes().to_vec(),
    //         );
    //     println!("init_chain result: {:?}", result);
    //     assert_eq!(result.is_ok(), true);
    //     let result = connect_or_get_connection(DEFAULT_ABCI_URL)
    //         .unwrap()
    //         .begin_block(
    //             "test-chain-id".to_owned(),
    //             height,
    //             vec![],
    //             vec![],
    //         );
    //     println!("begin_block result: {:?}", result);
    //     assert_eq!(result.is_ok(), true);
    //     let result = connect_or_get_connection(DEFAULT_ABCI_URL)
    //         .unwrap()
    //         .end_block(height);
    //     println!("end_block result: {:?}", result);
    //     assert_eq!(result.is_ok(), true);
    //     let result = connect_or_get_connection(DEFAULT_ABCI_URL)
    //         .unwrap()
    //         .commit();
    //     println!("commit result: {:?}", result);
    //     assert_eq!(result.is_ok(), true);
    // }
}
