use std::{convert::Infallible, env, net::SocketAddr};

use anyhow::Result;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::{graphql, Response};
use tonic::transport::Channel;
use warp::Filter;

use federation::pb::reviews::{reviews_client::ReviewsClient, reviews_graphql::ReviewsQuery};

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = ([0, 0, 0, 0], 4005).into();
    println!("{} listening on {}", env!("CARGO_BIN_NAME"), addr);

    let grpc_client = ReviewsClient::connect("http://localhost:4006").await?;

    let schema = Schema::build(
        ReviewsQuery::<Channel>::default(),
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
