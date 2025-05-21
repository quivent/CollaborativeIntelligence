# CI Migration Validation Test Suite

## Overview

This document outlines a comprehensive test suite for validating the migration of CI functionality to the external repository. The validation suite ensures that both functionality and compatibility are maintained throughout the transition period.

## Test Categories

The validation suite is organized into four main categories:

1. **Functionality Tests**: Ensure all features work correctly in the external repository
2. **Compatibility Tests**: Verify backward compatibility with existing projects 
3. **Integration Tests**: Test the integration between local and external implementations
4. **Error Handling Tests**: Validate error detection and recovery

## 1. Functionality Tests

### 1.1 Command Execution Tests

| Test ID | Description | Input | Expected Output |
|---------|-------------|-------|----------------|
| F-CMD-001 | Basic init command | `ci init test-project` | Successfully creates project with default settings |
| F-CMD-002 | Init with custom agents | `ci init test-project --agents "Athena,Architect"` | Project initialized with specified agents |
| F-CMD-003 | Init with custom integration | `ci init test-project --integration embedded` | Project initialized with embedded integration |
| F-CMD-004 | Status command | `ci status` | Correctly reports project status |
| F-CMD-005 | Integrate command | `ci integrate` | Successfully integrates with existing project |
| F-CMD-006 | Fix command | `ci fix` | Fixes common issues in the project |
| F-CMD-007 | Verify command | `ci verify` | Correctly verifies project setup |
| F-CMD-008 | Load command | `ci load Architect` | Successfully loads specified agent |
| F-CMD-009 | Map command | `ci map` | Correctly maps repository structure |

### 1.2 Configuration Tests

| Test ID | Description | Input | Expected Output |
|---------|-------------|-------|----------------|
| F-CFG-001 | Default configuration | Run without config | Uses default settings |
| F-CFG-002 | Custom config file | `ci --config custom.json init test` | Uses settings from custom config |
| F-CFG-003 | Environment variables | Set `CI_PATH` | Respects environment variable settings |
| F-CFG-004 | Project configuration | Edit project config | Changes reflected in behavior |
| F-CFG-005 | Configuration persistence | Create config, restart | Configuration maintained between runs |

### 1.3 Template Management Tests

| Test ID | Description | Input | Expected Output |
|---------|-------------|-------|----------------|
| F-TPL-001 | Default templates | Standard init | Correct CLAUDE.md generated |
| F-TPL-002 | Custom templates | Custom template dir | Uses custom templates |
| F-TPL-003 | Template variables | Init with custom name | Variables correctly replaced |
| F-TPL-004 | Template conditions | Different integration types | Conditional sections handled correctly |

### 1.4 Repository Detection Tests

| Test ID | Description | Input | Expected Output |
|---------|-------------|-------|----------------|
| F-REP-001 | Git repository | Init in git repo | Detects git properly |
| F-REP-002 | Non-git directory | Init in regular dir | Works without git |
| F-REP-003 | Repository with existing CI | Add to CI project | Detects and integrates properly |
| F-REP-004 | Nested repositories | Init in nested structure | Handles correctly |

## 2. Compatibility Tests

### 2.1 Project Compatibility Tests

| Test ID | Description | Input | Expected Output |
|---------|-------------|-------|----------------|
| C-PRJ-001 | Legacy project format | Old-style project | Successfully works with old format |
| C-PRJ-002 | Mixed format project | Partially updated | Handles mixed format correctly |
| C-PRJ-003 | New format project | Latest format | Works with new format |
| C-PRJ-004 | Migration between formats | Format migration | Successfully migrates between formats |

### 2.2 Command Compatibility Tests

| Test ID | Description | Input | Expected Output |
|---------|-------------|-------|----------------|
| C-CMD-001 | Old command syntax | Legacy commands | Accepts and processes correctly |
| C-CMD-002 | New command syntax | New style commands | Processes correctly |
| C-CMD-003 | Mixed command syntax | Mixed old/new | Handles appropriately |
| C-CMD-004 | Deprecated options | Old options | Works but shows deprecation notice |

### 2.3 Path Compatibility Tests

| Test ID | Description | Input | Expected Output |
|---------|-------------|-------|----------------|
| C-PTH-001 | Absolute paths | Use absolute paths | Works correctly |
| C-PTH-002 | Relative paths | Use relative paths | Works correctly |
| C-PTH-003 | Symlinked paths | Use symlinks | Resolves correctly |
| C-PTH-004 | Special characters | Paths with spaces, etc. | Handles properly |

### 2.4 Environment Compatibility Tests

