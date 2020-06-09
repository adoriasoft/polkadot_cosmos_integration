use tonic::{transport::Server, Request, Response, Status};

use abci::abci_server::{Abci, AbciServer};
use abci::{CheckTxRequest, DeliverTxRequest, EndMessage, TxResponse};

pub mod abci {
    tonic::include_proto!("abci");
}

#[derive(Debug, Default)]
pub struct MyAbci {}

#[tonic::async_trait]
impl Abci for MyAbci {
    async fn check_tx(
        &self,
        request: Request<CheckTxRequest>,
    ) -> Result<Response<TxResponse>, Status> {
        let msg = request.into_inner();

        println!("Got a check_tx request: {:?}", msg);
        Ok(Response::new(get_tx_response(
            b"check_tx data".to_vec(),
            "check_tx log".to_owned(),
        )))
    }

    async fn deliver_tx(
        &self,
        request: Request<DeliverTxRequest>,
    ) -> Result<Response<TxResponse>, Status> {
        let msg = request.into_inner();

        println!("Got a deliver_tx request: {:?}", msg);
        Ok(Response::new(get_tx_response(
            b"deliver_tx data".to_vec(),
            "deliver_tx log".to_owned(),
        )))
    }

    async fn end(&self, _request: Request<EndMessage>) -> Result<Response<EndMessage>, Status> {
        std::process::exit(0x0000);
    }
}

fn get_tx_response(data: Vec<u8>, log: String) -> TxResponse {
    type TagsType = std::collections::HashMap<String, Vec<u8>>;
    TxResponse {
        code: 1000_u32,
        data,
        log,
        info: "info".to_owned(),
        gas_wanted: 1000_i64,
        gas_used: 1000_i64,
        tags: vec![("tag".to_owned(), b"...".to_vec())]
            .into_iter()
            .collect::<TagsType>(),
        codespace: "codespace".to_owned(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let my_abci = MyAbci::default();

    println!("Server listening on :{}", addr);

    Server::builder()
        .add_service(AbciServer::new(my_abci))
        .serve(addr)
        .await?;
    Ok(())
}
