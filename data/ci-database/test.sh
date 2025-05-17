#!/bin/bash
# Run all tests for CI Database

set -e

echo "Building CI Database..."
cargo build --release

echo "Running unit tests..."
cargo test --lib

echo "Running integration tests..."
cargo test --test '*' -- --test-threads=1

echo "Running CLI tests..."
./target/release/ci-db db init
./target/release/ci-db agent register "TestAgent" --agent-type "test"
./target/release/ci-db agent list
./target/release/ci-db learning record "TestAgent" "test_trigger" "Test content"
./target/release/ci-db knowledge add "TestAgent" "test_type" "Test knowledge"
./target/release/ci-db knowledge search "Test"

echo "All tests passed!"