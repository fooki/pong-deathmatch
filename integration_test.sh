#!/bin/bash

set -e
set -u

test_crash() {
    TEST_CRASHED="true"
    return 0
}

interrupt_cleanup() {
    set +u
    kill "$SERVER_PID" || true
    kill "$P1_PID" || true
    kill "$P2_PID" || true
}
trap 'interrupt_cleanup' SIGINT

echo "===================="
echo "Building the project"
echo "===================="

cargo build

echo "================================"
echo "Running a game for a few seconds"
echo "================================"

cargo run -- -s &
SERVER_PID="$!"

sleep 2

cargo run -- --cpu &
P1_PID="$!"

sleep 2

cargo run -- --cpu &
P2_PID="$!"

sleep 10

kill "$SERVER_PID" || test_crash
kill "$P1_PID" || test_crash
kill "$P2_PID" || test_crash

if test -z "${TEST_CRASHED-}"; then

    echo "========================"
    echo "SUCCESS, Nothing crashed"
    echo "========================"

    exit 0

else

    echo "========================"
    echo "Test failed"
    echo "========================"

    exit 1
fi
