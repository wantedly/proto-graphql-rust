#!/bin/bash

set -euo pipefail

function cleanup {
    kill "$GRPC_ACCOUNTS_PID"
    kill "$GRPC_PRODUCTS_PID"
    kill "$GRPC_REVIEWS_PID"
    kill "$GRAPHQL_ACCOUNTS_PID"
    kill "$GRAPHQL_PRODUCTS_PID"
    kill "$GRAPHQL_REVIEWS_PID"
}
trap cleanup EXIT

cd "$(dirname "$0")"

cargo build --bins

../../target/debug/grpc-accounts &
GRPC_ACCOUNTS_PID=$!

../../target/debug/grpc-products &
GRPC_PRODUCTS_PID=$!

../../target/debug/grpc-reviews &
GRPC_REVIEWS_PID=$!

sleep 2

../../target/debug/graphql-accounts &
GRAPHQL_ACCOUNTS_PID=$!

../../target/debug/graphql-products &
GRAPHQL_PRODUCTS_PID=$!

../../target/debug/graphql-reviews &
GRAPHQL_REVIEWS_PID=$!

sleep 4

echo "GRPC_ACCOUNTS_PID=$GRPC_ACCOUNTS_PID"
echo "GRPC_PRODUCTS_PID=$GRPC_PRODUCTS_PID"
echo "GRPC_REVIEWS_PID=$GRPC_REVIEWS_PID"
echo "GRAPHQL_ACCOUNTS_PID=$GRAPHQL_ACCOUNTS_PID"
echo "GRAPHQL_PRODUCTS_PID=$GRAPHQL_PRODUCTS_PID"
echo "GRAPHQL_REVIEWS_PID=$GRAPHQL_REVIEWS_PID"

(
    cd gateway
    node index.js
)
