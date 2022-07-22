# Protocol Buffers -> GraphQL スキーマ変換

全体的に[既存のPOC](https://github.com/wantedly/wantedly-graphql-gateway/issues/23#issuecomment-769476148)を参考にした

protoの`message`と`enum`はほぼそのままGraphQLの`type`と`enum`に変換できる。

proto:

```proto
message Message {
  string value = 1;
}
enum Enum {
  A = 1;
  B = 2;
}
```

GraphQL:

```graphql
type Message {
  value: String!
}
enum Enum {
  A
  B
}
```

ややこしいのはprotoでサポートされているが、GraphQLでサポートされていないか、似ているが仕様が異なるようなオブジェクトであり、本章で説明する。
