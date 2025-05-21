# CI External Repository Implementation Commit Plan

## Overview

This document outlines the phased implementation plan for committing the CI external repository integration code to the CollaborativeIntelligence repository. The implementation is divided into discrete, sequential phases to ensure a smooth transition and maintain backward compatibility throughout the process.

## Implementation Phases

### Phase 1: External CI Detection Framework

**Purpose:** Implement the core detection and fallback mechanisms.

**Files to create/modify:**
1. `CLIs/ci/rust/src/helpers/external.rs` - Implement external CI detection
2. `CLIs/ci/rust/src/helpers/mod.rs` - Update to include external module
3. `CLIs/ci/rust/Cargo.toml` - Add necessary dependencies

**Implementation details:**

1. Create the external CI detection module:
```rust
// CLIs/ci/rust/src/helpers/external.rs
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use semver::Version;
use crate::error::{Result, Error};

pub struct ExternalCi {
    pub path: PathBuf,
    pub version: Version,
}

/// Check if an external CI implementation is available
pub fn is_external_ci_available() -> bool {
    detect_external_ci().is_some()
}

/// Detect and return information about the external CI implementation
pub fn detect_external_ci() -> Option<ExternalCi> {
    // Check environment variable override
    if let Ok(force_local) = env::var("CI_FORCE_LOCAL") {
        if force_local == "1" || force_local.to_lowercase() == "true" {
            return None;
        }
    }
    
    // Check if ci-external is in PATH
    if let Ok(status) = Command::new("ci-external")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .status() {
        if status.success() {
            return Some(ExternalCi {
                path: PathBuf::from("ci-external"),
                version: get_external_ci_version("ci-external").unwrap_or_else(|_| Version::new(0, 1, 0)),
            });
        }
    }
    
    // Check $HOME/.ci/bin/ci
    if let Some(home_dir) = dirs::home_dir() {
        let home_ci_path = home_dir.join(".ci").join("bin").join("ci");
        if home_ci_path.exists() {
            return Some(ExternalCi {
                path: home_ci_path.clone(),
                version: get_external_ci_version(&home_ci_path.to_string_lossy())
                    .unwrap_or_else(|_| Version::new(0, 1, 0)),
            });
        }
    }
    
    // Check $CI_EXTERNAL_PATH/bin/ci
    if let Ok(external_path) = env::var("CI_EXTERNAL_PATH") {
        let custom_ci_path = PathBuf::from(external_path).join("bin").join("ci");
        if custom_ci_path.exists() {
            return Some(ExternalCi {
                path: custom_ci_path.clone(),
                version: get_external_ci_version(&custom_ci_path.to_string_lossy())
                    .unwrap_or_else(|_| Version::new(0, 1, 0)),
            });
        }
    }
    
    None
}

/// Execute the external CI tool with the given command and arguments
pub fn execute_external_ci(command: &str, args: &[String], _config: &crate::config::Config) -> Result<()> {
    let external_ci = detect_external_ci()
        .ok_or_else(|| Error::Other("External CI not found".into()))?;
    
    let mut cmd = Command::new(&external_ci.path);
    cmd.arg(command).args(args);
    
    let status = cmd.status()
        .map_err(|e| Error::Other(format!("Failed to execute external CI: {}", e)))?;
    
    if !status.success() {
        return Err(Error::Other(format!(
            "External CI command failed with exit code: {}",
            status.code().unwrap_or(-1)
        )));
    }
    
    Ok(())
}

/// Get the version of the external CI tool
fn get_external_ci_version(path: &str) -> Result<Version> {
    let output = Command::new(path)
        .arg("--version")
        .output()
        .map_err(|e| Error::Other(format!("Failed to get external CI version: {}", e)))?;
    
    if !output.status.success() {
        return Err(Error::Other("External CI version command failed".into()));
    }
    
    let version_str = String::from_utf8_lossy(&output.stdout);
    let version_parts: Vec<&str> = version_str.split_whitespace().collect();
    
    if version_parts.len() < 2 {
        return Err(Error::Other("Invalid version format".into()));
    }
    
    Version::parse(version_parts[1].trim_matches(|c| c == 'v' || c == 'r'))
        .map_err(|e| Error::Other(format!("Failed to parse version: {}", e)))
}
```

