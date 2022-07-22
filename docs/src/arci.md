# アーキテクチャ

TODO: 箇条書きから文章へ変換

## proto-graphql-build

protoを読み込んで以下を生成

- GraphQLオブジェクトに対応するRustコード
  - これはprotoオブジェクトに対応するRustコード（後述）をベースに生成する
    - コード生成例

        次のようなprotoオブジェクトに対応するRustコードから

        ```rust
        /// Two RepositorySlug are considered as equal when both `user` and `name` are equal
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RepositorySlug {
            #[prost(string, tag = "1")]
            pub user: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub name: ::prost::alloc::string::String,
        }
        ```

        次のようなGraphQLオブジェクトに対応するRustコードを生成する

        ```rust
        /// Two RepositorySlug are considered as equal when both `user` and `name` are equal
        #[derive(
            Clone,
            PartialEq,
            ::async_graphql::SimpleObject,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        #[graphql(name = "RepositorySlug")]
        pub struct RepositorySlugGraphQl {
            pub user: ::prost::alloc::string::String,
            pub name: ::prost::alloc::string::String,
        }
        /// Two RepositorySlug are considered as equal when both `user` and `name` are equal
        #[derive(
            Clone,
            PartialEq,
            ::async_graphql::InputObject,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        #[graphql(name = "RepositorySlugInput")]
        pub struct RepositorySlugGraphQlInput {
            pub user: ::prost::alloc::string::String,
            pub name: ::prost::alloc::string::String,
        }
        ```

        これは次のGraphQLオブジェクトに対応する

        ```graphql
        type RepositorySlug {
            user: String!
            name: String!
        }
        input RepositorySlugInput {
            user: String!
            name: String!
        }
        ```

    - GraphQLオブジェクトに対応させるために必要な実装（(de)serializeなど）は`#[derive(::async_graphql::SimpleObject)]`および`#[derive(::async_graphql::InputObject)]`によって生成される
    - `#[graphql(name = "RepositorySlugInput")]`はGraphQLでの名前を指定する
      - Rustコード上では「protoオブジェクトに対応するRustコード」と「GraphQLオブジェクトに対応するRustコード」が同じ名前空間に生成されているためRustコード上での名前には「GraphQl」という名前が入る
      - Rustのモジュールやprotoのparentメッセージのようなものの対応物がGraphQlにはないので、いずれにしても名前を調整する必要がある
        - このparentメッセージをどうGraphQlオブジェクトの名前に変換するかは後述
    - GraphQlだと（Output）ObjectとInputObjectは別の型なので、rust上でもそのように表現される。
      - Inputとしても使用されるのかOutputのみなのかは基本的にはわからないので、今回の実装では常に両方を生成している
        - コメントを読んで「Output only」みたいなのがあればInputObjectは生成しないようにするようなコードを書くことは可能
        - どちらにしても、実際に使用されない場合はGraphQlスキーマには現れない

- protoオブジェクトとGraphQLオブジェクトの変換の実装
  - protoオブジェクトとGraphQLオブジェクトをどちらもRustのstruct・enumとして表現できるので、それを相互変換するコードを生成するだけ。
    - コード生成例

        ```rust
        impl From<RepositorySlug> for RepositorySlugGraphQl {
            fn from(other: RepositorySlug) -> Self {
                let RepositorySlug { user, name, .. } = other;
                Self {
                    user: user.into(),
                    name: name.into(),
                }
            }
        }
        impl From<RepositorySlugGraphQl> for RepositorySlug {
            fn from(other: RepositorySlugGraphQl) -> Self {
                let RepositorySlugGraphQl { user, name } = other;
                Self {
                    user: user.into(),
                    name: name.into(),
                }
            }
        }
        ```

    - Rustだとこの辺の変換はFromとIntoというtraitで大体表現できる
    - 失敗する可能性のある変換もあるので、TryFromとTryIntoを使うように書き換えた方がいいかも
    - mapとかoptionは単に`.into()`を呼びだすだけでは変換できないけど、この辺はフィールドの型を元に調整できる

