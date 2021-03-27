#![warn(rust_2018_idioms)]

use std::{env, net::SocketAddr};

use tonic::{async_trait, transport::Server, Request, Response, Status};

use pb::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

mod pb {
    include!("../generated/helloworld.rs");
}

#[derive(Default)]
struct MyGreeter {}

#[async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([0, 0, 0, 0], 4001).into();
    println!("{} listening on {}", env!("CARGO_BIN_NAME"), addr);

    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