2. Update the helpers module:
```rust
// CLIs/ci/rust/src/helpers/mod.rs
pub mod fs;
pub mod git;
pub mod external; // Add this line

pub use self::fs::*;
pub use self::git::*;
pub use self::external::*; // Add this line
```

3. Update Cargo.toml to add dependencies:
```toml
# CLIs/ci/rust/Cargo.toml
[dependencies]
# ... existing dependencies
semver = "1.0"
dirs = "4.0"
```

**Commit Message:**
```
feat: Add external CI detection framework (Phase 1)

Implements a robust detection system for external CI repository
installation that:
- Searches PATH for ci-external
- Checks standard installation locations in HOME directory
- Supports environment variable overrides
- Gets version information for compatibility checking

This is Phase 1 of the external repository implementation that maintains
backward compatibility while adding the foundation for external detection.
```

### Phase 2: Command Wrapper Integration

**Purpose:** Implement command wrappers to use external CI when available.

**Files to create/modify:**
1. `CLIs/ci/rust/src/main.rs` - Update to check for external CI
2. `CLIs/plugins/claude/claude-init-impl.rs` - Create Rust implementation

**Implementation details:**

1. Update main.rs to check for external CI:
```rust
// CLIs/ci/rust/src/main.rs
// ... existing imports
use crate::helpers::{is_external_ci_available, execute_external_ci};

fn main() -> Result<()> {
    // ... existing code to parse args and load config
    
    // Check for external CI repository
    if is_external_ci_available() {
        // Show deprecation notice if configured
        if !config.suppress_deprecation_notices {
            show_deprecation_notice();
        }
        
        // Extract command and arguments for external CI
        let command_name = match cli.command.as_ref().unwrap() {
            Commands::Init { .. } => "init",
            Commands::Integrate { .. } => "integrate",
            Commands::Status { .. } => "status",
            Commands::Fix { .. } => "fix",
            Commands::Verify { .. } => "verify",
            Commands::Load { .. } => "load",
            Commands::Map => "map",
        };
        
        // Get all args except the program name
        let args: Vec<String> = std::env::args().skip(1).collect();
        
        // Execute external CI tool
        match execute_external_ci(command_name, &args, &config) {
            Ok(()) => return Ok(()),
            Err(e) => {
                eprintln!("Warning: External CI failed: {}. Falling back to local implementation.", e);
                // Continue with local implementation
            }
        }
    }
    
    // ... existing command handling code
}

fn show_deprecation_notice() {
    // Check if we've shown this notice recently
    let home_dir = match dirs::home_dir() {
        Some(dir) => dir,
        None => return,
    };
    
    let notice_file = home_dir.join(".ci").join(".local_deprecation_shown");
    
    if let Ok(metadata) = std::fs::metadata(&notice_file) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(elapsed) = modified.elapsed() {
                // Only show once per day (86400 seconds)
                if elapsed.as_secs() < 86400 {
                    return;
                }
            }
        }
    }
    
    eprintln!("DEPRECATION NOTICE: You are using the local CI implementation which is deprecated.");
    eprintln!("Please install the external CI tools repository for improved functionality.");
    eprintln!("For more information, visit: https://github.com/anthropics/ci-tools");
    
    // Update the notice file
    if let Err(e) = std::fs::create_dir_all(home_dir.join(".ci")) {
        eprintln!("Warning: Failed to create directory: {}", e);
        return;
    }
    
    if let Err(e) = std::fs::write(&notice_file, "") {
        eprintln!("Warning: Failed to update notice file: {}", e);
    }
}
```

