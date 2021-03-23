use std::{env, net::SocketAddr};

use anyhow::Result;
use async_trait::async_trait;
use tonic::{transport::Server, Request, Response, Status};

use federation::pb::reviews::{
    reviews_server::{Reviews, ReviewsServer},
    Product, ProductRequest, Review, ReviewResponse, User, UserRequest,
};

#[derive(Default)]
struct ReviewsService {
    data: Vec<Review>,
}

#[async_trait]
impl Reviews for ReviewsService {
    async fn reviews(&self, request: Request<()>) -> Result<Response<ReviewResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let reply = ReviewResponse {
            reviews: self.data.clone(),
        };
        Ok(Response::new(reply))
    }

    async fn find_user_by_id(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<User>, Status> {
        let id = request.into_inner().id;
        let reply = User { id };
        Ok(Response::new(reply))
    }

    async fn find_product_by_upc(
        &self,
        request: Request<ProductRequest>,
    ) -> Result<Response<Product>, Status> {
        let upc = request.into_inner().upc;
        let reply = Product { upc };
        Ok(Response::new(reply))
    }

    async fn reviews_by_id(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<ReviewResponse>, Status> {
        let id = request.into_inner().id;
        let data = self
            .data
            .iter()
            .filter(|review| {
                review
                    .author
                    .as_ref()
                    .map_or(false, |author| author.id == id)
            })
            .cloned()
            .collect();
        let reply = ReviewResponse { reviews: data };
        Ok(Response::new(reply))
    }

    async fn reviews_by_upc(
        &self,
        request: Request<ProductRequest>,
    ) -> Result<Response<ReviewResponse>, Status> {
        let upc = request.into_inner().upc;
        let data = self
            .data
            .iter()
            .filter(|review| {
                review
                    .product
                    .as_ref()
                    .map_or(false, |product| product.upc == upc)
            })
            .cloned()
            .collect();
        let reply = ReviewResponse { reviews: data };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = ([0, 0, 0, 0], 4006).into();
    println!("{} listening on {}", env!("CARGO_BIN_NAME"), addr);

    let reviews = ReviewsService {
        data: vec![
            Review {
                body: "A highly effective form of birth control.".into(),
                author: User { id: "1234".into() }.into(),
                product: Product {
                    upc: "top-1".to_string(),
                }
                .into(),
            },
            Review {
                body: "Fedoras are one of the most fashionable hats around and can look \
                       great with a variety of outfits."
                    .into(),
                author: User { id: "1234".into() }.into(),
                product: Product {
                    upc: "top-1".to_string(),
                }
                .into(),
            },
            Review {
                body: "This is the last straw. Hat you will wear. 11/10".into(),
                author: User { id: "7777".into() }.into(),
                product: Product {
                    upc: "top-1".to_string(),
                }
                .into(),
            },
        ],
    };

    Server::builder()
        .add_service(ReviewsServer::new(reviews))
        .serve(addr)
        .await?;

    Ok(())
}
