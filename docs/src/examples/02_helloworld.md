# 例1: helloworld

この例では以下のシンプルなprotoをベースにサービスを作成する。

`proto/helloworld.proto`

```proto
syntax = "proto3";

package helloworld;

service Greeter {
  rpc SayHello (HelloRequest) returns (HelloReply) {}
}

message HelloRequest {
  string name = 1;
}

message HelloReply {
  string message = 1;
}
```

ビルドスクリプト(コンパイル時に実行されるプログラム) 。
コンパイル時にprotoファイルを読んでRustコードを生成する。

`build.rs`

```rust
fn main() {
    proto_graphql_build::compile_protos("proto/helloworld.proto").unwrap();
}
```

生成されるコード：<https://github.com/wantedly/bff-graphql-rust/blob/0ff22e938919b9a9b39c580956475bdd336639ed/examples/src/generated/helloworld.rs>

bffサーバ
`src/bin/bff-server.rs`

```rust
use std::{convert::Infallible, net::SocketAddr};

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_warp::{BadRequest, Response};
use warp::{http::Response as HttpResponse, Filter, Rejection};

use pb::{greeter_client::GreeterClient, greeter_graphql_schema::GreeterGraphQlSchema};

mod pb {
    // protoから生成されたRustコードをインポート
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();
    println!("bff-server listening on {}", addr);

    // gRPC clientに接続
    let grpc_client = GreeterClient::connect("http://[::1]:50051").await?;

    // gRPC clientからGraphQL schemaでラップする
    // 生成されたGraphQL schemaに含まれるmutationは対応するgRPCサービスのメソッドを呼び出すだけ
    let schema = grpc_client.into_graphql_schema();

    // .sdl()メソッドで実際のGraphQL schemaを取得できる
    println!("GraphQL schema:\n{}", schema.sdl());

    // GraphQLサービスの作成
    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (GreeterGraphQlSchema<_>, async_graphql::Request)| async move {
            // requestとresponseのデータはここで操作できる
            // 例：https://github.com/async-graphql/examples/blob/b36b5c44543b7323cb199ed229ea247e83b85d18/warp/token-from-header/src/main.rs#L37-L51
            // Request: https://docs.rs/async-graphql/2/async_graphql/struct.Request.html
            // Response: https://docs.rs/async-graphql/2/async_graphql/struct.Response.html
            Ok::<_, Infallible>(Response::from(schema.execute(request).await))
        },
    );

    // GraphQL playgroundで使用できるようにする
    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    // webサーバを実行
    let routes = graphql_playground.or(graphql_post);
    warp::serve(routes).run(addr).await;

    Ok(())
}
```

gRPCサーバ
`src/bin/grpc-server.rs`

```rust
use tonic::{transport::Server, Request, Response, Status};

use pb::greeter_server::{Greeter, GreeterServer};
use pb::{HelloReply, HelloRequest};

mod pb {
    // protoから生成されたRustコードをインポート
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
struct MyGreeter {}

// gRPCサーバの定義
#[tonic::async_trait]
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
async fn main() -> Result<()> {
    let addr = "[::1]:50052".parse().unwrap();
    println!("grpc-server listening on {}", addr);

    let greeter = MyGreeter::default();

    // gRPCサーバを実行
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
```

実行

```console
$ cargo run --bin grpc-server
grpc-server listening on [::1]:50052
```

```console
$ cargo run --bin bff-server
bff-server listening on [::1]:50051
graphql schema:
type NoopQuery {
        _noop: Empty!
}
type Empty {
        _noop: Boolean
}
type GreeterMutation {
        sayHello(request: HelloRequestInput!): HelloReply!
}
input HelloRequestInput {
name: String!
}
type HelloReply {
        message: String!
}
schema {
        query: NoopQuery
        mutation: GreeterMutation
}
```

(`.sdl()`メソッドが返すgraphql schemaで`input`がきちんとフォーマットされてないのは多分バグ)

GraphQL Playgroundまたは他のツールで`http://[::1]:50051`にmutationを投げる。

```graphql
mutation {
  sayHello(request:{name:"GraphQL"}){
    message
  }
}
```

```json
{
  "data": {
    "sayHello": {
      "message": "Hello GraphQL!"
    }
  }
}
```

```console
$ curl 'http://[::1]:50051' -H 'Accept-Encoding: gzip, deflate, br' -H 'Content-Type: application/json' -H 'Accept: application/json' -H 'Connection: keep-alive' -H 'DNT: 1' -H 'Origin: file://' --data-binary '{"query":"mutation {\n  sayHello(request:{name:\"o\"}){\n    message\n  }\n}"}' --compressed
{"data":{"sayHello":{"message":"Hello o!"}}}
```