| Test ID | Description | Input | Expected Output |
|---------|-------------|-------|----------------|
| C-ENV-001 | Windows environment | Run on Windows | Functions correctly |
| C-ENV-002 | macOS environment | Run on macOS | Functions correctly |
| C-ENV-003 | Linux environment | Run on Linux | Functions correctly |
| C-ENV-004 | Different shells | Bash, zsh, PowerShell | Works in all shells |

## 3. Integration Tests

### 3.1 Local/External Integration Tests

| Test ID | Description | Input | Expected Output |
|---------|-------------|-------|----------------|
| I-INT-001 | External detection | External installed | Correctly detects and uses |
| I-INT-002 | Fallback to local | No external | Falls back to local smoothly |
| I-INT-003 | Force local mode | Use `CI_FORCE_LOCAL=1` | Uses local regardless of external |
| I-INT-004 | Switch between modes | Install/uninstall external | Handles transitions correctly |

### 3.2 Plugin Integration Tests

| Test ID | Description | Input | Expected Output |
|---------|-------------|-------|----------------|
| I-PLG-001 | Claude plugin | Use claude-init-impl | Works with Claude plugin |
| I-PLG-002 | Other plugins | Use other plugins | Functions with all plugins |
| I-PLG-003 | Custom plugins | Add custom plugin | Integrates properly |
| I-PLG-004 | Plugin chain | Multiple plugins | Handles plugin chain correctly |

### 3.3 Tool Integration Tests

| Test ID | Description | Input | Expected Output |
|---------|-------------|-------|----------------|
| I-TL-001 | CI and TL interaction | Use both tools | Tools work together correctly |
| I-TL-002 | Shared configuration | Change shared config | Both tools respect changes |
| I-TL-003 | Independent operation | Use separately | Can be used independently |
| I-TL-004 | Command delegation | Delegated commands | Proper delegation between tools |

### 3.4 Repository Integration Tests

| Test ID | Description | Input | Expected Output |
|---------|-------------|-------|----------------|
| I-REP-001 | CollaborativeIntelligence repo | Use in primary repo | Works correctly |
| I-REP-002 | External projects | Use in other repos | Works correctly |
| I-REP-003 | No repository | Use outside repo | Functions appropriately |
| I-REP-004 | Nested repositories | Use in nested repo | Handles nesting correctly |

## 4. Error Handling Tests

### 4.1 Input Validation Tests

| Test ID | Description | Input | Expected Output |
|---------|-------------|-------|----------------|
| E-VAL-001 | Invalid command | `ci invalid-command` | Clear error message |
| E-VAL-002 | Missing required args | `ci init` (no name) | Helpful error message |
| E-VAL-003 | Invalid option values | Invalid integration type | Meaningful error |
| E-VAL-004 | Conflicting options | Mutually exclusive options | Clear error message |

### 4.2 File System Error Tests

| Test ID | Description | Input | Expected Output |
|---------|-------------|-------|----------------|
| E-FS-001 | Permission errors | Read-only directory | Helpful error message |
| E-FS-002 | File not found | Missing template | Clear error, fallback if possible |
| E-FS-003 | Path too long | Extremely long paths | Proper handling and error |
| E-FS-004 | Disk full | Write when disk full | Graceful error handling |

### 4.3 Configuration Error Tests

| Test ID | Description | Input | Expected Output |
|---------|-------------|-------|----------------|
| E-CFG-001 | Invalid config format | Malformed JSON | Clear error message |
| E-CFG-002 | Missing config values | Incomplete config | Uses defaults, warns |
| E-CFG-003 | Config type mismatch | Wrong value types | Meaningful error |
| E-CFG-004 | Conflicting configs | Conflicting settings | Clear resolution strategy |

### 4.4 Network Error Tests

| Test ID | Description | Input | Expected Output |
|---------|-------------|-------|----------------|
| E-NET-001 | External tool unreachable | Path exists but not executable | Clear error, fallback |
| E-NET-002 | Repository unavailable | Git remote down | Graceful handling |
| E-NET-003 | Intermittent failures | Random connection drop | Retry and recover |
| E-NET-004 | Timeout | Slow response | Proper timeout handling |

## Test Implementation

### Automated Test Implementation

The validation suite will be implemented as automated tests using Rust's testing framework. The following approach will be used:

1. **Unit Tests**: Test individual components in isolation
   - Located in `tests/` directory in the CI repository
   - Organized by component (commands, config, core, etc.)
   - Use mocks for external dependencies

2. **Integration Tests**: Test multiple components together
   - Located in `tests/integration/` directory
   - Test end-to-end workflows
   - Minimal mocking, use real file system when possible

