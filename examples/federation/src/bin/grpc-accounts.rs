use std::{env, net::SocketAddr};

use tonic::{async_trait, transport::Server, Request, Response, Status};

use federation::pb::accounts::{
    accounts_server::{Accounts, AccountsServer},
    User, UserRequest, UserResponse,
};

#[derive(Default)]
struct AccountsService {}

#[async_trait]
impl Accounts for AccountsService {
    async fn me(&self, request: Request<()>) -> Result<Response<User>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let reply = User {
            id: "1234".into(),
            username: "Me".to_string(),
        };
        Ok(Response::new(reply))
    }

    async fn find_user_by_id(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let id = request.into_inner().id;
        let username = if id == "1234" {
            "Me".to_string()
        } else {
            format!("User {:?}", id)
        };
        let reply = UserResponse {
            user: Some(User { id, username }),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([0, 0, 0, 0], 4002).into();
    println!("{} listening on {}", env!("CARGO_BIN_NAME"), addr);

    let accounts = AccountsService::default();

    Server::builder()
        .add_service(AccountsServer::new(accounts))
        .serve(addr)
        .await?;

    Ok(())
}
