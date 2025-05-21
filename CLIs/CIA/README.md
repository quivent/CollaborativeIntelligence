# CIA CLI

Collaborative Intelligence Adaption CLI for CollaborativeIntelligence repository intelligence adaption.

## Overview

The CIA CLI is an internal tool designed specifically for managing and maintaining the CollaborativeIntelligence repository. Unlike the external CI tool, CIA focuses solely on repository-related tasks such as agent management, memory operations, and knowledge distribution.

## Features

- **Agent Management**: Create, list, and activate agents
- **Memory Operations**: Load, optimize, and cache agent memory
- **Knowledge Management**: Search, index, and distribute knowledge
- **Debug Utilities**: Path information, configuration checks, and formatting tests
- **System Maintenance**: Clean temporary files, verify repository structure, and update terminal title

## Installation

To build and install the CIA CLI:

```bash
cd CLIs/CIA
cargo build --release
cargo install --path .
```

The binary will be available as `CIA` in your cargo bin directory.

## Usage

```
CIA [COMMAND]
```

### Available Commands

- `CIA agent activate <n>`: Activate an agent
- `CIA agent list`: List all available agents
- `CIA agent create <n> [category]`: Create a new agent

- `CIA memory load <agent>`: Load agent memory
- `CIA memory optimize <agent>`: Optimize agent memory
- `CIA memory cache <agent>`: Cache agent memory

- `CIA knowledge search <query> [category]`: Search the knowledge registry
- `CIA knowledge index`: Update knowledge registry indexes
- `CIA knowledge distribute [agent]`: Distribute knowledge to agents

- `CIA debug paths`: Print repo path information
- `CIA debug config`: Check system configuration
- `CIA debug format`: Test terminal formatting
- `CIA debug map`: Display visual map of CLI commands

- `CIA system clean [--cache]`: Clean temporary files
- `CIA system verify`: Verify repository structure
- `CIA system title [title]`: Update terminal title

## Design Principles

1. **Repository-Focused**: CIA is designed specifically for intelligence adaption of the CollaborativeIntelligence repository
2. **Self-Contained**: All functionality is self-contained without external dependencies
3. **Consistent Styling**: Uses consistent ANSI terminal styling for all output
4. **Intuitive Commands**: Command structure follows intuitive patterns for ease of use
5. **Proper Error Handling**: Robust error handling with descriptive error messages

## Architecture

The CIA CLI has been designed with a modular architecture:

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

The CIA CLI is implemented in Rust using:
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

## Collaborative Intelligence Adaption

CIA stands for "Collaborative Intelligence Adaption" with specific terminology distinctions:
- **Adaption**: An ongoing process by the system itself - the active, continuous process of self-modification
- **Adaptation**: Expression of a completed process - the result or state achieved through adaption
- **Adapting**: Reference to an external system adapting - when something else is doing the adapting

This CLI facilitates the ongoing intelligence adaption process within the CollaborativeIntelligence ecosystem.