- GraphQLクエリをgRPCリクエストに変換するGraphQL mutation
- これらはprost-buildという.protoファイルからRustコードを生成するためのライブラリを使用
  - ドキュメント：https://docs.rs/prost-build/0.7.0/prost_build/index.html
  - prost-buildが直接サポートしているのはprotoからprotoオブジェクトに対応するrustコードを生成することだけだが、ユーザ側で拡張できるように設計されている
    - 今回はこれのサービス生成機能を使用
    - ドキュメント：https://docs.rs/prost-build/0.7.0/prost_build/struct.Config.html#method.service_generator
  - 一部のコードはprost-buildに一度コードを生成させてからそれを編集、追加する形で生成している
- さらに以下を今回作ったライブラリで使用したツールによって同時に生成
  - protoオブジェクトに対応するrustコード
    - コード生成例
        例えば次のprotoメッセージから

        ```proto
        message ConditionFeedback {
          // Required.
          uint64 sender_user_id = 1;
          // Deprecated.  未回答フィードバックの実装に伴って廃止予定。
          // 今後はreceiver_user_id, company_id, beginning_of_weekをもとにConditionを特定する。
          uint64 condition_id = 2 [deprecated = true];
          // Optional.
          google.protobuf.StringValue message = 3;
          // Optional.
          google.protobuf.StringValue emoji = 4;
          // Required.
          uint64 receiver_user_id = 5;
          // Required.
          uint64 company_id = 6;
          // Required.
          google.type.Date beginning_of_week = 7;
        }
        ```

        次のRust構造体を生成

        ```rust
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ConditionFeedback {
            /// Required.
            #[prost(uint64, tag = "1")]
            pub sender_user_id: u64,
            /// Deprecated.  未回答フィードバックの実装に伴って廃止予定。
            /// 今後はreceiver_user_id, company_id, beginning_of_weekをもとにConditionを特定する。
            #[deprecated]
            #[prost(uint64, tag = "2")]
            pub condition_id: u64,
            /// Optional.
            #[prost(message, optional, tag = "3")]
            pub message: ::core::option::Option<::prost::alloc::string::String>,
            /// Optional.
            #[prost(message, optional, tag = "4")]
            pub emoji: ::core::option::Option<::prost::alloc::string::String>,
            /// Required.
            #[prost(uint64, tag = "5")]
            pub receiver_user_id: u64,
            /// Required.
            #[prost(uint64, tag = "6")]
            pub company_id: u64,
            /// Required.
            #[prost(message, optional, tag = "7")]
            pub beginning_of_week: ::core::option::Option<super::super::super::google::r#type::Date>,
        }
        ```

    - これはprost-buildによって生成される
    - proto messageへの(de)serializeに必要な実装は`#[derive(::prost::Message)]`によって生成される
    - Required、Optional、Deprecatedなどのコメントが解析され、型やアノテーションが調整される。
      - ただし、まだ微妙にバグがあるようで、例えば上の例の`beginning_of_week`はRequiredなのにOptionalとして生成されている。
        - 関連するバグレポート：https://github.com/danburkert/prost/issues/437
        - 今回作ったライブラリにprost-buildが生成したコードを解析して調整するステップがあるので、この辺はその一部として修正可能なはず
      - メッセージオプションはサポートされていない
        - 関連する機能リクエスト：https://github.com/danburkert/prost/issues/425
        - ただし、前述のように、prost-buildが生成したコードを解析して調整するステップがあるので、そこでコメントに基づいてカスタマイズすることは可能。
  - gRPCサービス（サーバおよびクライアント）
    - tonic-buildというprost-buildの同じ機能を使用する別のライブラリによって生成
    - tonic-buildはサービス生成部分を独立した関数としても使用できるように公開していて、それを今回作ったサービスジェネレータのサービス生成プロセス内で呼びだすという形で使用した
      - [tonic_build::client::generate](https://docs.rs/tonic-build/0.4.0/tonic_build/client/fn.generate.html)
      - [tonic_build::server::generate](https://docs.rs/tonic-build/0.4.0/tonic_build/server/fn.generate.html)
    - tonic-buildは[wantedly/apis-rust](https://github.com/wantedly/apis-rust)でも使用されている
