# Collaborative Intelligence CLI Systems

## Overview

This directory contains a centralized organization of all Command Line Interface (CLI) tools used throughout the Collaborative Intelligence system. The goal of this reorganization is to improve maintainability, streamline development, and provide a clear structure for developers working with these tools.

## Structure

The CLI system is organized into the following components:

```
CLIs/
├── ci/                    # Main CI CLI system (external repository integration)
│   ├── bash/              # Original Bash implementation
│   └── rust/              # Rust implementation with modular commands
│       ├── src/
│       │   ├── commands/  # Command modules (init, verify, etc.)
│       │   ├── config/    # Configuration management
│       │   └── helpers/   # Shared utility functions
├── tl/                    # Topologist CLI (internal repository management)
│   ├── src/
│   │   ├── commands/      # Command modules organized by domain
│   │   │   ├── agent.rs   # Agent management commands
│   │   │   ├── memory.rs  # Memory operations
│   │   │   ├── knowledge.rs # Knowledge management
│   │   │   ├── system.rs  # System operations
│   │   │   └── debug.rs   # Debugging utilities
│   │   ├── config/        # Configuration management
│   │   ├── helpers/       # Shared utility functions
│   │   └── main.rs        # Entry point with command delegation
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

The main Collaborative Intelligence command-line tool for external repository integration:

#### Bash Implementation (ci/bash/)
- Original shell-based implementation
- Commands: `ci-init`, `ci-fix`, `ci-verify`
- Handles project initialization, integration, and verification

#### Rust Implementation (ci/rust/)
- Modern, type-safe implementation 
- Enhanced error handling and improved performance
- Modular command architecture with clear separation of concerns
- Commands organized into discrete modules with focused responsibilities:
  - `init.rs`: Project initialization
  - `fix.rs`: Error correction and integration repair
  - `integrate.rs`: CI system integration
  - `load.rs`: Loading CI resources
  - `map.rs`: Repository mapping and analysis
  - `status.rs`: Integration status reporting
  - `verify.rs`: Validation of integration
- Shared utility modules in `helpers/` for terminal display, path operations, and execution

### 2. Topologist CLI (tl/)

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