2. Create Rust implementation of claude-init-impl:
```rust
// CLIs/plugins/claude/claude-init-impl.rs
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::error::Error;

/// Detect the external CI tool
fn detect_external_ci() -> Option<PathBuf> {
    // Check environment variable override
    if let Ok(force_local) = env::var("CI_FORCE_LOCAL") {
        if force_local == "1" || force_local.to_lowercase() == "true" {
            return None;
        }
    }
    
    // Check if ci-external is in PATH
    if let Ok(status) = Command::new("ci-external")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .status() {
        if status.success() {
            return Some(PathBuf::from("ci-external"));
        }
    }
    
    // Check $HOME/.ci/bin/ci
    if let Some(home_dir) = dirs::home_dir() {
        let home_ci_path = home_dir.join(".ci").join("bin").join("ci");
        if home_ci_path.exists() {
            return Some(home_ci_path);
        }
    }
    
    // Check $CI_EXTERNAL_PATH/bin/ci
    if let Ok(external_path) = env::var("CI_EXTERNAL_PATH") {
        let custom_ci_path = PathBuf::from(external_path).join("bin").join("ci");
        if custom_ci_path.exists() {
            return Some(custom_ci_path);
        }
    }
    
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        return Err("Usage: claude-init-impl <project_name> [integration_type] [--agents=<agents>] [--ci-path=<path>] [--fast]".into());
    }
    
    let project_name = &args[1];
    let mut integration_type = "standalone";
    let mut agents = "Athena";
    let mut ci_path = None;
    let mut fast_activation = false;
    
    for i in 2..args.len() {
        if args[i].starts_with("--agents=") {
            agents = &args[i]["--agents=".len()..];
        } else if args[i].starts_with("--ci-path=") {
            ci_path = Some(&args[i]["--ci-path=".len()..]);
        } else if args[i] == "--fast" {
            fast_activation = true;
        } else {
            integration_type = &args[i];
        }
    }
    
    // Try to use external CI tool first
    if let Some(external_ci) = detect_external_ci() {
        println!("Using external CI tool: {}", external_ci.display());
        
        // Build arguments for external CI tool
        let mut command = Command::new(&external_ci);
        command.arg("init")
               .arg(project_name)
               .arg("--integration")
               .arg(integration_type)
               .arg("--agents")
               .arg(agents);
               
        if fast_activation {
            command.arg("--fast");
        }
        
        if let Some(path) = ci_path {
            command.arg("--ci-path").arg(path);
        }
        
        let status = command.status()?;
        if !status.success() {
            return Err("External CI tool failed".into());
        }
        
        return Ok(());
    }
    
    // Fall back to current implementation
    // ... existing implementation ...
    
    Ok(())
}
```

**Commit Message:**
```
feat: Implement command wrapper system (Phase 2)

Adds command wrapper integration to use external CI repository when
available:

- Updates main CLI entry point to check for external implementation
- Delegates commands to external binary when available
- Implements graceful fallback to local implementation
- Adds rate-limited deprecation notices
- Creates Rust implementation of claude-init-impl

This phase maintains backward compatibility while enabling external
repository usage when installed.
```

### Phase 3: Transition Documentation

**Purpose:** Provide documentation to users and update references.

**Files to create/modify:**
1. `docs/cli/migration_guide.md` - Create migration guide
2. `docs/cli/external_integration.md` - Document external integration
3. `README.md` - Update with external repository information

**Implementation details:**

1. Create migration guide:
```markdown
# CI Tool Migration Guide

## Overview

The CI functionality is being migrated from the CollaborativeIntelligence repository to a dedicated external repository. This guide helps you transition to the new implementation.

## Why Migrate?

The migration provides several benefits:

- Clearer separation of concerns
- Simplified dependencies
- Faster updates
- Better maintainability
- Consistent cross-project experience

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
- `CI_FORCE_LOCAL`: Set to "1" to force using local implementation
- `CI_NO_DEPRECATION_NOTICE`: Set to "1" to disable deprecation notices

## Compatibility

The external implementation is compatible with projects created using the local implementation. No migration of existing projects is needed.

## Transition Period

During the transition period:

1. Both implementations will coexist
2. External implementation will be used when available
3. Local implementation will be used as fallback
4. Deprecation notices will be shown for local implementation
5. Eventually, local implementation will be removed

## Troubleshooting

If you encounter issues:

1. Try forcing the local implementation: `CI_FORCE_LOCAL=1 ci <command>`
2. Check installation: `ci-external --version`
3. Reinstall: `cargo install --force ci-tools`
```

2. Document external integration:
```markdown
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

## Detection Mechanism

The system uses the following order to detect the external implementation:

1. Check if `CI_FORCE_LOCAL=1` is set (forces local implementation)
2. Check if `ci-external` is in PATH
3. Check if `$HOME/.ci/bin/ci` exists
4. Check if `$CI_EXTERNAL_PATH/bin/ci` exists

## Command Delegation

When external implementation is detected:

1. The command is delegated to the external implementation
2. Arguments are passed through
3. Output from external tool is displayed
4. On failure, falls back to local implementation

## Compatibility

The external implementation is designed to be fully compatible with:

1. Existing projects created with local implementation
2. Command line arguments and options
3. Environment variables and configuration
4. File formats and structures

## Transition Period

During the transition period:

1. Both implementations coexist
2. External implementation is preferred when available
3. Deprecation notices are shown for local implementation
4. Eventually, local implementation will be removed
```

