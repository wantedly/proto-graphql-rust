
# RustのGraphQLエコシステム

[juniper](https://crates.io/crates/juniper)と[async-graphql](https://crates.io/crates/async-graphql)の2つのGraphQLサーバライブラリがメジャー。

今回の検証ではasync-graphqlを選択した。

## 概要と比較

- juniperはより長い歴史があり広く使用されている。
- async-graphqlは新しいがより積極的にメンテナンスされているように見える。
- どちらのライブラリもRust code firstで、スキーマからrustコードを生成するのではなく、rustコードからスキーマを組み立てるような形になる
  - juniperには[GraphQLスキーマから対応するrustコードを生成できるライブラリ](https://github.com/davidpdrsn/juniper-from-schema)があるが、最新のjuniperのサポートはまだリリースされていない（masterにはある）
- [古いバージョンのjuniperはRust構文とはかなり異なるDSLを書く必要があった](https://docs.rs/juniper/0.14.2/juniper/macro.graphql_object.html#examples)ようだが、最新バージョンでは必要なくなったようなので、少しAPIを見た感じではそれほどasync-graphqlとjuniperの間に基本的な使いやすさの違いはないかもしれない（juniperを実際に触ったわけではないので詳細はわからない）

## 今回の検証でasync-graphqlを選択した理由

今回の検証ではasync-graphqlを選択した。理由は主に以下:

- juniperがtokio（rustのもっとも広く使用される非同期ランタイム）の最新バージョンに未対応であった
- juniperのいくつかのマクロ関連のバグに遭遇した（おそらく簡単なバグなので、時間があれば修正を提出する予定）
- async-graphqlの主要なメンテナは2人ともGitHub/Rustコミュニティ上で非常にアクティブであり、問題があった場合に何らかの応答が得られる可能性が高いと考えた
