#!/bin/bash
# Initialize CI Database (Rust version)

set -e

echo "Building CI Database..."
cargo build --release

echo "Initializing databases..."
./target/release/ci-db db init

echo "CI Database initialized successfully!"
echo ""
echo "To use the CLI:"
echo "  ./target/release/ci-db --help"
echo ""
echo "Or add to PATH:"
echo "  export PATH=\"\$PATH:$(pwd)/target/release\""
echo "  ci-db --help"