# Collaborative Intelligence CLI Reference

The Collaborative Intelligence (CI) system includes a command-line interface for managing CI integration with projects. This document provides a reference for the CLI commands and their usage.

## Installation

The CI tools are automatically installed when you clone the CollaborativeIntelligence repository. To add them to your PATH, you can run:

```bash
cd /path/to/CollaborativeIntelligence
source ./integrate-ci.sh
```

## Basic Usage

```bash
CI COMMAND [OPTIONS]
```

## Commands

### Initialize a New Project

```bash
CI init PROJECT_NAME [OPTIONS]
```

Creates a new project directory with CI integration. The directory will contain:
- CLAUDE.md configured to load the CI system
- .collaborative-intelligence.json with integration configuration
- Basic directory structure (src, docs, tests)
- Git initialization with proper .gitignore

**Options:**
- `--agents=LIST` - Comma-separated list of agents (default: Athena,ProjectArchitect)
- `--integration=TYPE` - Integration type: embedded, sibling, symlink (default: embedded)
- `--no-fast` - Disable fast activation
- `--ci-path=PATH` - Path to CI repository

### Integrate with an Existing Project

```bash
CI integrate [PATH] [OPTIONS]
```

Integrates CI with an existing project in the specified directory (defaults to current directory).

**Options:**
- `--agents=LIST` - Comma-separated list of agents (default: Athena,ProjectArchitect)
- `--integration=TYPE` - Integration type: embedded, sibling, symlink (default: embedded)
- `--no-fast` - Disable fast activation
- `--ci-path=PATH` - Path to CI repository

### Check Integration Status

```bash
CI status [PATH]
```

Checks the CI integration status of a project in the specified directory (defaults to current directory).

### Fix Integration Issues

```bash
CI fix [PATH] [TYPE]
```

Repairs CI integration issues in the specified directory (defaults to current directory).

**Fix Types:**
- `verify` - Fix timeout issues in the verification command

### Verify Integration

```bash
CI verify [PATH]
```

Verifies that CI integration is working properly in the specified directory (defaults to current directory).

## Examples

Initialize a new project with specific agents:
```bash
CI init my-project --agents=Athena,Gaia
```

Integrate CI into the current directory:
```bash
CI integrate
```

Check integration status:
```bash
CI status
```

Fix verification timeout issues:
```bash
CI fix . verify
```

Verify integration is working:
```bash
CI verify
```

## Troubleshooting

If the verification command hangs, try:
```bash
CI fix . verify
```

This will add a timeout to prevent the command from hanging indefinitely.

If you continue to have issues with hung Claude processes, you can kill them with:
```bash
killall claude
```