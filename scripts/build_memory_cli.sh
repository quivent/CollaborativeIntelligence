#!/usr/bin/env bash
# build_memory_cli.sh - Build the memory CLI tool
# Part of the Memory Optimization Integration Plan

set -euo pipefail

# Determine script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CI_ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "ERROR: Rust and Cargo are required but not installed."
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check if the Cargo.toml file exists
if [[ ! -f "$SCRIPT_DIR/Cargo.toml" ]]; then
    echo "Creating Cargo.toml file..."
    cat > "$SCRIPT_DIR/Cargo.toml" << EOF
[package]
name = "memory_optimizer"
version = "0.1.0"
edition = "2021"
authors = ["Athena & Developer <developers@collaborative-intelligence.org>"]
description = "Memory optimization tools for the Collaborative Intelligence system"

[dependencies]
clap = { version = "3.2", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
md5 = "0.7"
log = "0.4"
env_logger = "0.9"
dashmap = "5.4"
notify = "4.0"
thiserror = "1.0"

[[bin]]
name = "memory_cli"
path = "memory_cli.rs"
EOF
fi

# Check if all required source files exist
for file in "memory_cli.rs" "agent_cache_system.rs" "memory_checksum_cache.rs" "memory_segment_parser.rs" "memory_loader.rs" "memory_updater.rs"; do
    if [[ ! -f "$SCRIPT_DIR/$file" ]]; then
        echo "ERROR: Required source file $file not found."
        echo "Please ensure all memory optimization components are available in the scripts directory."
        exit 1
    fi
fi

# Build the CLI tool
echo "Building memory CLI tool..."
cd "$SCRIPT_DIR"

# Create .cargo/config.toml if it doesn't exist
mkdir -p .cargo
if [[ ! -f ".cargo/config.toml" ]]; then
    cat > .cargo/config.toml << EOF
[build]
target-dir = "${CI_ROOT_DIR}/target"
EOF
fi

# Build in release mode
echo "Running cargo build --release..."
cargo build --release

# Check if build was successful
if [[ $? -eq 0 && -f "${CI_ROOT_DIR}/target/release/memory_cli" ]]; then
    echo "Build successful! CLI tool is available at: ${CI_ROOT_DIR}/target/release/memory_cli"
    
    # Create symbolic link in ci-tools/bin directory if it exists
    CI_TOOLS_BIN="${CI_ROOT_DIR}/ci-tools/bin"
    if [[ -d "$CI_TOOLS_BIN" ]]; then
        ln -sf "${CI_ROOT_DIR}/target/release/memory_cli" "${CI_TOOLS_BIN}/memory_cli"
        echo "Symbolic link created in ci-tools/bin directory."
    fi
    
    # Display help information
    echo -e "\nUsage information:"
    "${CI_ROOT_DIR}/target/release/memory_cli" --help
    
    exit 0
else
    echo "ERROR: Build failed."
    exit 1
fi