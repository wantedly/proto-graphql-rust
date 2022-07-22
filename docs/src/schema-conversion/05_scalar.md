# mapなどの型はGraphQLではサポートされていない

## 問題

protoの[map](https://developers.google.com/protocol-buffers/docs/proto#maps)などの型はGraphQLではサポートされていない。

```proto
message Map {
  map<string, string> map = 1;
}
```

## 本実装での解決策

[Scalar types](https://graphql.org/learn/schema/#scalar-types)（input/ouputは文字列だがアプリケーション内では解析された型として使用できる型）を使用した。

```graphql
type Map {
  map: JSON!
}
"""
A scalar that can represent any JSON value.
"""
scalar JSON
```

async-graphqlにはjsonに(de)serialize可能な型をラップする便利な[`async_graphql::Json`](https://docs.rs/async-graphql/2.5.4/async_graphql/types/struct.Json.html)型があったのでそれを使用した。

juniperにも同じようなものがあるかは十分には調べてないが、2021-03-09時点ではまだなさそう: [graphql-rust/juniper#280](https://github.com/graphql-rust/juniper/issues/280)