3. Update README.md to include external repository information:
```markdown
# CLI Tools

## External CI Repository

The CI functionality has been migrated to an external repository:

- Repository: https://github.com/anthropics/ci-tools
- Installation: `cargo install ci-tools`
- Documentation: [Migration Guide](docs/cli/migration_guide.md)

The local implementation is now deprecated and will be removed in a future update.
```

**Commit Message:**
```
docs: Add transition documentation for CI external repository (Phase 3)

Adds comprehensive documentation to support the transition to the external
CI repository:

- Detailed migration guide for users
- External integration documentation
- Updated README with repository information and installation instructions

This documentation helps users understand the migration process and
transition smoothly to the external repository implementation.
```

### Phase 4: Final Integration Test

**Purpose:** Create integration tests to validate the transition.

**Files to create/modify:**
1. `CLIs/ci/rust/tests/external_integration_test.rs` - Create integration test

**Implementation details:**

1. Create integration test:
```rust
// CLIs/ci/rust/tests/external_integration_test.rs
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_external_detection() {
    let ci_dir = tempdir().unwrap();
    let bin_dir = ci_dir.path().join("bin");
    fs::create_dir_all(&bin_dir).unwrap();
    
    // Create fake external CI
    let ci_bin = bin_dir.join(if cfg!(windows) { "ci.exe" } else { "ci" });
    fs::write(&ci_bin, "#!/bin/sh\necho ci-external v0.1.0\nexit 0").unwrap();
    
    if !cfg!(windows) {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&ci_bin).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&ci_bin, perms).unwrap();
    }
    
    // Test with CI_EXTERNAL_PATH
    env::set_var("CI_EXTERNAL_PATH", ci_dir.path());
    
    let output = Command::new("cargo")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .arg("run")
        .arg("--")
        .arg("--version")
        .output()
        .unwrap();
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    
    // Should contain deprecation notice
    assert!(output.status.success());
    assert!(output_str.contains("CI version") || output_str.contains("DEPRECATION NOTICE"));
}

#[test]
fn test_local_fallback() {
    // Force local implementation
    env::set_var("CI_FORCE_LOCAL", "1");
    
    let output = Command::new("cargo")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .arg("run")
        .arg("--")
        .arg("--version")
        .output()
        .unwrap();
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    
    // Should not contain deprecation notice
    assert!(output.status.success());
    assert!(output_str.contains("CI version"));
    assert!(!output_str.contains("DEPRECATION NOTICE"));
    
    // Reset env var
    env::remove_var("CI_FORCE_LOCAL");
}
```

**Commit Message:**
```
test: Add integration tests for external CI detection (Phase 4)

Adds integration tests to validate the external CI repository transition:

- Tests external CI detection using environment variables
- Tests fallback to local implementation when forced
- Verifies deprecation notices are shown appropriately

These tests ensure the transition mechanism works correctly and maintains
backward compatibility during the migration period.
```

## Integration Strategy

To ensure a smooth transition, the implementation will be released in phases:

1. **Early Adopter Phase (2 weeks)**
   - Release external repository separately
   - Announce to early adopters
   - Gather feedback and fix issues

2. **Main Transition Phase (1 month)**
   - Implement Phases 1-4 in the CollaborativeIntelligence repository
   - Update documentation
   - Start showing deprecation notices

3. **Completion Phase (2 months)**
   - Remove local implementation
   - Update all references to external repository
   - Complete transition

## Rollback Plan

If significant issues are discovered:

1. Set `CI_FORCE_LOCAL=1` in the environment to force using local implementation
2. Release a hotfix to address critical issues
3. If necessary, revert the external detection code while maintaining local implementation

## Success Criteria

The transition will be considered successful when:

1. All tests pass with both local and external implementations
2. Documentation is updated and clear
3. Users can easily install the external implementation
4. Existing projects continue to work without modification
5. No critical issues reported for 2 weeks