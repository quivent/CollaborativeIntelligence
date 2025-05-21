# Claude CI Plugin

This is a plugin for the Claude CLI that adds support for Collaborative Intelligence (CI) integration.

## Overview

The plugin enhances the Claude CLI's `init` command with the following features:

- `--with-ci`: Initialize a project with CI integration
- `--integration=[type]`: Specify the integration type (embedded, sibling, symlink)
- `--agents=[list]`: Specify which agents to include
- `--no-fast-activation`: Disable fast activation
- `--ci-path=[path]`: Specify a custom path to the CI repository

## Implementation

The plugin is implemented in Rust to ensure high performance, security, and maintainability. It:

1. Detects if the external CI tool is available
2. Uses the external tool when possible
3. Falls back to a local implementation when needed
4. Shows deprecation notices for the local implementation

## Build Process

The plugin is built using Cargo:

```bash
cd CLIs/plugins/claude
cargo build --release
```

This produces a dynamic library that is loaded by the wrapper script.

## Integration

To install the plugin for Claude:

```bash
# From the plugin directory
./install.sh

# Or from anywhere (specifying the Claude installation directory)
./install.sh --claude-path /path/to/claude/installation
```

## Example Usage

Once installed, you can use the CI integration with Claude:

```bash
# Create a new project with CI integration
claude init --with-ci my-project

# Specify agents
claude init --with-ci --agents=Athena,Developer my-project

# Use custom integration type
claude init --with-ci --integration=symlink my-project

# Disable fast activation
claude init --with-ci --no-fast-activation my-project

# Use custom CI path
claude init --with-ci --ci-path=/custom/path/to/ci my-project
```

## External CI Tool Integration

The plugin automatically detects and uses the external CI tool if available:

1. Checks for `ci-external` in PATH
2. Checks for `$HOME/.ci/bin/ci`
3. Checks for `$CI_EXTERNAL_PATH/bin/ci`

To install the external CI tool:

```bash
npm install -g @anthropics/ci-tools
```

---

Last Updated: May 21, 2025