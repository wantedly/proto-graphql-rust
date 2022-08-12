#![warn(rust_2018_idioms)]

pub mod pb {
    #![allow(clippy::derive_partial_eq_without_eq)]
    pub mod accounts {
        include!("generated/federation.accounts.rs");
    }
    pub mod products {
        include!("generated/federation.products.rs");
    }
    pub mod reviews {
        include!("generated/federation.reviews.rs");
    }
}

use async_graphql::ComplexObject;
use pb::reviews;

#[ComplexObject]
impl reviews::UserGraphQl {
    async fn reviews(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<Vec<reviews::ReviewGraphQl>> {
        use reviews::*;
        let mut grpc_client = ctx
            .data::<reviews_client::ReviewsClient<tonic::transport::Channel>>()?
            .clone();
        let response = grpc_client
            .reviews_by_id(reviews::UserRequest {
                id: self.id.clone(),
            })
            .await
            .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
        let response = response
            .into_inner()
            .reviews
            .into_iter()
            .map(ReviewGraphQl::from)
            .collect();
        Ok(response)
    }
}

#[ComplexObject]
impl reviews::ProductGraphQl {
    async fn reviews(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<Vec<reviews::ReviewGraphQl>> {
        use reviews::*;
        let mut grpc_client = ctx
            .data::<reviews_client::ReviewsClient<tonic::transport::Channel>>()?
            .clone();
        let response = grpc_client
            .reviews_by_upc(reviews::ProductRequest {
                upc: self.upc.clone(),
            })
            .await
            .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
        let response = response
            .into_inner()
            .reviews
            .into_iter()
            .map(ReviewGraphQl::from)
            .collect();
        Ok(response)
    }
}
