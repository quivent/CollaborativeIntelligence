# Collaborative Intelligence CLI Systems

## Overview

This directory contains CLI tools for the Collaborative Intelligence system. After the CLI migration and decoupling process (completed 2025-05-22), the structure has been simplified to focus on active, non-redundant tools.

## Current Structure

```
interfaces/
├── CIA/                   # CIA CLI - CollaborativeIntelligence-specific functionality
│   ├── src/
│   │   ├── commands/      # Command modules (agent, memory, knowledge, etc.)
│   │   ├── config/        # Configuration management
│   │   ├── helpers/       # Shared utility functions
│   │   └── terminal/      # Terminal integration features
├── ci/                    # Legacy CI system components (mostly for reference)
│   ├── bash/              # Bash utilities and scripts
│   └── rust/              # Rust implementation (separate from ../CI external system)
├── memory/                # Memory management CLI tools
│   ├── src/               # Rust source code
│   └── schemas/           # JSON schemas
├── knowledge/             # Knowledge registry CLI tools
│   └── src/               # Implementation
├── topologist/            # Topological analysis tools
│   └── src/               # Rust source code
├── agent-cache/           # Agent caching CLI
│   └── src/               # Rust source code
├── plugins/               # Plugin systems for external integration
│   └── claude/            # Claude CLI plugin
└── docs/                  # Centralized CLI documentation
    ├── usage/             # Usage guides
    ├── development/       # Development documentation
    └── examples/          # Example usage
```

## CLI Components

### 1. CIA CLI (CIA/)

The primary CLI for CollaborativeIntelligence-specific functionality:
- **Agent Management**: Load, activate, and manage agents
- **Memory Operations**: Handle agent memory and learning systems  
- **Knowledge Management**: Access and search knowledge registry
- **System Operations**: Repository maintenance and diagnostics
- **Terminal Integration**: Enhanced terminal title management and context tracking

### 2. Legacy CI System (ci/)

**Note**: After the 2025-05-22 decoupling, this system is largely for reference. Active CI functionality moved to external ../CI system.

#### Bash Implementation (ci/bash/)
- Legacy shell-based utilities and scripts
- Agent activation, config validation, terminal title updates

#### Rust Implementation (ci/rust/)
- Experimental Rust implementation (separate from external ../CI)
- Modular command architecture for learning purposes

### 3. Specialized CLI Tools
- Agent-focused command structure with domain-specific modules
- Role-based permissions and operation verification
- High-performance Rust implementation
- Comprehensive error handling with detailed diagnostics

#### Command Structure
- **agent.rs**: Agent lifecycle and metadata management
- **memory.rs**: Memory operations and optimization
- **knowledge.rs**: Knowledge management and distribution
- **system.rs**: Repository and system-level operations
- **debug.rs**: Diagnostics and troubleshooting utilities

Each command module follows a consistent pattern with:
- Command registration and argument parsing
- Operation execution with proper validation
- Structured error handling and reporting
- Terminal-friendly output formatting

### 3. Memory CLI System (memory/)

A Rust-based CLI system for managing agent memory:

- `memory_cli.rs`: Primary interface for memory operations
- `unified_memory_cli.rs`: Consolidated memory management
- Supporting modules for loading, parsing, and updating memory segments

Features include:
- Loading agent memory with contextual filtering
- Memory validation and checksum caching
- Status reporting and diagnostics

### 4. Agent Cache CLI (agent-cache/)

CLI for managing agent caching operations:

- `main.rs`: Command-line interface with subcommands
- Supports activation, deactivation, status reporting, and cache cleanup

### 5. Knowledge Registry CLI (knowledge/)

A Bash-based tool for searching and managing the knowledge registry:

- `search_cli.sh`: Simple interface to query and retrieve information
- Supports category-based, tag-based, and full-text search

### 6. Claude CLI Plugin (plugins/claude/)

A plugin extending the Claude CLI with CI integration:

- `claude-init-impl.sh`: Implementation for the `claude init` command
- Adds the `--with-ci` flag to initialize projects with CI integration

## Usage

Most CLIs can be invoked directly, but for convenience, the following patterns are recommended:

### CI CLI
```bash
# Initialize a new project
ci init my-project

# Integrate CI into an existing project
ci integrate /path/to/project

# Check integration status
ci status

# Verify integration
ci verify
```

### Memory CLI
```bash
# Load agent memory
memory-cli load --agent Athena

# Check cache status
memory-cli status
```

### Agent Cache CLI
```bash
# Activate an agent
agent-cache activate Athena --directory /current/path --initializer User

# List active agents
agent-cache list
```

## Development

When developing or extending CLI functionality:

1. Choose the appropriate CLI component to extend
2. Maintain backward compatibility with existing commands
3. Follow language-specific conventions (Bash or Rust)
4. Add appropriate documentation in the docs/ directory
5. For Rust implementations, use the standard crates:
   - clap: For command-line argument parsing
   - anyhow/thiserror: For error handling
   - serde: For serialization/deserialization

## Workspace Benefits

The consolidated CLI structure provides:

1. **Unified Build Process**: Single `cargo build` command builds all CLIs
2. **Shared Dependencies**: Common crates defined in workspace 
3. **Consistent Versioning**: All CLIs use workspace version
4. **Simplified Maintenance**: Centralized dependency management

## Development Guidelines

When developing or extending CLI functionality:

1. Choose the appropriate CLI component to extend
2. Maintain backward compatibility with existing commands
3. Follow the modular architecture pattern
4. Place code in the appropriate module based on responsibility
5. Follow language-specific conventions (Bash or Rust)
6. Add appropriate documentation in the docs/ directory
7. For Rust implementations, use these standard crates:
   - clap: For command-line argument parsing
   - anyhow/thiserror: For error handling
   - serde: For serialization/deserialization
   - colored: For terminal output formatting

## Building and Installation

```bash
# Build all CLIs in workspace
cd interfaces/
cargo build --release

# Install specific CLI
cargo install --path agent-cache

# Build specific CLI
cargo build -p memory
```

## Migration Notes (2025-05-22)

As part of the CLI consolidation:
- `agent_cache/` → `interfaces/agent-cache/` 
- `data/db-cli/` → `interfaces/db/`
- `scripts/` → `interfaces/memory/`

Deprecation notices have been left in original locations to assist with migration.