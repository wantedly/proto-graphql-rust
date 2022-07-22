# GraphQLではInput objectでunionを使用できない

## 問題

proto oneofはリクエストとレスポンスの両方で使用できるがGraphQL unionはレスポンス(output)のみ。

これについては公式リポジトリを含め色々と議論されているよう。

- [graphql/graphql-spec#488](https://github.com/graphql/graphql-spec/issues/488)
- <https://brunoscheufler.com/blog/2019-05-19-reaching-consensus-graphql-input-union>

(私が読み間違えていなければ、受け入れられたけど実装はされていない、という状況。)

## 本実装での解決策

今回の実装では[async-graphqlのメンテナがこの制限の回避策として使用していると述べた方法](https://github.com/async-graphql/async-graphql/issues/373#issuecomment-753761917)を採用した。

この方法では、以下のunionに対し

```graphql
union Union = A | B
```

以下のinput typeを生成する

```graphql
input UnionInput {
  A: A
  B: B
}
```

これは [graphql/graphql-spec#488](https://github.com/graphql/graphql-spec/issues/488) で「Directive」と呼ばれている方法と基本的に同じ。

この方法にはフィールドの型が重複するunionをサポートできるという利点があるが、スキーマでの表現（複数のオプションフィールドをもつtype）と実際の処理（union）が異なるなどの欠点がある。

また、実際にオブジェクト変換を実装する際に（複数のフィールドが同時に指定されていないか）チェックが必要。
