# CI Tool Migration Guide

## Overview

The CI functionality is being migrated from the CollaborativeIntelligence repository to a dedicated external repository. This guide helps you transition to the new implementation.

## Why Migrate?

The migration provides several benefits:

- **Clearer Separation of Concerns**: Separate CLI tools for project integration (CI) and repository management (TL)
- **Simplified Dependencies**: Independent evolution of each tool
- **Faster Updates**: Updates can be released independently
- **Better Maintainability**: Focused repositories with specific purposes
- **Consistent Experience**: Unified experience across different projects

## Installation Options

### Option 1: Install from GitHub

```bash
git clone https://github.com/anthropics/ci-tools.git
cd ci-tools
cargo install --path .
```

### Option 2: Install from Cargo

```bash
cargo install ci-tools
```

### Option 3: Script Installation

```bash
curl -sSL https://raw.githubusercontent.com/anthropics/ci-tools/main/install.sh | bash
```

## Environment Configuration

Configure using environment variables:

- `CI_EXTERNAL_PATH`: Path to the external CI repository
  ```bash
  export CI_EXTERNAL_PATH=/path/to/ci-tools
  ```

- `CI_FORCE_LOCAL`: Set to "1" to force using local implementation
  ```bash
  export CI_FORCE_LOCAL=1
  ```

- `CI_NO_DEPRECATION_NOTICE`: Set to "1" to disable deprecation notices
  ```bash
  export CI_NO_DEPRECATION_NOTICE=1
  ```

## Detection Mechanism

The system automatically detects the external implementation in the following order:

1. Checks if `CI_FORCE_LOCAL=1` is set (forces local implementation)
2. Checks if `ci-external` is in PATH
3. Checks if `$HOME/.ci/bin/ci` exists
4. Checks if `$CI_EXTERNAL_PATH/bin/ci` exists

When the external implementation is detected, commands will be automatically delegated.

## Compatibility

The external implementation is compatible with projects created using the local implementation. No migration of existing projects is necessary.

The following is maintained for compatibility:

- **Command Line Interface**: Same commands and arguments
- **Configuration Files**: Same format and structure
- **Project Formats**: Works with existing projects
- **Plugin System**: Supports existing plugins

## Transition Period

During the transition period:

1. **Both Implementations Coexist**: Both local and external implementations will be available
2. **External is Preferred**: External implementation will be used when available
3. **Graceful Fallback**: Falls back to local implementation when needed
4. **Deprecation Notices**: Shows notices about the migration
5. **Final Removal**: Eventually, local implementation will be removed

## Command Differences

| Command | Local Implementation | External Implementation |
|---------|---------------------|-------------------------|
| `ci init` | Creates project in current repository | Same behavior, enhanced features |
| `ci integrate` | Integrates with existing project | Same behavior, enhanced features |
| `ci status` | Shows project status | Same behavior, enhanced features |
| `ci fix` | Fixes common issues | Same behavior, enhanced features |
| `ci verify` | Verifies project setup | Same behavior, enhanced features |
| `ci load` | Loads specific agent | Same behavior, enhanced features |
| `ci map` | Maps repository structure | Same behavior, enhanced features |

## Troubleshooting

If you encounter issues:

### External Tool Not Detected

If the external tool is installed but not detected:

1. Check if it's in your PATH:
   ```bash
   which ci-external
   ```

2. Check if environment variables are set correctly:
   ```bash
   echo $CI_EXTERNAL_PATH
   ```

3. Force using the external implementation:
   ```bash
   ci-external --version
   ```

### Command Errors

If a command fails when using the external implementation:

1. Try forcing the local implementation:
   ```bash
   CI_FORCE_LOCAL=1 ci <command>
   ```

2. Check logs for errors:
   ```bash
   CI_LOG_LEVEL=debug ci <command>
   ```

3. Reinstall the external tool:
   ```bash
   cargo install --force ci-tools
   ```

### Multiple Versions

If you have multiple versions installed:

1. Check which version is being used:
   ```bash
   ci --version
   ci-external --version
   ```

2. Update all versions:
   ```bash
   cargo install --force ci-tools
   ```

## Reporting Issues

If you encounter any issues during the migration:

1. Check the [Known Issues](https://github.com/anthropics/ci-tools/issues) list
2. Report new issues on [GitHub](https://github.com/anthropics/ci-tools/issues/new)
3. Include details about your environment and the specific error