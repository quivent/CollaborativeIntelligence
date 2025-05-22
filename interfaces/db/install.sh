#!/bin/bash
# Install DB CLI

set -e

echo "Building DB CLI..."
cargo build --release

echo "Installing to /usr/local/bin..."
sudo cp target/release/db /usr/local/bin/db

echo "Setting permissions..."
sudo chmod +x /usr/local/bin/db

echo "DB CLI installed successfully!"
echo ""
echo "You can now use 'db' command from anywhere:"
echo "  db agent list"
echo "  db admin health"
echo "  db --help"