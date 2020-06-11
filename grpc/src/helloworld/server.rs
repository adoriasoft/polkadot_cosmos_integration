use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, EndMessage, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let msg = request.into_inner();

        println!("Got a request with name: {}", msg.name);
        let reply = HelloReply {
            message: format!("Hello {}!", msg.name),
        };

        Ok(Response::new(reply))
    }

    async fn end(
        &self,
        _request: Request<EndMessage>,
    ) -> Result<Response<EndMessage>, Status> {
        std::process::exit(0x0000);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("Server listening on :{}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}
