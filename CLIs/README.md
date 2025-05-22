# Collaborative Intelligence CLI Systems

## Overview

This directory contains CLI tools for the Collaborative Intelligence system. After the CLI migration and decoupling process (completed 2025-05-22), the structure has been simplified to focus on active, non-redundant tools.

## Current Structure

```
CLIs/
├── CIA/                   # CIA CLI - CollaborativeIntelligence-specific functionality
│   ├── src/
│   │   ├── commands/      # Command modules (agent, memory, knowledge, etc.)
│   │   ├── config/        # Configuration management
│   │   ├── helpers/       # Shared utility functions
│   │   └── terminal/      # Terminal integration features
├── TL-source/             # TL CLI source (internal repository management)
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

### 3. TL CLI Source (TL-source/)

A specialized internal CLI for repository management and topological operations:

#### Features
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

## Usage Examples

### CI & Topologist CLI
```bash
# External repository integration
ci init my-project
ci verify

# Internal repository management
tl agent activate Athena
tl system analyze --report
tl debug trace --command ci-verify
```

### Memory & Cache CLI
```bash
# Memory operations
memory-cli load --agent Athena
memory-cli status

# Agent cache operations
agent-cache activate Athena --directory /current/path --initializer User
agent-cache list
```

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

## Configuration

CLIs support local configuration files that should be excluded from version control:

```
CLIs/
├── tl/
│   ├── config.local.json     # Local configuration (gitignored)
│   └── settings.local.toml   # Local settings (gitignored)
├── ci/
    └── rust/
        └── .cache/           # Local cache (gitignored)
```

## Future Directions

1. **Rust Migration**: Complete migration from Bash to Rust for improved reliability and maintainability
2. **CLI Integration**: Standardize interfaces between CLI components
3. **Enhanced Documentation**: Expand usage examples and integration guides
4. **Plugin System**: Develop a more robust plugin architecture for extensions
5. **Testing Framework**: Add comprehensive test coverage for all CLI components