use std::{convert::Infallible, env, fs, net::SocketAddr};

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::{graphql, Response};
use tonic::transport::Channel;
use warp::Filter;

use federation::pb::products::{products_client::ProductsClient, products_graphql::ProductsQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([0, 0, 0, 0], 4003).into();
    println!("{} listening on {}", env!("CARGO_BIN_NAME"), addr);

    let grpc_client = ProductsClient::connect("http://localhost:4004").await?;

    let schema = Schema::build(
        ProductsQuery::<Channel>::default(),
        EmptyMutation,
        // Mutation(grpc_client.into(), AddtionalMutation),
        EmptySubscription,
    )
    .data(grpc_client)
    .enable_federation()
    .finish();

    fs::write(
        format!("graphql/{}.graphql", env!("CARGO_BIN_NAME")),
        schema.sdl(),
    )?;
    fs::write(
        format!("graphql/{}.federation.graphql", env!("CARGO_BIN_NAME")),
        schema.federation_sdl(),
    )?;

    warp::serve(graphql(schema).and_then(
        move |(schema, request): (Schema<_, _, _>, async_graphql::Request)| async move {
            let response = schema.execute(request).await;
            Ok::<_, Infallible>(Response::from(response))
        },
    ))
    .run(addr)
    .await;

    Ok(())
}
