use std::{env, net::SocketAddr, pin::Pin};

use anyhow::Result;
use async_trait::async_trait;
use futures::stream::Stream;
use tokio::time::{self, Duration};
use tonic::{transport::Server, Request, Response, Status};

use pb::{
    subscription_server::{Subscription, SubscriptionServer},
    HelloReply, HelloRequest,
};

mod pb {
    include!("../generated/subscription.rs");
}

#[derive(Default)]
struct MySubscription {}

#[async_trait]
impl Subscription for MySubscription {
    type ServerStreamingStream =
        Pin<Box<dyn Stream<Item = Result<HelloReply, tonic::Status>> + Send + Sync + 'static>>;

    async fn server_streaming(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<Self::ServerStreamingStream>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        Ok(Response::new(Box::pin(async_stream::stream! {
            let request = request.into_inner();
            let mut interval = request.interval.unwrap_or(1.0);
            let duration = Duration::from_secs_f32(request.interval.unwrap_or(1.0) as _);
            for name in request.names {
                time::sleep(duration).await;
                println!("Reply to {} ({}s)", name, interval);
                interval += duration.as_secs_f32();
                yield Ok(HelloReply {
                    message: format!("Hello {}!", name),
                });
            }
        })))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = ([0, 0, 0, 0], 4001).into();
    println!("{} listening on {}", env!("CARGO_BIN_NAME"), addr);

    let greeter = MySubscription::default();

    Server::builder()
        .add_service(SubscriptionServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
