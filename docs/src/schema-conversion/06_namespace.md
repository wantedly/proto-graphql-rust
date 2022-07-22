# GraphQlでのnamespace

## 問題

Rustのモジュールやprotoのparentメッセージのようなものの対応物がGraphQlにはない。

そのため、GraphQlオブジェクトの名前を調整して名前空間を表現する必要がある。

## 本実装での解決策

今回の実装では単一のprotoファイルをルートとして扱って、そこからのparentメッセージをRoot側から順に全て名前のprefixとして追加してある。

この方法では、例えば次のprotoのInnerメッセージは「ParentInner」になる。

```proto
message Parent {
  string value = 1;
  message Inner {
    string value = 1;
  }
}
```

```graphql
type Parent {
  value: String!
}
type ParentInner {
  value: String!
}
```

単一のprotoファイルではなく、パッケージ名を解析すればもっと良いかもしれない。
