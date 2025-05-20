# CIR to CI Naming Fixes

This document provides a comprehensive guide for fixing the inconsistent naming where "CI" was incorrectly renamed to "CIR" during the Rust port.

## Problem Statement

During the port of the CLI system to Rust, some references to "CI" were changed to "CIR" (presumably meaning "CI Rust"). This has created inconsistency and confusion in the codebase. All references should be reverted to "CI" for clarity and consistency.

## Files to Check and Update

The following file patterns should be checked for "CIR" references:

1. **Rust Source Files**: `*.rs`
2. **Cargo Configuration**: `Cargo.toml`
3. **Documentation**: `*.md`
4. **Shell Scripts**: `*.sh`
5. **Configuration Files**: `*.json`, `*.toml`

## Common Naming Patterns to Fix

1. **Binary Names**:
   - Change `cir` to `ci`
   - Change `CIR` to `CI`

2. **Package Names**:
   - Change `cir-cli` to `ci-cli`
   - Change `collaborative-intelligence-rust` to `collaborative-intelligence`

3. **Module Namespaces**:
   - Change `cir::` to `ci::`
   - Change `CIR::` to `CI::`

4. **Command Structures**:
   - Change `CIRCommands` to `CICommands`
   - Change `CIRConfig` to `CIConfig`

5. **File Names**:
   - Change `cir_*.rs` to `ci_*.rs`
   - Change `cir-*.rs` to `ci-*.rs`

## Implementation Recommendations

When making these changes, be careful to maintain backward compatibility:

1. **Add Aliases**: For command-line tools, add aliases so `cir` still works but prints a deprecation notice
2. **Update Documentation**: Ensure all documentation references "CI", not "CIR"
3. **Version Carefully**: Consider a minor version bump to indicate these naming changes

## Specific Changes Needed

Based on a cursory examination of the codebase, here are specific changes needed:

### Rust Code Changes

```rust
// BEFORE
mod cir_commands;
use cir_commands::CIRCommand;

struct CIRConfig {
    // ...
}

const CIR_VERSION: &str = "1.0.0";

// AFTER
mod ci_commands;
use ci_commands::CICommand;

struct CIConfig {
    // ...
}

const CI_VERSION: &str = "1.0.0";
```

### Cargo.toml Changes

```toml
# BEFORE
[package]
name = "cir-cli"
version = "0.2.0"
description = "Collaborative Intelligence Rust CLI"

[[bin]]
name = "cir"
path = "src/main.rs"

# AFTER
[package]
name = "ci-cli"
version = "0.2.0"
description = "Collaborative Intelligence CLI"

[[bin]]
name = "ci"
path = "src/main.rs"
```

### Documentation Changes

```markdown
<!-- BEFORE -->
# CIR: Collaborative Intelligence Rust CLI

The CIR tool provides integration with the Collaborative Intelligence system.

```bash
cir init my-project
```

<!-- AFTER -->
# CI: Collaborative Intelligence CLI

The CI tool provides integration with the Collaborative Intelligence system.

```bash
ci init my-project
```
```

## Backward Compatibility

For backward compatibility, implement the following:

1. Create symbolic links from new names to old names:
   ```bash
   ln -s /usr/local/bin/ci /usr/local/bin/cir
   ```

2. Add deprecation notices:
   ```rust
   // In cir.rs (compatibility wrapper)
   fn main() {
       eprintln!("Warning: 'cir' is deprecated. Please use 'ci' instead.");
       ci::main();
   }
   ```

## Testing

After making these changes, test thoroughly to ensure:

1. All functionality continues to work
2. Documentation is consistent
3. No old "CIR" references remain
4. Scripts or tools depending on the old names still function

## Conclusion

Standardizing on "CI" rather than "CIR" will improve clarity, consistency, and maintainability of the codebase. This change aligns the code with the project's name (Collaborative Intelligence) and removes confusing terminology.