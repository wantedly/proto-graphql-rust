# Apollo Federation

## How to run

```sh
cd examples/simple
./start.sh
```

Then, open <http://localhost:4000> in browser.

[*Generated GraphQL schema*](graphql/graphql-gateway.graphql)

Query example:

```graphql
query {
  me {
    id
    username
    reviews {
      body
      product {
        name
      }
    }
  }
}
```

Result:

```json
{
  "data": {
    "me": {
      "id": "1234",
      "username": "Me",
      "reviews": [
        {
          "body": "A highly effective form of birth control.",
          "product": {
            "name": "Trilby"
          }
        },
        {
          "body": "Fedoras are one of the most fashionable hats around and can look great with a variety of outfits.",
          "product": {
            "name": "Trilby"
          }
        }
      ]
    }
  },
}
```
