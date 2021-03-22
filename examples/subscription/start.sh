#!/bin/bash

set -euo pipefail

cleanup() {
    kill "$GRPC_PID"
}
trap cleanup EXIT

cd "$(dirname "$0")"

cargo build --bins

../../target/debug/subscription-graphql-gateway --sdl >graphql/subscription-graphql-gateway.graphql

../../target/debug/subscription-grpc-server &
GRPC_PID=$!

sleep 2

echo "GRPC_PID=$GRPC_PID"

../../target/debug/subscription-graphql-gateway
