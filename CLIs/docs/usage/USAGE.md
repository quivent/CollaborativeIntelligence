# Collaborative Intelligence CLI Usage Guide

This document provides detailed information about the Collaborative Intelligence command-line interface.

## Basic Usage

```
CI COMMAND [OPTIONS]
```

## Core Commands

The CI system offers the following core commands:

### init

Initialize a new project with Collaborative Intelligence integration.

```
CI init PROJECT_NAME [OPTIONS]
```

Options:
- `--agents=LIST` - Comma-separated list of agents (default: Athena,ProjectArchitect)
- `--integration=TYPE` - Integration type: embedded, sibling, symlink (default: embedded)
- `--no-fast` - Disable fast activation
- `--ci-path=PATH` - Path to CI repository

Example:
```
CI init my-project --agents=Athena,Gaia
```

### integrate

Integrate CI into an existing project.

```
CI integrate [PATH] [OPTIONS]
```

Options:
- `--agents=LIST` - Comma-separated list of agents (default: Athena,ProjectArchitect)
- `--integration=TYPE` - Integration type: embedded, sibling, symlink (default: embedded)
- `--no-fast` - Disable fast activation
- `--ci-path=PATH` - Path to CI repository

Example:
```
CI integrate --agents=TheFixer,Optimizer
```

### status

Check CI integration status of a project.

```
CI status [PATH]
```

Example:
```
CI status
```

### fix

Repair CI integration issues.

```
CI fix [PATH] [TYPE]
```

Where TYPE can be:
- `verify` - Fix timeout in verify command

Example:
```
CI fix
CI fix verify
```

### verify

Verify CI integration is working properly.

```
CI verify [PATH]
```

Example:
```
CI verify
```

## Advanced Commands

The CI system includes additional command modules for advanced functionality.

### agent

Manage agents in the Collaborative Intelligence system.

```
ci-tools/cmd/agent.sh <subcommand> [options]
```

Subcommands:
- `list` - List all available agents
- `info <agent_name>` - Show detailed information about an agent
- `create <agent_name>` - Create a new agent
- `enable <agent_name>` - Enable an agent in the current project
- `disable <agent_name>` - Disable an agent in the current project
- `template <template>` - Create an agent from a template

Options:
- `-c, --config FILE` - Use custom configuration file

Examples:
```
ci-tools/cmd/agent.sh list
ci-tools/cmd/agent.sh info Athena
ci-tools/cmd/agent.sh create MyAgent
ci-tools/cmd/agent.sh enable Optimizer
ci-tools/cmd/agent.sh disable Debugger
ci-tools/cmd/agent.sh template DataScientist
```

### session

Manage interaction sessions in the Collaborative Intelligence system.

```
ci-tools/cmd/session.sh <subcommand> [options]
```

Subcommands:
- `list` - List all available sessions
- `info <session_id>` - Show detailed information about a session
- `create <session_name>` - Create a new session
- `archive <session_id>` - Archive a session
- `export <session_id>` - Export a session to markdown or JSON
- `import <file>` - Import a session from markdown or JSON

Options:
- `-a, --agent AGENT` - Specify agent for the session
- `-c, --config FILE` - Use custom configuration file
- `--format FORMAT` - Specify export format (markdown or json)

Examples:
```
ci-tools/cmd/session.sh list
ci-tools/cmd/session.sh list --agent Athena
ci-tools/cmd/session.sh info ABC123
ci-tools/cmd/session.sh create "Feature Planning"
ci-tools/cmd/session.sh archive ABC123
ci-tools/cmd/session.sh export ABC123
ci-tools/cmd/session.sh import session.md
```

### validate-config

Validate and fix configuration issues.

```
ci-tools/validate-config.sh [OPTIONS]
```

Options:
- `-v, --verbose` - Show detailed validation information
- `-f, --fix` - Fix configuration issues when detected
- `-c, --config FILE` - Use custom configuration file

Examples:
```
ci-tools/validate-config.sh --verbose
ci-tools/validate-config.sh --fix
```

## Environment Variables

The CI system uses the following environment variables:

- `CI_REPO_PATH` - Path to the CI repository (default: determined automatically)
- `CI_DEBUG` - Enable debug mode when set to "true"
- `CI_VERBOSE` - Enable verbose output when set to "true"

## Configuration Files

The CI system stores its configuration in the following files:

- `$CI_ROOT_DIR/.ci-config/config.json` - Global configuration
- `$CI_ROOT_DIR/.ci-config/user.json` - User preferences
- `$CI_ROOT_DIR/.ci-config/project.json` - Project-specific settings

## Directory Structure

The CI system uses the following directory structure:

- `$CI_ROOT_DIR/AGENTS/` - Agent definitions and memory files
- `$CI_ROOT_DIR/Sessions/` - Session records and outputs
- `$CI_ROOT_DIR/ci-tools/` - Command-line tools and utilities
  - `cmd/` - Command modules
  - `lib/` - Shared libraries
  - `templates/` - Templates for agents and projects

## Integration Types

The CI system supports the following integration types:

- `embedded` - CI files are directly integrated into the project
- `sibling` - CI files are in a sibling directory to the project
- `symlink` - CI files are symlinked into the project