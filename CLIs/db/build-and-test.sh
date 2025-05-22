#!/bin/bash
# Build and test DB CLI

set -e

echo "Building DB CLI..."
cargo build --release

echo "Running tests..."
cargo test

echo "Creating test database if it doesn't exist..."
mkdir -p ~/.ci-database

echo "Testing basic commands..."

# Test help
echo "Testing: db --help"
./target/release/db --help

echo ""
echo "Testing: db agent --help"
./target/release/db agent --help

echo ""
echo "Testing: db admin health"
./target/release/db admin health

echo ""
echo "Build and basic tests completed successfully!"
echo "To install system-wide, run: ./install.sh"