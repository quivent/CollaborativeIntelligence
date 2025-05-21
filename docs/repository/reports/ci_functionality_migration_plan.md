# CI Functionality Migration Plan

## Overview

This document outlines the plan for Phase 3 of the CLI modernization effort, focusing on migrating integration functionality to the external CI repository while maintaining backward compatibility during the transition period.

## Migration Analysis

### Components to Migrate

Based on analysis of the current codebase, the following components should be migrated to the external CI repository:

1. **Project Initialization**
   - `ci-init` script
   - `CLIs/ci/rust/src/commands/init.rs`
   - `CLIs/plugins/claude/claude-init-impl.sh`

2. **Integration Commands**
   - `ci-fix` script
   - `CLIs/ci/rust/src/commands/fix.rs`
   - `CLIs/ci/rust/src/commands/integrate.rs`

3. **Verification Commands**
   - `ci-verify` script
   - `CLIs/ci/rust/src/commands/verify.rs`
   - `CLIs/ci/rust/src/commands/status.rs`

4. **Support Libraries**
   - `CLIs/ci/rust/src/helpers/` directory
   - `CLIs/ci/bash/lib/` directory

5. **Configuration Management**
   - `CLIs/ci/rust/src/config/` directory
   - `.collaborative-intelligence.json` structure

### Components to Retain

The following components should remain in this repository:

1. **TL CLI System**
   - Complete `CLIs/tl/` directory
   - All internal repository management commands

2. **Transition Shims**
   - Wrapper scripts that delegate to the external CI repository
   - Compatibility layer for existing integration

3. **Documentation**
   - Usage guides for both CLI systems
   - Clear separation explanation

## Implementation Strategy

### 1. External Repository Setup

1. **Create External CI Repository**
   - Initialize new repository structure
   - Prepare directory structure aligned with CLI architecture

2. **Mirror Command Structure**
   - Replicate the command modules structure
   - Ensure command signature compatibility

### 2. Functionality Migration

1. **Project Initialization**
   - Move core initialization code to external repository
   - Create wrapper in this repository that delegates to external CLI

2. **Integration Commands**
   - Move fix and integrate commands to external repository
   - Create compatible command wrappers

3. **Verification**
   - Move verification logic to external repository
   - Maintain compatible interfaces

4. **Support Libraries**
   - Migrate helpers while ensuring compatibility
   - Extract shared functionality as needed

### 3. Update Dependencies

1. **Update Root Scripts**
   - Modify `ci-init`, `ci-fix`, and `ci-verify` to detect and use external CLI
   - Add fallback mechanisms for backward compatibility

2. **Update Plugin Integration**
   - Modify `claude-init-impl.sh` to use external CI CLI
   - Maintain support for existing projects

### 4. Documentation Updates

1. **Update User Guides**
   - Document migration process for users
   - Create transition guides for dependent projects

2. **API Documentation**
   - Document all external interfaces
   - Clarify integration points

## Compatibility Strategy

To ensure a smooth transition:

1. **Wrapper Pattern**
   - Create wrapper scripts that detect external CI installation
   - Use external CI when available, fall back to local implementation otherwise
   - Provide consistent interface regardless of implementation used

2. **API Stability**
   - Maintain identical command signatures during migration
   - Ensure compatible output formats for scripts that parse CLI output

3. **Configuration Compatibility**
   - Support both old and new configuration formats
   - Provide migration utilities for existing projects

4. **User Notification**
   - Add deprecation notices to commands using the local implementation
   - Guide users to install the external CLI

## Migration Timeline

1. **Phase 3A: Preparation (Current)**
   - Create migration plan
   - Update current repository to reference external repo
   - Implement wrapper pattern

2. **Phase 3B: Initial Migration**
   - Create external repository structure
   - Migrate core functionality
   - Implement compatibility layer

3. **Phase 3C: Transition Period**
   - Maintain dual implementation support
   - Gather feedback from users
   - Refine external implementation

4. **Phase 3D: Completion**
   - Remove duplicate functionality from this repository
   - Keep only wrapper scripts and TL CLI
   - Update all documentation to reference external CLI

## Testing Strategy

1. **Compatibility Testing**
   - Test wrappers with and without external CLI installed
   - Verify identical behavior in both scenarios

2. **Integration Testing**
   - Test with real projects that use the CI integration
   - Verify commands work correctly with various configurations

3. **Error Handling**
   - Test fallback mechanisms
   - Verify appropriate error messages when external CLI is required but not available

## Next Steps

1. Create wrapper scripts for `ci-init`, `ci-fix`, and `ci-verify`
2. Update plugin integration to support external CI CLI
3. Update documentation to reflect new architecture
4. Begin external repository setup

## Conclusion

This migration plan enables a clean separation between the CI and TL tools while maintaining backward compatibility. The external CI repository will handle all integration functionality, while this repository will focus on internal repository management through the TL CLI.

---

**Report Date**: May 21, 2025  
**Created By**: Topologist