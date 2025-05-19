#!/bin/bash

# Script to build the agent_cache_system.rs file

# Ensure we have rustc installed
if ! command -v rustc &> /dev/null; then
    echo "Error: rustc is not installed or not in PATH"
    echo "Please install Rust using: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Create a temporary Cargo.toml file for dependencies
cat > Cargo.toml << EOF
[package]
name = "agent_cache_system"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
EOF

# Compile the Rust file
echo "Building agent_cache_system.rs..."
rustc --edition=2021 -L dependency=. agent_cache_system.rs

# Check if compilation was successful
if [ $? -eq 0 ]; then
    echo "Build successful! The binary is available at ./agent_cache_system"
    # Make the binary executable
    chmod +x agent_cache_system
else
    echo "Build failed!"
    exit 1
fi

# Clean up temporary Cargo.toml file
rm Cargo.toml

echo "You can run the program with: ./agent_cache_system"