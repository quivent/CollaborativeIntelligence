# CI System Workflow Integration

This document outlines the integration of the new Collaborative Intelligence command module system with existing workflows.

## Integration Overview

The new CI command module system enhances the existing CI workflows by:

1. Providing a modular architecture for command implementation
2. Adding agent and session management capabilities
3. Improving error handling and configuration validation
4. Maintaining backward compatibility with existing scripts

This document explains how to integrate these improvements while maintaining compatibility with existing workflows.

## Backward Compatibility

The refactored system maintains backward compatibility through:

1. **Wrapper Scripts**: Root-level wrapper scripts that preserve the original command interface
2. **Environment Variables**: Continued support for existing environment variables
3. **Configuration Format**: Backward compatible configuration files
4. **Command Structure**: Consistent command structure for the main CI commands

## Integration Steps

### 1. Update Root Wrapper Scripts

The root directory contains these wrapper scripts:

- `CI` - Main command-line tool
- `ci-init` - Project initialization shorthand
- `ci-verify` - Verification shorthand 
- `ci-fix` - Fix issues shorthand
- `ci-agent` - NEW: Agent management
- `ci-session` - NEW: Session management

These scripts should all use a consistent pattern:

```bash
#!/bin/bash
# ci-xxx - Description
export CI_REPO_PATH="${CI_REPO_PATH:-$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)}"
exec "$CI_REPO_PATH/ci-tools/cmd/xxx.sh" "$@"
```

### 2. Update Project Integration

For projects that use CI integration:

1. **Check existing integrations**:
   ```bash
   CI status /path/to/project
   ```

2. **Fix any integration issues**:
   ```bash
   CI fix /path/to/project
   ```

3. **Update to use new modules** (if desired):
   ```bash
   # Enable specific agents
   ci-agent enable Athena ProjectArchitect
   
   # Create project-specific sessions
   ci-session create "Project Development"
   ```

### 3. CLI Integration

The new command modules can be used in shell scripts and aliases:

```bash
# Add aliases to .bashrc or .zshrc
alias cia='ci-agent'
alias cis='ci-session'

# Use in shell scripts
if ci-agent list | grep -q "Athena"; then
  echo "Athena agent is available"
fi
```

## Common Workflows

### Development Workflow

```bash
# Initialize a new project
CI init my-project

# Enable specific agents for the project
cd my-project
ci-agent enable Athena Debugger Optimizer

# Create a development session
ci-session create "Feature Development"

# Verify integration
CI verify
```

### Documentation Workflow

```bash
# Create a documentation session
ci-session create "Documentation Update" --agent DocumentationSpecialist

# Export the session when complete
ci-session export "Documentation Update" --format markdown
```

### Debugging Workflow

```bash
# Enable the Debugger agent
ci-agent enable Debugger

# Create a debug session
ci-session create "Bug Fix" --agent Debugger

# Archive the session when complete
ci-session archive "Bug Fix"
```

## Integration with External Tools

### Git Integration

Add CI status checking to Git hooks:

```bash
# In .git/hooks/pre-commit
#!/bin/bash
if ! CI verify &>/dev/null; then
  echo "CI verification failed. Run 'CI fix' to repair."
  exit 1
fi
```

### CI/CD Pipeline Integration

Add CI verification to CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
jobs:
  verify-ci:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Set up CI
        run: |
          git clone https://github.com/your-username/CollaborativeIntelligence.git ci-repo
          export CI_REPO_PATH=$(pwd)/ci-repo
          ci-repo/CI verify
```

## Automated Testing

The command modules include automated tests for validating integration:

```bash
# Run all tests
ci-tools/test/run-all.sh

# Test specific component
ci-tools/test/test-agent.sh
ci-tools/test/test-session.sh
```

## Configuration Management

The configuration system supports three levels:

1. **System-wide**: In `CI_REPO_PATH/.ci-config/config.json`
2. **User-specific**: In `CI_REPO_PATH/.ci-config/user.json`
3. **Project-specific**: In `PROJECT_DIR/.collaborative-intelligence.json`

To standardize configuration across projects:

```bash
# Create template config
ci-tools/cmd/config.sh template > template-config.json

# Apply to project
ci-tools/cmd/config.sh apply template-config.json /path/to/project
```

## Environment Variable Management

Environment variables can be set in project-specific `.env` files:

```bash
# Create .env file
echo "CI_REPO_PATH=/path/to/CollaborativeIntelligence" > .env
echo "CI_DEBUG=true" >> .env
```

These variables will be loaded by the CI tools automatically.

## Customizing Integration

For custom integration needs:

1. **Custom Agents**: Create specialized agents for specific workflows
   ```bash
   ci-agent create MyWorkflowAgent
   ```

2. **Custom Sessions**: Create and archive sessions for specific tasks
   ```bash
   ci-session create "Custom Task"
   ```

3. **Custom Templates**: Use templates for consistent configurations
   ```bash
   ci-tools/cmd/template.sh create my-template
   ```

## Troubleshooting Integration

### Verifying Integration

Run the verification tool with verbose output:

```bash
CI verify --verbose
```

### Fixing Integration Issues

Fix common integration issues:

```bash
# Fix verification timeout
CI fix verify

# Fix configuration issues
ci-tools/validate-config.sh --fix
```

### Logging Issues

Enable debug logging for troubleshooting:

```bash
export CI_DEBUG=true
CI status
```

## Conclusion

The new modular CI system can be seamlessly integrated with existing workflows by:

1. Using the backward-compatible wrapper scripts
2. Updating project configurations as needed
3. Leveraging the new agent and session management capabilities
4. Following standardized configuration practices

For more information on specific commands, see the [COMMAND_USAGE.md](./COMMAND_USAGE.md) document.