# Collaborative Intelligence CLI Systems

## Overview

This directory contains a centralized organization of all Command Line Interface (CLI) tools used throughout the Collaborative Intelligence system. The goal of this refactoring is to improve maintainability, streamline development, and provide a clear structure for developers working with these tools.

## Structure

The CLI system is organized into the following components:

```
CLIs/
├── ci/                    # Main CI CLI system
│   ├── bash/              # Original Bash implementation
│   └── rust/              # Rust implementation
├── memory/                # Memory management CLI
│   ├── src/               # Rust source code
│   └── schemas/           # JSON schemas
├── agent-cache/           # Agent caching CLI
│   └── src/               # Rust source code
├── knowledge/             # Knowledge registry CLI
│   └── src/               # Implementation
├── plugins/               # CLI plugins
│   └── claude/            # Claude CLI plugin
└── docs/                  # Centralized CLI documentation
    ├── usage/             # Usage guides
    ├── development/       # Development documentation
    └── examples/          # Example usage
```

## CLI Components

### 1. CI Command Line Interface (ci/)

The main Collaborative Intelligence command-line tool with two implementations:

#### Bash Implementation (ci/bash/)
- Original shell-based implementation
- Commands: `ci-init`, `ci-fix`, `ci-verify`
- Handles project initialization, integration, and verification

#### Rust Implementation (ci/rust/)
- Modern, type-safe implementation 
- Enhanced error handling and improved performance
- Maintains feature parity with the Bash implementation
- Currently being phased in as the primary implementation

### 2. Memory CLI System (memory/)

A Rust-based CLI system for managing agent memory:

- `memory_cli.rs`: Primary interface for memory operations
- `unified_memory_cli.rs`: Consolidated memory management
- Supporting modules for loading, parsing, and updating memory segments

Features include:
- Loading agent memory with contextual filtering
- Memory validation and checksum caching
- Status reporting and diagnostics

### 3. Agent Cache CLI (agent-cache/)

CLI for managing agent caching operations:

- `main.rs`: Command-line interface with subcommands
- Supports activation, deactivation, status reporting, and cache cleanup

### 4. Knowledge Registry CLI (knowledge/)

A Bash-based tool for searching and managing the knowledge registry:

- `search_cli.sh`: Simple interface to query and retrieve information
- Supports category-based, tag-based, and full-text search

### 5. Claude CLI Plugin (plugins/claude/)

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

## Future Directions

1. **Rust Migration**: Continuing the migration from Bash to Rust for improved reliability and maintainability
2. **CLI Consolidation**: Further consolidation of overlapping functionality
3. **Enhanced Documentation**: Expanding usage examples and integration guides
4. **Plugin System**: Developing a more robust plugin architecture for extensions