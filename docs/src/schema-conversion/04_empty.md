# GraphQLは空のオブジェクトとクエリをサポートしていない

## 問題

protoではからのメッセージとサービスはサポートされているが、GraphQLでは空のオブジェクトとクエリはサポートされていない。

proto:

```proto
message Empty {}
```

GraphQL:

```graphql
# ERROR!
type Empty {}
```

これも色々と議論されている。

- [graphql/graphql-spec#568](https://github.com/graphql/graphql-spec/issues/568)
- [graphql/graphql-js#937](https://github.com/graphql/graphql-js/issues/937)

実際には実装依存で、許可してる実装もあるよう。

- [graph-gophers/graphql-go#209](https://github.com/graph-gophers/graphql-go/issues/209)

RustのGraphQLサーバ実装はどちらも現在は許可していない。

- [async-graphql/async-graphql#315](https://github.com/async-graphql/async-graphql/issues/315)
- [graphql-rust/juniper#172](https://github.com/graphql-rust/juniper/issues/172)

## 本実装での解決策

これの主な回避策は [graphql/graphql-spec#568](https://github.com/graphql/graphql-spec/issues/568) と [graphql/graphql-js#937](https://github.com/graphql/graphql-js/issues/937) で言及されているように、Booleanのnoopフィールドを使用する方法で、今回の実装ではそれを採用した。例えば前述のprotoは次のGraphQL typeに変換される

```graphql
type Empty {
  _noop: Boolean
}
```

[wantedly-bff-graphqlも同じ方法を採用してるよう](https://github.com/wantedly/wantedly-graphql-gateway/blob/06be39beeafac58b44985140651c23bcccc08d4e/src/__generated__/schema.graphql#L1188)

クエリが一つも定義されていないケースも同様に許可されないのでこれについてもNoopQueryを定義して回避してる。

```graphql
type NoopQuery {
  _noop: Empty!
}
```
