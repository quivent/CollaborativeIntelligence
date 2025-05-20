# Phase 1 Implementation: Naming Correction and CLI Separation

This document outlines the implementation steps for Phase 1 of the CLI migration plan, focusing on correcting the CIR to CI naming and establishing clear separation between external and internal CLI tools.

## 1. CIR to CI Naming Correction

### Files to Update

Based on our initial search, the following files need updating:

1. **Documentation**:
   - `/CLAUDE.md` - Multiple CIR references (configuration directives)
   - `/.cir/agents/Athena.md` - CIR project references

2. **Directory Structure**:
   - Rename `/.cir` directory to `/.ci`

For the parent directory (`../CI`), we need to:
1. Create a catalog of all CIR references
2. Systematically rename all instances back to CI
3. Update any internal namespaces or module names

### Implementation Steps

1. **Update CLAUDE.md**:
   - Change all occurrences of `CIR` to `CI`
   - Update all directives (e.g., `_CIR.config` to `_CI.config`)
   - Revise explanatory text

2. **Directory Renaming**:
   - Rename `.cir` to `.ci`
   - Update all references to this directory in other files

3. **External CI Repository**:
   - Work with the parent directory to identify and fix all CIR references
   - Ensure binary names and packages use CI instead of CIR
   - Update namespaces and module structures

## 2. CLI Separation Implementation

### Create External CLI Structure (../CI)

The external CLI should be focused on project integration:

1. **Main Commands**:
   - `init`: Initialize a project with CI
   - `integrate`: Add CI to an existing project
   - `status`: Check integration status
   - `verify`: Verify CI integration

2. **Command Structure**:
   ```
   ci
   ├── init [project_name]
   ├── integrate [path]
   ├── status [path]
   ├── verify [path]
   ├── intelligence
   │   ├── agent <name>
   │   └── memory <commands>
   ├── system
   │   ├── update
   │   └── config
   └── lifecycle
       ├── create
       └── clean
   ```

### Create Internal CLI Structure (TL)

The internal CLI should focus on repository maintenance:

1. **Main Commands**:
   - `agent`: Agent management
   - `memory`: Memory operations
   - `knowledge`: Knowledge registry
   - `debug`: Debugging tools
   - `system`: System optimization

2. **Command Structure**:
   ```
   tl
   ├── agent
   │   ├── activate
   │   ├── create
   │   ├── validate
   │   └── list
   ├── memory
   │   ├── load
   │   ├── validate
   │   ├── cache
   │   └── stats
   ├── knowledge
   │   ├── search
   │   ├── index
   │   └── distribute
   ├── debug
   │   ├── trace
   │   ├── inspect
   │   └── profile
   └── system
       ├── optimize
       └── report
   ```

### Shared Components

Create shared utility libraries for both CLIs:

1. **Output Formatting**:
   - Terminal styling
   - Progress indicators
   - Error formatting

2. **Configuration Management**:
   - Path resolution
   - Config file handling

3. **Error Handling**:
   - Consistent error types
   - Error propagation

## 3. Implementation Plan

### Week 1: CIR to CI Naming Correction

1. Update documentation files
2. Rename directories
3. Work with external CI repository to update all references

### Week 2: External CLI Structure

1. Define command modules
2. Implement shared utilities
3. Create proper command registration

### Week 3: Internal CLI Structure

1. Create the TL binary
2. Define command structure
3. Implement shared functionality

### Week 4: Integration Testing

1. Verify both CLIs function correctly
2. Ensure no naming inconsistencies remain
3. Document the new CLI structures

## 4. Success Criteria

Phase 1 will be considered complete when:

1. All CIR references have been changed to CI
2. The external CLI (CI) and internal CLI (TL) have clear, separate structures
3. Common utilities are shared between both CLIs
4. Documentation is updated to reflect the changes
5. Both CLIs can be built and run successfully

## 5. Next Steps

After completing Phase 1, we will move to Phase 2: Core Structure Alignment, focusing on:

1. Creating consistent command structure across both CLIs
2. Standardizing error handling and logging
3. Implementing common configuration management
4. Consolidating styling and display elements
5. Creating shared utility libraries