# gRPCのstreaming rpcに対応するGraphQLの機能

## 問題

gRPCにはリクエスト・レスポンスが1:1になるUnary通信だけでなく、レスポンスまたはリクエスト、または両方を複数送信・返却できるStreaming通信もサポートしている。（[公式ドキュメント](https://grpc.io/docs/what-is-grpc/core-concepts）

- Unary RPC (リクエスト・レスポンスが1:1)

  ```proto
  service Service {
    rpc Unary (Request) returns (stream Response);
  }
  ```

- Server Streaming RPC (リクエスト・レスポンスが1:多)

  ```proto
  service Service {
    rpc ServerStreaming (Request) returns (stream Response);
  }
  ```

- Client Streaming RPC (リクエスト・レスポンスが多:1)

  ```proto
  service Service {
    rpc ClientStreaming (stream Request) returns (Response);
  }
  ```

- Bidirectional Streaming RPC (リクエスト・レスポンスが多:多)

  ```proto
  service Service {
    rpc BidirectionalStreaming (stream Request) returns (stream Response);
  }
  ```

## 本実装での解決策

未解決
