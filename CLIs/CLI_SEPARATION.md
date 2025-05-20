# CLI Separation: External vs. Internal

This document provides guidance on the separation between the external integration CLI (CI) and the internal maintenance CLI (TL).

## Purpose and Scope

### External CLI (CI)

The external CLI, maintained at `../CI`, serves as the integration point for other projects to use the Collaborative Intelligence system:

- **Primary Purpose**: Allow external projects to integrate with and leverage Collaborative Intelligence
- **Users**: Developers integrating CI into their own projects
- **Key Functionality**: Project initialization, integration, verification, agent activation

### Internal CLI (TL)

The internal CLI, maintained in this repository as `TL`, is for maintaining and working with the Collaborative Intelligence repository itself:

- **Primary Purpose**: Internal repository maintenance and development
- **Users**: Collaborative Intelligence developers and maintainers
- **Key Functionality**: Agent management, memory operations, knowledge registry, advanced debugging

## Command Scope Separation

Each CLI should have a clearly defined scope of commands to avoid confusion:

### External CI Commands

```
ci
├── init [project_name]          # Initialize a new project with CI integration
├── integrate [path]             # Integrate CI into an existing project
├── status [path]                # Check CI integration status
├── verify [path]                # Verify CI integration
├── intelligence
│   ├── agent <name>             # Basic agent activation (limited scope)
│   └── memory <commands>        # Basic memory operations
├── system
│   ├── update                   # Update CI configuration
│   └── config                   # Configure CI settings
└── lifecycle
    ├── create                   # Create project resources
    └── clean                    # Clean project artifacts
```

### Internal TL Commands

```
tl
├── agent
│   ├── activate                 # Activate an agent with detailed options
│   ├── create                   # Create a new agent
│   ├── validate                 # Validate agent structure
│   └── list                     # List available agents
├── memory
│   ├── load                     # Load agent memory
│   ├── validate                 # Validate memory files
│   ├── cache                    # Manage memory cache
│   └── stats                    # Display memory statistics
├── knowledge
│   ├── search                   # Search the knowledge registry
│   ├── index                    # Index knowledge content
│   └── distribute               # Distribute knowledge
├── debug
│   ├── trace                    # Trace command execution
│   ├── inspect                  # Inspect system state
│   └── profile                  # Profile performance
└── system
    ├── optimize                 # Optimize system performance
    └── report                   # Generate system reports
```

## Naming Conventions

To maintain clear separation:

1. **Binary Names**:
   - External: Always use `ci` (lowercase)
   - Internal: Always use `tl` (lowercase)

2. **Package Names**:
   - External: `collaborative-intelligence-cli`
   - Internal: `teaching-learning-cli` or `tl-cli`

3. **Module Names**:
   - External: Use `ci::*` namespaces
   - Internal: Use `tl::*` namespaces

4. **Config Files**:
   - External: `.collaborative-intelligence.json`
   - Internal: `.tl-config.json`

## Shared Utilities

Some utilities can be shared between both CLIs:

1. **Styling and Formatting**: Maintain consistent visual style across both CLIs
2. **Error Handling**: Use the same error handling approach
3. **Base Utilities**: Path resolution, file operations, etc.
4. **Documentation Generation**: Use the same documentation approach

These should be maintained in a shared library accessible to both CLIs.

## Implementation Considerations

When implementing the separation:

1. **Fix CIR References**: All instances of "CIR" should be changed back to "CI" in the external CLI
2. **No Cross-Dependencies**: The external CLI should not depend on internal tooling
3. **Documentation**: Clearly document which CLI should be used for which purpose
4. **Testing**: Test both CLIs independently to ensure they function correctly
5. **Backwards Compatibility**: Ensure existing scripts referencing the CLIs continue to work

## Migration Path

1. First, focus on the external CI CLI, ensuring it has a clean implementation
2. Then, develop the internal TL CLI using lessons learned from the external CLI
3. Finally, coordinate the release of both to ensure consistency

## Examples

### Example External CI Usage

```bash
# Initialize a new project
ci init my-project

# Verify integration
ci verify

# Activate the Athena agent
ci intelligence agent Athena
```

### Example Internal TL Usage

```bash
# List all available agents
tl agent list

# Validate memory for the Gaia agent
tl memory validate Gaia

# Search the knowledge registry
tl knowledge search "memory optimization"
```

By maintaining this clear separation, both CLIs can evolve independently while preserving a consistent user experience.