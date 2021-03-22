#!/bin/bash

set -euo pipefail

cleanup() {
    kill "$GRPC_PID"
}
trap cleanup EXIT

cd "$(dirname "$0")"

cargo build --bins

../../target/debug/simple-graphql-gateway --sdl >graphql/simple-graphql-gateway.graphql

../../target/debug/simple-grpc-server &
GRPC_PID=$!

sleep 2

echo "GRPC_PID=$GRPC_PID"

../../target/debug/simple-graphql-gateway
