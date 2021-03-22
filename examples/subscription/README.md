# Server Streaming RPC example

## How to run

```sh
cd examples/subscription
./start.sh
```

Open http://localhost:4000 in browser.

[Generated GraphQL schema](graphql/subscription-graphql-gateway.graphql)

Subscription example:

```graphql
subscription  {
  serverStreaming(request:{
    names:["Rust","ProtoBuf","GraphQL"]
  }){
    message
  }
}
```

Result:

```json
{
  "data": {
    "serverStreaming": {
      "message": "Hello Rust!"
    }
  }
}
{
  "data": {
    "serverStreaming": {
      "message": "Hello ProtoBuf!"
    }
  }
}
{
  "data": {
    "serverStreaming": {
      "message": "Hello GraphQL!"
    }
  }
}
```
