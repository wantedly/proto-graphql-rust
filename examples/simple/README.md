# Simple graphql-gateway

## How to run

```sh
cd examples/simple
./start.sh
```

Then, open <http://localhost:4000> in browser.

[*Generated GraphQL schema*](graphql/simple-graphql-gateway.graphql)

Query example:

```graphql
query {
  sayHello(request:{name:"GraphQL"}){
    message
  }
}
```

Result:

```json
{
  "data": {
    "serverStreaming": {
      "message": "Hello GraphQL!"
    }
  }
}
```
