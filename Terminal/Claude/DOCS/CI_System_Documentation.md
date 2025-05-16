# Collaborative Intelligence System Documentation

## Overview

The Collaborative Intelligence (CI) system is a framework for managing and integrating specialized AI agents with Claude Code. It provides a structured environment for agent organization, session management, and configuration management.

This document provides a comprehensive guide to the CI system command-line interface and its capabilities.

## Implementation Status

**Current Status: Phase 2 - 90% Complete (May 15, 2025)**

Recent implementations:
- ✅ Agent management command module
- ✅ Session management command module
- ✅ Enhanced configuration validation
- ✅ Comprehensive documentation

For detailed status information, see the [REFACTORING_PLAN.md](/REFACTORING_PLAN.md) file.

## CI Command-Line Interface

### Basic Usage

```
CI COMMAND [OPTIONS]
```

### Core Commands

#### init

Initializes a new project with Collaborative Intelligence integration.

```bash
CI init PROJECT_NAME [OPTIONS]
```

Options:
- `--agents=LIST` - Comma-separated list of agents (default: Athena,ProjectArchitect)
- `--integration=TYPE` - Integration type: embedded, sibling, symlink (default: embedded)
- `--no-fast` - Disable fast activation
- `--ci-path=PATH` - Path to CI repository

#### integrate

Integrates CI into an existing project.

```bash
CI integrate [PATH] [OPTIONS]
```

Options:
- `--agents=LIST` - Comma-separated list of agents
- `--integration=TYPE` - Integration type
- `--no-fast` - Disable fast activation
- `--ci-path=PATH` - Path to CI repository

#### status

Checks the CI integration status of a project.

```bash
CI status [PATH]
```

#### fix

Repairs CI integration issues.

```bash
CI fix [PATH] [TYPE]
```

Where TYPE can be:
- `verify` - Fix timeout in verify command

#### verify

Verifies that CI integration is working properly.

```bash
CI verify [PATH]
```

### Advanced Commands

The CI system includes additional command modules for advanced functionality. These modules have been implemented as part of the Phase 2 refactoring plan.

#### Agent Management

Manages agents in the Collaborative Intelligence system. This module provides comprehensive tools for creating, configuring, and managing specialized agents.

```bash
ci-tools/cmd/agent.sh <subcommand> [options]
```

Subcommands:
- `list` - List all available agents with their status and description
- `info <agent_name>` - Show detailed information about an agent, including capabilities and sessions
- `create <agent_name>` - Create a new agent with standard files (README.md, MEMORY.md, ContinuousLearning.md)
- `enable <agent_name>` - Enable an agent in the current project configuration
- `disable <agent_name>` - Disable an agent in the current project configuration
- `template <template>` - Create a new agent from a predefined template

**Implementation details:**
- Comprehensive agent data visualization with formatted output
- JSON-based agent configuration management
- Session integration for agent-specific session tracking
- Template-based agent creation for standardization

#### Session Management

Manages interaction sessions in the Collaborative Intelligence system. This module provides tools for creating, documenting, and archiving interaction sessions.

```bash
ci-tools/cmd/session.sh <subcommand> [options]
```

Subcommands:
- `list` - List all available sessions with timestamps and metadata
- `info <session_id>` - Show detailed information about a session, including contents and metadata
- `create <session_name>` - Create a new session with standardized structure
- `archive <session_id>` - Archive a completed session for reference
- `export <session_id>` - Export a session to markdown or JSON format
- `import <file>` - Import a session from markdown or JSON file

Options:
- `-a, --agent AGENT` - Specify agent for the session
- `--format FORMAT` - Specify export format (markdown or json)

**Implementation details:**
- Session ID generation with timestamp and descriptive slug
- Structured session documentation with README.md
- Metadata tracking with JSON
- Agent-specific session organization
- Import/export functionality for session sharing

#### Configuration Validation

Validates and fixes configuration issues in the CI system. This utility ensures that configuration files and directory structures meet system requirements.

```bash
ci-tools/validate-config.sh [OPTIONS]
```

