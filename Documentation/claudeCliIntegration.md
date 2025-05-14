# Claude CLI Integration with Collaborative Intelligence

The Collaborative Intelligence (CI) system integrates seamlessly with the Claude CLI tool. This document explains how they work together and troubleshooting common issues.

## Integration Architecture

The CI system relies on a chain of references between various config files:

1. **Claude CLI startup** - When Claude is started in a directory, it looks for CLAUDE.md or CLAUDE.local.md
2. **Project-level configuration** - CLAUDE.local.md contains references to the central CI repository
3. **CI System loading** - The system loads from the CI repository CLAUDE.md
4. **Agent activation** - Specific agents are loaded based on user input or configuration

## Setup Process

When setting up a new project with CI:

1. Run `CI init PROJECT_NAME` or `CI integrate` for existing projects
2. This creates CLAUDE.local.md with proper paths to the CI repository
3. It also creates .collaborative-intelligence.json with configuration details
4. The system is now ready to use via standard Claude CLI

## Verification and Troubleshooting

### Verifying Integration

Run `CI verify` to check if the integration is working properly. This command:

1. Creates a temporary test file requesting verification from Athena
2. Runs Claude with this input to check for proper response
3. Reports success or failure based on the output

### Common Issues and Solutions

#### Verification Timeout

If `CI verify` hangs or times out:

```bash
CI fix . verify
```

This adds a 10-second timeout to the verification command to prevent indefinite hanging.

#### Stuck Claude Processes

If Claude becomes unresponsive:

```bash
killall claude
```

This terminates all running Claude processes.

#### Incorrect Configuration

If integration files are missing or incorrect:

```bash
CI fix
```

This repairs the integration by recreating necessary configuration files.

#### Agent Not Responding

If a specific agent doesn't respond as expected:

1. Check agent availability in `AGENTS/` directory
2. Verify agent names are correct in configuration
3. Run `CI status` to check integration configuration

## Advanced Configuration

### Integration Types

The CI system supports multiple integration types:

- **embedded** - References absolute path to CI repository (default)
- **sibling** - Expects CI repository in sibling directory
- **symlink** - Uses symbolic links for integration

### Fast Activation

Fast activation improves startup time by:

1. Loading essential components first
2. Deferring non-critical loading
3. Using optimized reference mechanisms

Enable/disable with:
```bash
CI integrate --no-fast  # Disable
CI integrate            # Enable (default)
```

## Reference

For complete CLI documentation, see [CLI Reference](./CLI.md).