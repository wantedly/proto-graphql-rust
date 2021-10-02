#![warn(rust_2018_idioms)]

fn main() {
    let config = proto_graphql_build::Config::new();
    proto_graphql_build::configure()
        .out_dir("src/generated")
        .compile_with_config(config, &["proto/subscription.proto"], &["proto"])
        .unwrap();
}
