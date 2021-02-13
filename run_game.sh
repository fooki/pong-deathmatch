#!/bin/bash

set -e
set -u

cleanup() {
    kill "$SERVER_PID" || true
    kill "$P1_PID" || true
    kill "$P2_PID" || true
}

trap 'cleanup' SIGINT

cargo run -- -s &
SERVER_PID=$!

sleep 2

cargo run -- --cpu &
P1_PID=$!

sleep 2

cargo run -- --cpu &
P2_PID=$!

sleep 600
cleanup
