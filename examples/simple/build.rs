#![warn(rust_2018_idioms)]

fn main() {
    let mut config = proto_graphql_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");
    proto_graphql_build::configure()
        .out_dir("src/generated")
        .compile_with_config(config, &["proto/helloworld.proto"], &["proto"])
        .unwrap();
}