Options:
- `-v, --verbose` - Show detailed validation information during checks
- `-f, --fix` - Automatically fix configuration issues when detected

**Implementation details:**
- Comprehensive validation of configuration value structure
- Directory path existence verification
- Agent validity checking to ensure referenced agents exist
- Auto-repair capabilities for common configuration issues
- Detailed error reporting and resolution suggestions
- Non-destructive checking mode for safety

## System Architecture

### Configuration Files

The CI system stores its configuration in the following JSON files:

- `$CI_ROOT_DIR/.ci-config/config.json` - Global configuration
- `$CI_ROOT_DIR/.ci-config/user.json` - User preferences
- `$CI_ROOT_DIR/.ci-config/project.json` - Project-specific settings

### Directory Structure

The CI system uses the following directory structure:

- `$CI_ROOT_DIR/AGENTS/` - Agent definitions and memory files
  - `$CI_ROOT_DIR/AGENTS/[AgentName]/` - Individual agent directories
  - `$CI_ROOT_DIR/AGENTS/[AgentName]/README.md` - Agent description
  - `$CI_ROOT_DIR/AGENTS/[AgentName]/MEMORY.md` - Agent memory storage
  - `$CI_ROOT_DIR/AGENTS/[AgentName]/ContinuousLearning.md` - Learning record
  - `$CI_ROOT_DIR/AGENTS/[AgentName]/Sessions/` - Agent-specific sessions

- `$CI_ROOT_DIR/Sessions/` - System-wide session records
  - `$CI_ROOT_DIR/Sessions/[SessionID]/` - Individual session directories
  - `$CI_ROOT_DIR/Sessions/[SessionID]/README.md` - Session description
  - `$CI_ROOT_DIR/Sessions/[SessionID]/metadata.json` - Session metadata

- `$CI_ROOT_DIR/ci-tools/` - Command-line tools and utilities
  - `ci-tools/cmd/` - Command modules
  - `ci-tools/lib/` - Shared libraries
  - `ci-tools/templates/` - Templates for agents and projects

## Integration Types

The CI system supports the following integration types:

- `embedded` - CI files are directly integrated into the project
- `sibling` - CI files are in a sibling directory to the project
- `symlink` - CI files are symlinked into the project

## Environment Variables

The CI system uses the following environment variables:

- `CI_REPO_PATH` - Path to the CI repository
- `CI_DEBUG` - Enable debug mode when set to "true"
- `CI_VERBOSE` - Enable verbose output when set to "true"

## Best Practices

### Agent Organization

Based on our implementation of the agent management system, we recommend these best practices:

- **Specialization**: Create focused agents with clearly defined responsibilities
- **Documentation**: Maintain comprehensive README.md files with capabilities section
- **Memory Management**: Separate knowledge into MEMORY.md (core) and ContinuousLearning.md (evolving)
- **Session Organization**: Create agent-specific sessions for domain-relevant interactions
- **Enabling/Disabling**: Use agent.sh commands to manage agent availability rather than moving files
- **Templates**: Use templates for consistent agent creation across the organization

### Session Management

Our session.sh implementation revealed these effective practices:

- **Structured Sessions**: Create sessions with clear purpose, defined in README.md
- **Metadata Tracking**: Use metadata.json to track session attributes and relationships
- **Regular Archiving**: Archive completed sessions rather than deleting them
- **Export for Sharing**: Export important sessions to markdown for sharing with team members
- **ID Conventions**: Use descriptive slugs in session IDs to improve searchability
- **Agent Association**: Associate sessions with specific agents when domain-specific

### Configuration Management

Based on our enhanced configuration validation implementation:

- **Regular Validation**: Run validate-config.sh periodically, not just when issues arise
- **Proactive Fixing**: Use the --fix option to address issues before they cause problems
- **Configuration Backups**: Back up configuration before making major changes
- **Path Consistency**: Use absolute paths in configuration to prevent path resolution issues
- **Agent Verification**: Validate that enabled agents actually exist in the system
- **Documentation**: Keep notes of configuration changes in a dedicated file

