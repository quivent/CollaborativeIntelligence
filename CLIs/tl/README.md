# TL CLI

Teaching/Learning CLI for CollaborativeIntelligence repository maintenance.

## Overview

The TL CLI is an internal tool designed specifically for managing and maintaining the CollaborativeIntelligence repository. Unlike the external CI tool, TL focuses solely on repository-related tasks such as agent management, memory operations, and knowledge distribution.

## Features

- **Agent Management**: Create, list, and activate agents
- **Memory Operations**: Load, optimize, and cache agent memory
- **Knowledge Management**: Search, index, and distribute knowledge
- **Debug Utilities**: Path information, configuration checks, and formatting tests
- **System Maintenance**: Clean temporary files, verify repository structure, and update terminal title

## Installation

To build and install the TL CLI:

```bash
cd CLIs/TL
cargo build --release
cargo install --path .
```

The binary will be available as `TL` in your cargo bin directory.

## Usage

```
TL [COMMAND]
```

### Available Commands

- `TL agent activate <n>`: Activate an agent
- `TL agent list`: List all available agents
- `TL agent create <n> [category]`: Create a new agent

- `TL memory load <agent>`: Load agent memory
- `TL memory optimize <agent>`: Optimize agent memory
- `TL memory cache <agent>`: Cache agent memory

- `TL knowledge search <query> [category]`: Search the knowledge registry
- `TL knowledge index`: Update knowledge registry indexes
- `TL knowledge distribute [agent]`: Distribute knowledge to agents

- `TL debug paths`: Print repo path information
- `TL debug config`: Check system configuration
- `TL debug format`: Test terminal formatting
- `TL debug map`: Display visual map of CLI commands

- `TL system clean [--cache]`: Clean temporary files
- `TL system verify`: Verify repository structure
- `TL system title [title]`: Update terminal title

## Design Principles

1. **Repository-Focused**: TL is designed specifically for maintenance of the CollaborativeIntelligence repository
2. **Self-Contained**: All functionality is self-contained without external dependencies
3. **Consistent Styling**: Uses consistent ANSI terminal styling for all output
4. **Intuitive Commands**: Command structure follows intuitive patterns for ease of use
5. **Proper Error Handling**: Robust error handling with descriptive error messages

## Architecture

The TL CLI has been designed with a modular architecture:

- **Configuration**: Central configuration system with repository path detection
- **Error Handling**: Custom error types for reliable error handling and propagation
- **Helper Modules**:
  - `terminal`: Terminal output formatting and styling
  - `path`: Path resolution and repository structure helpers
  - `repository`: Git repository interaction and status checking
  - `execution`: Command execution and process management
- **Utils**: Legacy compatibility wrappers around helper modules
- **Command Modules**: Each command category is implemented in its own module

## Development

The TL CLI is implemented in Rust using:
- `clap`: Command-line argument parsing 
- `colored`: Terminal output styling
- `anyhow`: Error handling and propagation
- `thiserror`: Custom error type definitions
- `glob`: Pattern-based file searching
- `dirs`: Home directory detection

Key components:
- `main.rs`: Entry point and command structure definition
- `config/mod.rs`: Configuration management
- `error.rs`: Custom error types
- `helpers/`: Helper modules for common functionality
  - `terminal.rs`: Terminal output formatting and styling
  - `path.rs`: Path resolution and repository structure helpers
  - `repository.rs`: Git repository interaction
  - `execution.rs`: Command execution utilities
  - `command.rs`: Legacy command helpers (deprecated)
- `utils/`: Compatibility layer for legacy code
  - `terminal.rs`: Terminal output wrappers
  - `path.rs`: Path utility wrappers
- `commands/`: Implementation of all command functionality
  - `agent.rs`: Agent management commands
  - `memory.rs`: Memory system commands
  - `knowledge.rs`: Knowledge management commands
  - `debug.rs`: Debugging utilities
  - `system.rs`: System maintenance commands