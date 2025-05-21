# CI CLI Migration Implementation Plan

## Overview

This document outlines the plan to complete the external CI CLI utility and ensure it can reliably access CollaborativeIntelligence repository functionality from any external project location.

## Current Status Assessment

1. **Two CLI Systems**:
   - **External CI Utility** (`../CI`): Standalone utility tool that enables external projects to integrate with and use CollaborativeIntelligence repository functionality (agents, knowledge base, memory systems)
   - **Internal TL Tool** (`CLIs/tl`): Repository-specific maintenance tool for internal operations

2. **Implementation Issues**:
   - The external CI utility needs better integration with this repository's systems
   - Environment variable conflicts affecting repository detection  
   - The external utility CLI behaves differently when run inside vs outside this repository
   - Repository detection mechanism needs enhancement for reliable access to CollaborativeIntelligence functionality

3. **Naming Resolution Status**:
   - CIR to CI renaming has mostly been completed 
   - Some potential residual references might still exist

## Migration Action Plan

### 1. Environment Configuration

- [x] **Standardize Environment Variables**:
   ```bash
   # Ensure CI_PATH is properly set in ~/.zshrc
   export CI_PATH=/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence
   
   # Remove any CI_REPO_PATH variables that might be causing conflicts
   # Comment out or remove from ~/.zshrc if present
   ```

- [ ] **Create CI_PATH Installation Process**:
   - [ ] Update installation scripts to automatically configure this environment variable
   - [ ] Include validation to prevent path conflicts

### 2. Complete CIR to CI Migration

- [ ] **Verify Removal of CIR References**:
   - [ ] Run the `remove_cir_references.sh` script again with enhanced patterns
   - [ ] Conduct thorough grep searches for any remaining "CIR" references
   - [ ] Check shell scripts, configuration files, and documentation

- [ ] **Update Related Variable Names**:
   - [ ] Ensure consistent naming in environment variables
   - [ ] Update any internal function names using CIR

### 3. External CLI Utility Enhancement  

- [ ] **Complete Utility Integration**:
   - [ ] Enhance external CI utility's ability to access CollaborativeIntelligence repository systems
   - [ ] Ensure reliable connection to agent system, knowledge base, and memory systems
   - [ ] Implement proper authentication/authorization for external access

- [ ] **Command Implementation**:
   - [ ] Complete implementations for `init` command
   - [ ] Complete implementations for `integrate` command
   - [ ] Complete implementations for `status` command
   - [ ] Complete implementations for `verify` command
   - [ ] Ensure all commands work with explicit `--ci-path` parameters

- [ ] **Path Resolution Enhancement**:
   - [ ] Improve the repository detection logic in the external CLI
   - [ ] Add more flexible path resolution mechanisms
   - [ ] Implement proper fallbacks for when standard paths aren't found

### 4. Repository Detection Improvement

- [x] **Enhance Detection Algorithm**:
   - [x] Enhanced `detect_external_ci()` to find CI CLI in more locations
   - [x] Added support for local build paths in common directories
   - [x] Added better handling of CI executable in PATH

- [x] **Configuration Loading Robustness**:
   - [x] Enhanced `execute_external_ci()` to pass `CI_PATH` environment variable
   - [x] Added better deprecation notice with installation guidance

### 5. Documentation Updates

- [ ] **Usage Documentation**:
   - [ ] Create clear instructions for external repository usage
   - [ ] Document the `--ci-path` flag importance
   - [ ] Explain environment variable setup

- [ ] **Installation Documentation**:
   - [ ] Document the required environment configuration
   - [ ] Create step-by-step setup guides

## Implementation Steps

- [ ] **Environment Variable Setup**
```bash
# 1. Environment Variable Setup
echo 'export CI_PATH=/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence' >> ~/.zshrc
# Comment out or remove CI_REPO_PATH from ~/.zshrc if present
```

- [ ] **External CLI Enhancement**
```bash
# 2. External CLI Enhancement
cd ../CI
# Integrate functionality from CollaborativeIntelligence/CLIs/ci/rust/
cp -r /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CLIs/ci/rust/src/helpers/terminal.rs src/helpers/terminal_enhanced.rs
```

- [ ] **Complete Command Implementation**
```bash
# 3. Complete Command Implementation in ../CI
# Implement fully functional commands instead of placeholders
```

- [ ] **Create Installation Script**
```bash
# 4. Create Installation Script
cat > install.sh << EOF
#!/bin/bash
# Install the CI CLI and set up environment variables

# Build the CLI
cargo build --release

# Create symlink in /usr/local/bin
sudo ln -sf "$(pwd)/target/release/ci" /usr/local/bin/ci

# Add CI_PATH to user's shell configuration
SHELL_CONFIG=~/.zshrc  # or ~/.bashrc
echo "# CI CLI configuration" >> $SHELL_CONFIG
echo "export CI_PATH=$(pwd)" >> $SHELL_CONFIG

echo "Installation complete! Please restart your terminal or run 'source $SHELL_CONFIG'"
EOF
chmod +x install.sh
```

- [ ] **Create Test Project for Validation**
```bash
# 5. Create Test Project for Validation
mkdir -p ~/test-ci-project
cd ~/test-ci-project
# Test CI CLI integration using explicit path flag
ci init test-project --ci-path=/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence
```

## External CLI Integration Testing

- [ ] **Basic Connection Test**:
   - [ ] Test repository detection using environment variable
   - [ ] Test repository detection using `--ci-path` flag
   - [ ] Test with various relative and absolute paths

- [ ] **Command Functionality Test**:
   - [ ] Verify `init` command creates proper project structure
   - [ ] Verify `integrate` adds CI to existing projects
   - [ ] Verify `status` and `verify` report correctly

- [ ] **Error Handling Test**:
   - [ ] Test behavior when repository not found
   - [ ] Test with malformed repository structure
   - [ ] Test with missing components

## Progress Updates

| Date | Task | Status | Notes |
|------|------|--------|-------|
| 2025-05-21 | Initial plan creation | Completed | Created migration plan document |
| 2025-05-21 | Added progress tracking with checkboxes | Completed | Enhanced document with todo items |
| 2025-05-21 | Environment variable analysis | Completed | Verified no CI_REPO_PATH references in codebase, only CI_PATH is used |
| 2025-05-21 | Enhanced External CLI detection | Completed | Updated `detect_external_ci()` to check more paths and handle local builds |
| 2025-05-21 | Improved CI_PATH handling | Completed | Enhanced `execute_external_ci()` to pass CI_PATH to external CLI |
| 2025-05-21 | Added better installation guidance | Completed | Updated deprecation notice with detailed installation steps |

## Conclusion

By completing this implementation, we'll ensure the external CI utility can reliably access and use CollaborativeIntelligence repository functionality from any external project location. The focus is on enhancing the external `../CI` utility tool to provide seamless integration with the agent system, knowledge base, and collaborative intelligence features.

The external utility will be improved to reliably connect to the CollaborativeIntelligence repository regardless of where it's executed from, enabling external projects to leverage the full power of the collaborative intelligence system.