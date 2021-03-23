use std::{env, net::SocketAddr};

use anyhow::Result;
use async_trait::async_trait;
use tonic::{transport::Server, Request, Response, Status};

use federation::pb::products::{
    products_server::{Products, ProductsServer},
    Product, ProductRequest, ProductResponse, ProductsResponse,
};

#[derive(Default)]
struct ProductsService {
    data: Vec<Product>,
}

#[async_trait]
impl Products for ProductsService {
    async fn top_products(
        &self,
        request: Request<()>,
    ) -> Result<Response<ProductsResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let reply = ProductsResponse {
            products: self.data.clone(),
        };
        Ok(Response::new(reply))
    }

    async fn find_product_by_upc(
        &self,
        request: tonic::Request<ProductRequest>,
    ) -> Result<tonic::Response<ProductResponse>, tonic::Status> {
        let upc = request.into_inner().upc;
        let reply = ProductResponse {
            product: self.data.iter().find(|product| product.upc == upc).cloned(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = ([0, 0, 0, 0], 4004).into();
    println!("{} listening on {}", env!("CARGO_BIN_NAME"), addr);

    let products = ProductsService {
        data: vec![
            Product {
                upc: "top-1".to_string(),
                name: "Trilby".to_string(),
                price: 11,
            },
            Product {
                upc: "top-2".to_string(),
                name: "Fedora".to_string(),
                price: 22,
            },
            Product {
                upc: "top-3".to_string(),
                name: "Boater".to_string(),
                price: 33,
            },
        ],
    };

    Server::builder()
        .add_service(ProductsServer::new(products))
        .serve(addr)
        .await?;

    Ok(())
}