### Modular Architecture

The CI system's modular design works best when following these principles:

- **Command Isolation**: Keep command modules independent with minimal dependencies
- **Shared Libraries**: Use the lib/ directory for common functions instead of duplicating code
- **Consistent Interfaces**: Maintain consistent parameter parsing across commands
- **Verbose Output**: Include verbose options for all commands to aid troubleshooting
- **Error Handling**: Implement thorough error handling with descriptive messages
- **Documentation**: Document each command thoroughly in both --help and USAGE.md

## Getting Started

### Setting Up a New Project

1. Initialize a new project with CI:
   ```bash
   CI init my-project
   ```

2. Verify the integration:
   ```bash
   CI verify
   ```

3. Start using agents in Claude Code:
   ```
   # In Claude Code
   Athena
   ```

### Adding a New Agent

1. Create a new agent:
   ```bash
   ci-tools/cmd/agent.sh create MyAgent
   ```

2. Enable the agent:
   ```bash
   ci-tools/cmd/agent.sh enable MyAgent
   ```

3. Start using the agent in Claude Code:
   ```
   # In Claude Code
   MyAgent
   ```

### Creating and Managing Sessions

1. Create a new session:
   ```bash
   ci-tools/cmd/session.sh create "Feature Design"
   ```

2. View session details:
   ```bash
   ci-tools/cmd/session.sh info [SESSION_ID]
   ```

3. Export a session:
   ```bash
   ci-tools/cmd/session.sh export [SESSION_ID]
   ```

## Standardization Principles

The implementation of the CI system follows these repository standardization principles:

### Directory Structure Standards

The system uses a consistent directory hierarchy:

- **Command-Type Grouping**: All command modules are in `/ci-tools/cmd/`
- **Library Separation**: Shared code is isolated in `/ci-tools/lib/`
- **Template Organization**: Templates are centralized in `/ci-tools/templates/`
- **Documentation Location**: Usage guides are in the root of `ci-tools/`

### Naming Conventions

The following naming patterns are used consistently:

- **Command Files**: All command modules use the `.sh` extension with descriptive names
- **Library Files**: Library modules use function-focused names (e.g., `config.sh`, `output.sh`)
- **Utility Scripts**: Standalone tools use hyphenated names (e.g., `validate-config.sh`)
- **Functions**: Functions use lowercase with underscores (e.g., `validate_config()`)

### Code Style Standards

All script files follow these conventions:

- **Script Header**: Standard header with description and strict mode
- **Help Function**: All commands provide a consistent `show_help()` function
- **Main Function**: Core logic encapsulated in a `main()` function
- **Function Documentation**: All functions have usage comments
- **Error Handling**: Consistent error reporting through shared libraries
- **Exit Codes**: Standardized exit codes for success (0) and failure (non-zero)

### Documentation Standards

Documentation follows these patterns:

- **File-Level Documentation**: Purpose statement at the top of each file
- **Command Documentation**: Consistent format for command description and usage
- **Examples**: Practical examples for all commands
- **README Presence**: Each directory has a README.md explaining its purpose
- **Usage Guides**: Comprehensive USAGE.md with command details

## Troubleshooting

### Common Issues

1. **Configuration Problems**:
   - Run `ci-tools/validate-config.sh --verbose` to identify issues
   - Use `ci-tools/validate-config.sh --fix` to repair common problems

2. **Integration Issues**:
   - Run `CI fix` to repair integration problems
   - Check `CLAUDE.md` and `CLAUDE.local.md` for proper path references

3. **Agent Issues**:
   - Use `ci-tools/cmd/agent.sh list` to verify agent availability
   - Check agent directories for proper structure

4. **Session Issues**:
   - Use `ci-tools/cmd/session.sh list` to view available sessions
   - Check session metadata for correct information

### Getting Help

For additional assistance, run the help command for any tool:

```bash
CI --help
ci-tools/cmd/agent.sh --help
ci-tools/cmd/session.sh --help
ci-tools/validate-config.sh --help
```