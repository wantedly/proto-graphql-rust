# ドキュメント

- [概要](about.md)
- [アーキテクチャ](arci.md)
- [Examples](examples/01_chapter.md)
  - [例1: helloworld](examples/02_helloworld.md)
- [Protocol Buffers -> GraphQL スキーマ変換](schema-conversion/01_chapter.md)
  - [proto oneof -> GraphQL union](schema-conversion/02_union_oneof.md)
  - [GraphQLではInput objectでunionを使用できない](schema-conversion/03_union_input.md)
  - [GraphQLは空のオブジェクトとクエリをサポートしていない](schema-conversion/04_empty.md)
  - [mapなどの型はGraphQLではサポートされていない](schema-conversion/05_scalar.md)
  - [GraphQlでのnamespace](schema-conversion/06_namespace.md)
  <!-- - [gRPCのstreaming rpcに対応するgraphqlの機能](schema-conversion/07_streaming.md) -->
- [RustのGraphQLエコシステム](rust-graphql.md)
- [RustのgRPCエコシステム](rust-grpc.md)
