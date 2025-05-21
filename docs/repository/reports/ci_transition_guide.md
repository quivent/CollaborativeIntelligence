# CI Transition Guide for Dependent Projects

This guide provides information for projects that depend on the CollaborativeIntelligence (CI) system about the migration of the CI functionality to an external repository.

## Overview

As part of our CLI modernization effort, we are moving the external project integration functionality from the CollaborativeIntelligence repository to a dedicated CI Tools repository. This separation allows for:

1. Clearer separation of concerns
2. Independent evolution of both tools
3. Simplified maintenance
4. Better dependency management

## Timeline

- **Phase 1**: CLI Modernization (Completed)
- **Phase 2**: CIR to CI Naming Standardization (Completed)
- **Phase 3A**: External Repository Preparation (Current)
- **Phase 3B**: Functional Migration (Upcoming)
- **Phase 3C**: Transition Period (1-2 months)
- **Phase 3D**: Full Migration (By end of 2025)

## Changes for Dependent Projects

### During Transition Period (Now through Phase 3C)

The transition will be transparent to most dependent projects. The existing scripts will:

1. Detect if the external CI tool is installed
2. Use the external tool if available
3. Fall back to the local implementation if not available
4. Show a deprecation notice when using the local implementation

**No action is required** during this period, though installing the external CI tool is recommended.

### After Full Migration (Phase 3D)

Once the full migration is complete:

1. The local implementations will be removed
2. The wrapper scripts will require the external CI tool
3. Projects that haven't migrated will need to install the external tool

## How to Install the External CI Tool

```bash
# Using npm
npm install -g @anthropics/ci-tools

# Verify installation
ci-external --version
```

## Compatibility Guarantees

We are committed to maintaining compatibility during and after the transition:

1. **Command Signatures**: All commands will maintain the same signature and behavior
2. **Configuration Files**: Existing .collaborative-intelligence.json files will remain compatible
3. **Claude Integration**: The Claude CLI plugin will continue to work as before
4. **Project Structure**: Existing project structures will not need modification

## Updating Existing Scripts

If your project uses CI commands in scripts or automation:

```bash
# Before
ci-init my-project

# After - Option 1 (Recommended)
ci-external init my-project

# After - Option 2 (Still supported during transition)
ci-init my-project
```

## Advanced Configuration

For projects with custom CI integrations, you may need to update paths:

```json
// Before
{
  "integration_type": "embedded",
  "repository_path": "/path/to/CollaborativeIntelligence"
}

// After
{
  "integration_type": "embedded",
  "repository_path": "/path/to/CollaborativeIntelligence",
  "ci_external_path": "/path/to/ci-tools"  // Optional
}
```

## Troubleshooting

If you encounter issues during the transition:

1. **Command not found**: Install the external CI tool
2. **Compatibility issues**: Check for version mismatches
3. **Configuration errors**: Verify .collaborative-intelligence.json format

## Support

For assistance with the transition:

- **Documentation**: https://github.com/anthropics/ci-tools/docs
- **Issues**: https://github.com/anthropics/ci-tools/issues
- **Contact**: citools-support@anthropic.com

## Conclusion

This transition will improve the developer experience and maintainability of the CI tools while ensuring a smooth migration path for dependent projects.

---

**Document Date**: May 21, 2025  
**Last Updated**: May 21, 2025