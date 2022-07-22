# RustのgRPCエコシステム

以下の4つがメジャー:

- [grpc-rs](https://github.com/tikv/grpc-rs) - The gRPC library for Rust built on C Core library and futures
- [grpc-rust](https://github.com/stepancheg/grpc-rust) - Rust implementation of gRPC
- [tower-grpc](https://github.com/tower-rs/tower-grpc) - A client and server gRPC implementation based on Tower
- [tonic](https://github.com/hyperium/tonic) - A native gRPC client & server implementation with async/await support

出典：<https://github.com/grpc-ecosystem/awesome-grpc#rust>

比較:

- tower-grpcはtonicの元の名前で、現在は非推奨
- grpc-rsはPingCAPによってメンテナンスされていて、tikvなどで使用されているが、以前にtikvのメンテナの人が「tower-grpcに切り替えたい」みたいなこと言ってた
  - <https://medium.com/@siddontang/use-tower-grpc-for-tikv-6109cf8c61>

    > - grpc-rs is not pure Rust
    > - Its performance is not very good. It is slower than our customized protocol.
    > - Sadly, we even met some panic in gRPC in production environments before.

- grpc-rustは個人のプロジェクトでかなり古くからある
  - メンテナが1人だけなので開発が止まりがちな印象
- tonicはtower-grpcの置き換えとして作られた
  - メインのメンテナはAWSの人で、他にもEmbark Studiosの人が何人か開発に関わってる
- tonicは[wantedly/refine-image-rust](https://github.com/wantedly/refine-image-rust)でも使用されている
