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

This separation provides clearer boundaries between tools intended for different purposes:

- **CI Tool**: For end users integrating with Collaborative Intelligence
- **TL Tool**: For repository maintainers managing the CollaborativeIntelligence repository

## Detection Mechanism

The system uses the following order to detect the external implementation:

1. Check if `CI_FORCE_LOCAL=1` is set (forces local implementation)
2. Check if `ci-external` is in PATH
3. Check if `$HOME/.ci/bin/ci` exists
4. Check if `$CI_EXTERNAL_PATH/bin/ci` exists

The detection is implemented in `CLIs/ci/rust/src/helpers/external.rs` and follows this algorithm:

```rust
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
                version: get_external_ci_version("ci-external"),
            });
        }
    }
    
    // Check $HOME/.ci/bin/ci
    if let Some(home_dir) = dirs::home_dir() {
        let home_ci_path = home_dir.join(".ci").join("bin").join("ci");
        if home_ci_path.exists() {
            return Some(ExternalCi {
                path: home_ci_path.clone(),
                version: get_external_ci_version(&home_ci_path.to_string_lossy()),
            });
        }
    }
    
    // Check $CI_EXTERNAL_PATH/bin/ci
    if let Ok(external_path) = env::var("CI_EXTERNAL_PATH") {
        let custom_ci_path = PathBuf::from(external_path).join("bin").join("ci");
        if custom_ci_path.exists() {
            return Some(ExternalCi {
                path: custom_ci_path.clone(),
                version: get_external_ci_version(&custom_ci_path.to_string_lossy()),
            });
        }
    }
    
    None
}
```

## Command Delegation

When an external implementation is detected, the local implementation delegates commands to it:

1. The command and arguments are captured
2. The external implementation is executed with those arguments
3. The exit code from the external implementation is passed through
4. In case of errors, execution falls back to the local implementation

This delegation happens in the main entry point (`CLIs/ci/rust/src/main.rs`):

```rust
fn main() -> anyhow::Result<()> {
    // Parse arguments and load config...
    
    // Check for external CI repository
    if is_external_ci_available() {
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
                // Log failure and continue with local implementation
                eprintln!("Warning: External CI failed: {}. Falling back to local implementation.", e);
            }
        }
    }
    
    // Proceed with local implementation...
}
```

## Plugin Integration

Plugins like `claude-init-impl` also support external implementation. The Rust implementation checks for the external CI tool first:

```rust
fn main() -> Result<(), Box<dyn Error>> {
    // Parse arguments...
    
    // Try to use external CI tool first
    if let Some(external_ci) = detect_external_ci() {
        println!("Using external CI tool: {}", external_ci.path.display());
        
        // Build arguments for external CI tool
        let mut command = Command::new(&external_ci.path);
        command.arg("init")
               .arg(&args.project_name)
               .arg("--integration")
               .arg(&args.integration_type)
               .arg("--agents")
               .arg(&args.agents);
        
        // Add other arguments...
        
        // Execute external CI tool
        let status = command.status()?;
        if !status.success() {
            return Err("External CI tool failed".into());
        }
        
        return Ok(());
    }
    
    // Fall back to local implementation...
}
```

## Compatibility

The external implementation is designed to be fully compatible with:

1. **Project Format**: Projects created with either implementation are compatible
2. **Command Line Interface**: Same commands and arguments structure
3. **Configuration Files**: Same format and structure
4. **Plugin System**: Supports all existing plugins
5. **Environment Variables**: Same environment variables

## Deprecation Notices

During the transition period, a deprecation notice is shown to encourage migration:

```
⚠️  This local implementation will be deprecated in future releases.
⚠️  Please install the external CI tool for continued support:
    npm install -g @anthropics/ci-tools
```

This notice is rate-limited to show at most once per day to avoid being too intrusive.

## Configuration

The system respects the following environment variables:

- `CI_EXTERNAL_PATH`: Path to the external CI repository
- `CI_FORCE_LOCAL`: Set to "1" to force using local implementation
- `CI_NO_DEPRECATION_NOTICE`: Set to "1" to disable deprecation notices

## Error Handling

The system uses a robust error handling strategy:

1. **Detection Errors**: Silent fallback to local implementation
2. **Execution Errors**: Show warning and fall back to local implementation
3. **Version Incompatibilities**: Show warning but continue if possible
4. **Fatal Errors**: Show clear error message with recovery steps

## Future Development

All new features will be added to the external repository first. The local implementation will only receive critical bug fixes during the transition period.

## Transition Timeline

1. **Phase 1**: External detection mechanism (current)
2. **Phase 2**: Wrapper implementation and delegation 
3. **Phase 3**: Documentation updates
4. **Phase 4**: Transition notice period (3 months)
5. **Phase 5**: Removal of local implementation

## Implementation Details

### External Tool Detection

The system uses this sequence to locate the external CI tool:

1. Check for `ci-external` in PATH
2. Check for `$HOME/.ci/bin/ci`
3. Check for `$CI_EXTERNAL_PATH/bin/ci`

### Command Delegation

1. Commands are passed through to the external implementation
2. Arguments are reconstructed to ensure compatibility
3. Exit codes are properly propagated to maintain script behavior

### Backward Compatibility

During the transition period:

1. All command signatures remain identical
2. Output formats are maintained for scripts that parse CLI output
3. Configuration format is compatible between versions
4. Projects created with either implementation can be used with both

## Examples

### Command Usage

```bash
# Initialize a project
ci init my-project              # Uses external tool if available

# Fix integration issues
ci fix                          # Uses external tool if available

# Verify integration
ci verify                       # Uses external tool if available

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

Document Version: 2.0  
Last Updated: May 21, 2025