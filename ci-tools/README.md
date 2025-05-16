# Collaborative Intelligence Tools

This directory contains command-line tools and utilities for the Collaborative Intelligence (CI) system.

## Overview

The CI tools provide a modular and extensible framework for managing Collaborative Intelligence integration, agents, sessions, and configuration.

## Directory Structure

- `/ci-tools/cmd/` - Command modules
  - `init.sh` - Project initialization
  - `verify.sh` - Integration verification
  - `fix.sh` - Error resolution
  - `agent.sh` - Agent management
  - `session.sh` - Session handling
- `/ci-tools/lib/` - Shared libraries
  - `common.sh` - Path handling, argument parsing, file operations
  - `output.sh` - Standardized colored output, progress indicators
  - `config.sh` - Configuration loading/saving/validation
- `/ci-tools/templates/` - Templates for agents and projects

## Core Scripts

- `CI` - Main command-line interface
- `validate-config.sh` - Configuration validation utility
- `install.sh` - Installation script

## Command Modules

### init.sh

Initializes a new project with Collaborative Intelligence integration.

```bash
CI init PROJECT_NAME [OPTIONS]
```

### verify.sh

Verifies CI integration is working properly.

```bash
CI verify [PATH]
```

### fix.sh

Repairs CI integration issues.

```bash
CI fix [PATH] [TYPE]
```

### agent.sh

Manages agents in the Collaborative Intelligence system.

```bash
cmd/agent.sh <subcommand> [options]
```

Subcommands:
- `list` - List all available agents
- `info <agent_name>` - Show detailed information about an agent
- `create <agent_name>` - Create a new agent
- `enable <agent_name>` - Enable an agent in the current project
- `disable <agent_name>` - Disable an agent in the current project
- `template <template>` - Create an agent from a template

### session.sh

Manages interaction sessions in the Collaborative Intelligence system.

```bash
cmd/session.sh <subcommand> [options]
```

Subcommands:
- `list` - List all available sessions
- `info <session_id>` - Show detailed information about a session
- `create <session_name>` - Create a new session
- `archive <session_id>` - Archive a session
- `export <session_id>` - Export a session to markdown or JSON
- `import <file>` - Import a session from markdown or JSON

## Utilities

### validate-config.sh

Validates and fixes configuration issues.

```bash
./validate-config.sh [OPTIONS]
```

Options:
- `-v, --verbose` - Show detailed validation information
- `-f, --fix` - Fix configuration issues when detected

## Documentation

For detailed documentation, see [USAGE.md](USAGE.md) in this directory.

## Installation

To install the CI tools:

```bash
./install.sh
```

## Development

The CI tools follow a modular architecture with shared libraries and command modules. To add a new command:

1. Create a new command module in the `cmd/` directory
2. Follow the module template with standardized argument parsing
3. Use the shared libraries for common functionality
4. Update the documentation in USAGE.md

## Testing

To test the CI tools:

1. Use the built-in validation functionality
2. Run commands with the `--verbose` flag for detailed output
3. Check error handling with invalid inputs