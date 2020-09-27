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

pub fn increment_on_initialize_variable() -> i64 {
    let mut value = ON_INITIALIZE_VARIABLE.lock().unwrap();
    if value.is_none() {
        *value = Some(0);
    }
    let temp = value.unwrap();
    *value = Some(temp + 1);
    value.unwrap()
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

pub fn set_chain_id(chain_id: &str) -> AbciResult<()> {
    let mut stored_chain_id = ABCI_CHAIN_ID.lock()?;
    *stored_chain_id = Some(chain_id.to_owned());
    Ok(())
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

    pub fn init_chain(&mut self, genesis: &str) -> AbciResult<protos::ResponseInitChain> {
        let genesis: serde_json::Value = serde_json::from_str(genesis).map_err(|e| e.to_string())?;
        let chain_id = genesis["chain_id"].as_str().ok_or("chain_id not found".to_owned())?;
        // let genesis_time = genesis["genesis_time"].as_str().ok_or("chain_id not found".to_owned())?;
        let pub_key_types: Vec<String> = genesis["consensus_params"]["validator"]["pub_key_types"]
            .as_array()
            .ok_or("pub_keys_types not found".to_owned())?
            .into_iter()
            .map(|v| v.as_str().unwrap().to_owned())
            .collect();
        let max_bytes = genesis["consensus_params"]["block"]["max_bytes"]
            .as_str()
            .ok_or("chain_id not found".to_owned())?
            .parse::<i64>()?;
        let max_gas = genesis["consensus_params"]["block"]["max_gas"]
            .as_str()
            .ok_or("chain_id not found".to_owned())?
            .parse::<i64>()?;
        let max_age_num_blocks = genesis["consensus_params"]["evidence"]["max_age_num_blocks"]
            .as_str()
            .ok_or("chain_id not found".to_owned())?
            .parse::<i64>()?;
        let max_age_duration = genesis["consensus_params"]["evidence"]["max_age_duration"]
            .as_str()
            .ok_or("chain_id not found".to_owned())?
            .parse::<u64>()?;
        let app_state_bytes = genesis["app_state"].to_string().as_bytes().to_vec();
        // Sets chain_id for future begin_block calls
        set_chain_id(chain_id)?;
        let request = tonic::Request::new(protos::RequestInitChain {
            time: Some(SystemTime::now().into()),
            chain_id: chain_id.to_owned(),
            consensus_params: Some(protos::ConsensusParams {
                block: Some(protos::BlockParams { max_bytes, max_gas }),
                evidence: Some(protos::EvidenceParams {
                    max_age_num_blocks,
                    max_age_duration: Some(Duration::from_micros(max_age_duration).into()),
                }),
                validator: Some(protos::ValidatorParams { pub_key_types }),
            }),
            validators: vec![],
            app_state_bytes,
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
        let chain_id: String = get_chain_id()?.to_string();
        let request = tonic::Request::new(protos::RequestBeginBlock {
            hash,
            header: Some(protos::Header {
                version: None,
                chain_id,
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
        println!("{:?}", response);
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
mod tests {
    use super::*;

    #[test]
    fn should_set_and_get_chain_id() {
        assert!(
            set_chain_id("default-chain-id").is_ok(),
            "should set chain id"
        );

        assert_eq!(
            get_chain_id().unwrap().as_str(),
            "default-chain-id".to_string()
        );
    }

    #[test]
    fn should_init_variable_and_increment_variable() {
        assert_eq!(increment_on_initialize_variable(), 1);

        assert_eq!(get_on_initialize_variable(), 1);
    }
}
