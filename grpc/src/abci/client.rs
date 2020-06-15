use abci::abci_client::AbciClient;
use abci::{CheckTxRequest, DeliverTxRequest, CheckTxType, EndMessage};

pub mod abci {
    tonic::include_proto!("abci");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AbciClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(CheckTxRequest {
        tx: b"...".to_vec(),
        req_type: CheckTxType::CheckTxNew as i32,
    });
    let response = client.check_tx(request).await?;
    println!("Check tx log: {:?}", response.into_inner().log);

    let request = tonic::Request::new(DeliverTxRequest {
        tx: b"...".to_vec(),
    });
    let response = client.deliver_tx(request).await?;
    println!("Deliver tx log: {:?}", response.into_inner().log);

    let end_request = tonic::Request::new(EndMessage {});
    client.end(end_request).await.ok();
    Ok(())
}
