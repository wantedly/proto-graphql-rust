#![warn(rust_2018_idioms)]

fn main() {
    let config = proto_graphql_build::Config::new();
    proto_graphql_build::configure()
        .out_dir("src/generated")
        .federation(true)
        .compile_with_config(
            config,
            &[
                "proto/accounts.proto",
                "proto/products.proto",
                "proto/reviews.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
