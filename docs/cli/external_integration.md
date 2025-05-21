# External CI Integration

This document describes how the Collaborative Intelligence (CI) system integrates with the external CI Tools repository.

## Architecture

The CLI system is now divided into two separate components:

1. **External CI Tool**: Handles project integration and management
   - Repository: https://github.com/anthropics/ci-tools
   - Package: @anthropics/ci-tools
   - Commands: `ci-external`, `ci`

2. **Internal TL CLI**: Manages the CollaborativeIntelligence repository
   - Repository: This repository (CollaborativeIntelligence)
   - Commands: `tl`

## Integration Points

### Wrapper Scripts

The root CI command scripts (`ci-init`, `ci-fix`, `ci-verify`) now act as wrappers that:

1. Detect if the external CI tool is installed
2. If available, delegate to the external tool
3. If not available, fall back to the local implementation
4. Show deprecation notices when using the local implementation

### Implementation

```bash
# Check if external CI is available
if command -v ci-external &> /dev/null; then
  # Use external CI
  ci-external [command] [args]
else
  # Fall back to local implementation
  source ./CLIs/ci/bash/cmd/[command].sh
fi
```

### Claude Plugin

The Claude CLI plugin has been updated to use the external CI tool when available:

```bash
# claude init --with-ci my-project
if command -v ci-external &> /dev/null; then
  ci-external init "$PROJECT_NAME" [options]
else
  # Fall back to local implementation
fi
```

## Configuration

Projects can configure the external CI tool in their `.collaborative-intelligence.json`:

```json
{
  "integration_type": "embedded",
  "repository_path": "/path/to/CollaborativeIntelligence",
  "ci_external_path": "/custom/path/to/ci-tools",  // Optional
  "active_agents": ["Athena", "ProjectArchitect"],
  "fast_activation": true
}
```

## Environment Variables

- `CI_EXTERNAL_PATH`: Override the path to the external CI tool
- `CI_NO_DEPRECATION_NOTICE`: Disable deprecation notices
- `CI_FORCE_LOCAL`: Force the use of local implementation even if external is available

## Implementation Details

### External Tool Detection

The system uses this sequence to locate the external CI tool:

1. Check for `ci-external` in PATH
2. Check for `$HOME/.ci/bin/ci`
3. Check for `$CI_EXTERNAL_PATH/bin/ci`

### Command Delegation

1. The wrapper script builds the appropriate arguments for the external tool
2. Command execution is done via `exec` to replace the current process
3. Exit codes are properly propagated

### Backward Compatibility

During the transition period:

1. All command signatures remain identical
2. Output formats are maintained for scripts that parse CLI output
3. Configuration format is compatible between versions

## Transition Strategy

1. **Current Phase (3A)**: Preparation
   - Wrappers detect and use external tool if available
   - Local implementations remain functional

2. **Next Phase (3B)**: Migration
   - External repository functionality will be implemented
   - Integration guides will be updated

3. **Final Phase (3D)**: Completion
   - Local implementations will be removed
   - Wrappers will require the external tool

## Examples

### Command Usage

```bash
# Initialize a project
ci-init my-project              # Uses external tool if available

# Fix integration issues
ci-fix                          # Uses external tool if available

# Verify integration
ci-verify                       # Uses external tool if available

# Direct external tool usage
ci-external init my-project     # Always uses external tool
```

### CI Tool Installation

```bash
# Install CI tools
npm install -g @anthropics/ci-tools

# Verify installation
ci-external --version
ci --version  # Alias for ci-external
```

---

Document Version: 1.0  
Last Updated: May 21, 2025