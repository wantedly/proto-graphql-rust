# proto oneof -> GraphQL union

## 問題

GraphQLのunionはprotoのoneofと似ているが、GraphQL unionのフィールドの型は他のフィールドと異なる型である必要がある。

例えば以下のproto oneofはGraphQL unionにそのまま変換できない

```proto
oneof OneOf {
  string A = 1;
  string B = 2;
}
```

```graphql
# ERROR!
union OneOf = String | String
```

## 本実装での解決策

今回の実装では個々のフィールドに対して新しいtypeを定義することで回避した

```graphql
union OneOf = OneOfA | OneOfB
type OneOfA {
  a: String!
}
type OneOfB {
  b: String!
}
```

元のフィールド名が生成されたtypeのsuffixとそのフィールド名として残ってる。
この辺のnamingはもっといいのがありそう。

また、unionのフィールドにオブジェクト以外を置けないという問題もあり、それも同様の方法で対処した。
