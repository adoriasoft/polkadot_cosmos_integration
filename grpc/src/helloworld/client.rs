use hello_world::greeter_client::GreeterClient;
use hello_world::{HelloRequest, EndMessage};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });
    let response = client.say_hello(request).await?;
    println!("RESPONSE: {:?}", response.into_inner().message);

    let end_request = tonic::Request::new(EndMessage {});
    client.end(end_request).await.ok();
    Ok(())
}
