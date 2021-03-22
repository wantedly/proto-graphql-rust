pub mod pb {
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
impl reviews::User {
    async fn reviews(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<Vec<reviews::Review>> {
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
        Ok(response.into_inner().reviews)
    }
}

#[ComplexObject]
impl reviews::Product {
    async fn reviews(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<Vec<reviews::Review>> {
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
        Ok(response.into_inner().reviews)
    }
}