3. **Compatibility Tests**: Verify backward compatibility
   - Create test fixtures for different project formats
   - Test with different environment variables and configurations
   - Include cross-platform tests

### Test Harness

The test harness will provide:

1. **Fixture Management**: Creating and cleaning up test environments
2. **Environment Control**: Setting up controlled test environments
3. **Result Verification**: Comparing actual output with expected output
4. **Error Injection**: Deliberately causing errors to test handling

```rust
// Example test fixture manager
pub struct TestFixture {
    pub root_dir: PathBuf,
    pub project_dir: PathBuf,
    cleanup_on_drop: bool,
}

impl TestFixture {
    pub fn new() -> Result<Self> {
        let root_dir = tempfile::tempdir()?.into_path();
        let project_dir = root_dir.join("test-project");
        std::fs::create_dir_all(&project_dir)?;
        
        Ok(Self {
            root_dir,
            project_dir,
            cleanup_on_drop: true,
        })
    }
    
    pub fn with_template(&self, template_name: &str, content: &str) -> Result<PathBuf> {
        let template_dir = self.root_dir.join("templates");
        std::fs::create_dir_all(&template_dir)?;
        
        let template_path = template_dir.join(template_name);
        std::fs::write(&template_path, content)?;
        
        Ok(template_path)
    }
    
    pub fn with_config(&self, config: &serde_json::Value) -> Result<PathBuf> {
        let config_dir = self.project_dir.join(".ci");
        std::fs::create_dir_all(&config_dir)?;
        
        let config_path = config_dir.join("config.json");
        std::fs::write(&config_path, serde_json::to_string_pretty(config)?)?;
        
        Ok(config_path)
    }
    
    pub fn run_command(&self, args: &[&str]) -> Result<(ExitStatus, String, String)> {
        let mut command = Command::new("ci");
        command.args(args).current_dir(&self.project_dir);
        
        let output = command.output()?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        Ok((output.status, stdout, stderr))
    }
}

impl Drop for TestFixture {
    fn drop(&mut self) {
        if self.cleanup_on_drop {
            let _ = std::fs::remove_dir_all(&self.root_dir);
        }
    }
}
```

### Test Matrix

To ensure comprehensive coverage, tests will be run across a matrix of:

1. **Platforms**: Windows, macOS, Linux
2. **Environment Configurations**: 
   - With external CI installed
   - Without external CI
   - Various environment variable combinations
3. **Project Types**:
   - New projects
   - Existing projects with legacy format
   - Projects in various stages of migration

### Continuous Integration

Tests will be run automatically in CI/CD pipelines:

1. **Pull Request Testing**: Run core tests for every PR
2. **Nightly Testing**: Run complete test suite nightly
3. **Release Testing**: Run complete test suite before releases
4. **Cross-Platform Testing**: Test on all supported platforms

## Manual Testing Procedures

Some aspects require manual testing to ensure a smooth user experience:

### Installation Testing

1. Install from various sources (cargo, script, etc.)
2. Verify PATH setup
3. Check permissions
4. Test uninstallation and cleanup

### Usability Testing

1. Follow documentation examples
2. Test help output clarity
3. Verify error message helpfulness
4. Check command completion experience

### Upgrade Testing

1. Test upgrading from various previous versions
2. Verify configuration preservation
3. Check for deprecated feature warnings
4. Test downgrade scenarios

## Test Reports

Test results will be documented in standardized reports:

1. **Automated Test Reports**: Generated by CI/CD pipeline
2. **Compatibility Matrix**: Shows compatibility across environments
3. **Coverage Report**: Shows test coverage statistics
4. **Regression Analysis**: Compares performance with previous versions

## Validation Checklist

Before completing the migration, verify:

- [ ] All automated tests pass
- [ ] Manual testing completed successfully
- [ ] All features implemented and verified
- [ ] Backward compatibility maintained
- [ ] Error handling verified
- [ ] Performance benchmarks acceptable
- [ ] Documentation updated
- [ ] Upgrade path tested

## Implementation Plan

1. **Week 1: Test Framework Setup**
   - Create test harness
   - Implement fixture management
   - Set up CI/CD integration

2. **Week 2: Core Test Implementation**
   - Implement functionality tests
   - Implement basic compatibility tests
   - Create initial test fixtures

3. **Week 3: Comprehensive Testing**
   - Implement integration tests
   - Implement error handling tests
   - Create cross-platform test strategy

4. **Week 4: Validation and Refinement**
   - Run complete test suite
   - Address issues and edge cases
   - Finalize test documentation
   - Create reporting framework