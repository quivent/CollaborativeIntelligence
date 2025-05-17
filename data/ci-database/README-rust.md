# CI Database (Rust Implementation)

A high-performance database management system for the Collaborative Intelligence project, implemented in Rust for type safety, performance, and seamless integration with the CI CLI.

## Features

- Dual database architecture (internal operations + external projects)
- Thread-safe connection pooling
- Comprehensive CLI with 80+ commands
- JSON support for flexible metadata
- Full-text search capabilities
- Automatic timestamp management

## Quick Start

```bash
# Build and initialize
./init-rust.sh

# Or manually:
cargo build --release
./target/release/ci-db db init
```

## Usage

```bash
# Agent management
ci-db agent register "Athena" --agent-type "system-architect"
ci-db agent list
ci-db agent get "Athena"

# Learning system
ci-db learning record "Athena" "protocol_update" "Learned new protocol"
ci-db learning get "Athena"

# Project tracking
ci-db project register "MyProject" "/path/to/project"
ci-db operation log "MyProject" "ci init"

# Knowledge base
ci-db knowledge add "Athena" "system_design" "Content here..."
ci-db knowledge search "protocol"
```

## Architecture

```
ci-database/
├── src/
│   ├── lib.rs              # Main library entry
│   ├── connection.rs       # Database connection management
│   ├── error.rs           # Error handling
│   ├── api.rs             # High-level API
│   ├── internal/          # Internal database (agents, learning)
│   │   ├── models.rs      # Data structures
│   │   └── repository.rs  # Database operations
│   ├── projects/          # Projects database (external tracking)
│   │   ├── models.rs      # Data structures
│   │   └── repository.rs  # Database operations
│   └── bin/
│       └── cli.rs         # Command-line interface
├── internal/
│   └── schema.sql         # Internal database schema
└── projects/
    └── schema.sql         # Projects database schema
```

## Database Structure

### Internal Database
- Agents registry and configuration
- Learning entries and knowledge base
- Session tracking and collaboration
- Memory snapshots

### Projects Database
- Project registration and metadata
- CI operation logging
- Performance metrics
- Feature usage analytics

## Integration with CI

The database seamlessly integrates with the Rust CI CLI:

```rust
// In your CI code
use ci_database::{Database, CIDatabaseAPI};

let db = Database::new("/path/to/data").await?;
let api = CIDatabaseAPI::new(db);

// Use the API
let agent = api.get_agent("Athena").await?;
```

## Development

```bash
# Run tests
cargo test

# Build in release mode
cargo build --release

# Run with verbose logging
ci-db --data-dir ./test-data -vvv agent list
```

## Performance

- Startup time: ~1-5ms
- Query performance: <1ms for indexed queries
- Memory usage: ~10MB baseline
- Connection pooling for concurrent access

## Migration from Python

This Rust implementation replaces the Python version with:
- 5-10x performance improvement
- Type safety and compile-time checks
- Direct integration with Rust CI
- Single binary distribution

## License

Part of the Collaborative Intelligence project