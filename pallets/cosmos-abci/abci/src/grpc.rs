mod protos;

use protos::abci_application_client;
use std::{
    future::Future,
    time::{Duration, SystemTime},
};
use tokio::{runtime::Runtime, task::block_in_place};

type AbciClient = abci_application_client::AbciApplicationClient<tonic::transport::Channel>;

pub struct AbciinterfaceGrpc {
    rt: Runtime,
    client: AbciClient,
    chain_id: String,
}

impl AbciinterfaceGrpc {
    pub fn connect(abci_endpoint: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut rt = Runtime::new()?;
        let future = async {
            // Translates str into static str
            let endpoint: &'static str = Box::leak(abci_endpoint.into());
            AbciClient::connect(endpoint).await
        };
        let client = rt.block_on(future)?;
        Ok(AbciinterfaceGrpc {
            rt,
            client,
            chain_id: "default chain id".to_string(),
        })
    }
}

impl crate::ABCIInterface for AbciinterfaceGrpc {
    fn echo(&mut self, message: String) -> crate::AbciResult<dyn crate::ResponseEcho> {
        let request = tonic::Request::new(protos::RequestEcho { message });
        let future = self.client.echo(request);
        let response = wait(&self.rt, future)?;
        Ok(Box::new(response.into_inner()))
    }

    /// Type: 0 - New, 1 - Recheck
    fn check_tx(
        &mut self,
        tx: Vec<u8>,
        r#type: i32,
    ) -> crate::AbciResult<dyn crate::ResponseCheckTx> {
        let request = tonic::Request::new(protos::RequestCheckTx { tx, r#type });
        let future = self.client.check_tx(request);
        let response = wait(&self.rt, future)?;
        Ok(Box::new(response.into_inner()))
    }

    fn deliver_tx(&mut self, tx: Vec<u8>) -> crate::AbciResult<dyn crate::ResponseDeliverTx> {
        let request = tonic::Request::new(protos::RequestDeliverTx { tx });
        let future = self.client.deliver_tx(request);
        let response = wait(&self.rt, future)?;
        Ok(Box::new(response.into_inner()))
    }

    fn init_chain(
        &mut self,
        time_seconds: i64,
        time_nanos: i32,
        chain_id: &str,
        pub_key_types: Vec<String>,
        max_bytes: i64,
        max_gas: i64,
        max_age_num_blocks: i64,
        max_age_duration: u64,
        app_state_bytes: Vec<u8>,
    ) -> crate::AbciResult<dyn crate::ResponseInitChain> {
        //let time = SystemTime::now().into();
        let evidence = protos::EvidenceParams {
            max_age_num_blocks: max_age_num_blocks,
            max_age_duration: Some(Duration::from_micros(max_age_duration).into()),
        };
        let block = protos::BlockParams {
            max_bytes: max_bytes,
            max_gas: max_gas,
        };
        let validator = protos::ValidatorParams {
            pub_key_types: pub_key_types,
        };

        let consensus_params = protos::ConsensusParams {
            block: Some(block),
            evidence: Some(evidence),
            validator: Some(validator),
        };

        self.chain_id = chain_id.to_string();

        let request = tonic::Request::new(protos::RequestInitChain {
            time: Some(prost_types::Timestamp {
                seconds: time_seconds,
                nanos: time_nanos,
            }),
            chain_id: chain_id.to_owned(),
            consensus_params: Some(consensus_params),
            validators: vec![],
            app_state_bytes,
        });

        let future = self.client.init_chain(request);
        let response = wait(&self.rt, future)?;
        Ok(Box::new(response.into_inner()))
    }

    fn begin_block(
        &mut self,
        height: i64,
        hash: Vec<u8>,
        proposer_address: Vec<u8>,
    ) -> crate::AbciResult<dyn crate::ResponseBeginBlock> {
        let chain_id: String = self.chain_id.clone();

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
        Ok(Box::new(response.into_inner()))
    }

    fn end_block(&mut self, height: i64) -> crate::AbciResult<dyn crate::ResponseEndBlock> {
        let request = tonic::Request::new(protos::RequestEndBlock { height });
        let future = self.client.end_block(request);
        let response = wait(&self.rt, future)?;
        Ok(Box::new(response.into_inner()))
    }

    fn commit(&mut self) -> crate::AbciResult<dyn crate::ResponseCommit> {
        let request = tonic::Request::new(protos::RequestCommit {});
        let future = self.client.commit(request);
        let response = wait(&self.rt, future)?;
        Ok(Box::new(response.into_inner()))
    }

    fn query(
        &mut self,
        path: String,
        data: Vec<u8>,
        height: i64,
        prove: bool,
    ) -> crate::AbciResult<dyn crate::ResponseQuery> {
        let request = tonic::Request::new(protos::RequestQuery {
            path,
            data,
            height,
            prove,
        });
        let future = self.client.query(request);
        let response = wait(&self.rt, future)?;
        Ok(Box::new(response.into_inner()))
    }

    fn info(&mut self) -> crate::AbciResult<dyn crate::ResponseInfo> {
        let app_configs = crate::defaults::get_app_configs();
        let request = tonic::Request::new(protos::RequestInfo {
            p2p_version: app_configs.p2p_version,
            block_version: app_configs.block_version,
            version: app_configs.app_version,
        });
        let future = self.client.info(request);
        let response = wait(&self.rt, future)?;
        Ok(Box::new(response.into_inner()))
    }

    fn set_option(
        &mut self,
        key: &str,
        value: &str,
    ) -> crate::AbciResult<dyn crate::ResponseSetOption> {
        let request = tonic::Request::new(protos::RequestSetOption {
            key: key.to_string(),
            value: value.to_string(),
        });
        let future = self.client.set_option(request);
        let response = wait(&self.rt, future)?;
        Ok(Box::new(response.into_inner()))
    }
}

fn wait<F: Future>(rt: &Runtime, future: F) -> F::Output {
    let handle = rt.handle().clone();
    block_in_place(move || handle.block_on(future))
}
