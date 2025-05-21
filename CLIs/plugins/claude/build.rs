//! Build script for Claude CI plugin

use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=claude-init-impl.rs");
    
    // Get output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).join("../../../");
    
    // Copy the shell wrapper that will load the compiled library
    let wrapper_content = r#"#!/bin/bash
# Claude CI plugin wrapper
# This script loads the compiled Rust library and executes the init command

# Get the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check if the Rust library exists
if [ -f "$SCRIPT_DIR/libclaude_ci_plugin.so" ]; then
  LIB_PATH="$SCRIPT_DIR/libclaude_ci_plugin.so"
elif [ -f "$SCRIPT_DIR/libclaude_ci_plugin.dylib" ]; then
  LIB_PATH="$SCRIPT_DIR/libclaude_ci_plugin.dylib"
elif [ -f "$SCRIPT_DIR/claude_ci_plugin.dll" ]; then
  LIB_PATH="$SCRIPT_DIR/claude_ci_plugin.dll"
else
  echo "Error: Claude CI plugin library not found"
  exit 1
fi

# Check if --with-ci flag is present
WITH_CI=false
for arg in "$@"; do
  if [ "$arg" = "--with-ci" ]; then
    WITH_CI=true
    break
  fi
done

# Only proceed with CI integration if --with-ci flag is present
if [ "$WITH_CI" = false ]; then
  # Just create a basic project without CI integration
  mkdir -p "$1"
  cd "$1" || exit 1
  echo "Project $1 created (without CI integration)"
  exit 0
fi

# Execute the Rust library using LD_PRELOAD (or equivalent)
LD_PRELOAD="$LIB_PATH" DYLD_INSERT_LIBRARIES="$LIB_PATH" "$SHELL" -c "exit 0"
exit $?
"#;

    // Write the wrapper script
    let wrapper_path = format!("{}/claude-init-impl.sh", out_dir);
    fs::write(&wrapper_path, wrapper_content).unwrap();
    
    // Make it executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&wrapper_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&wrapper_path, perms).unwrap();
    }
    
    // Copy to the target directory
    let dest_path = target_dir.join("claude-init-impl.sh");
    fs::copy(wrapper_path, dest_path).unwrap();
    
    // Add the rustc link search path
    println!("cargo:rustc-link-search=native={}", out_dir);
    
    // Make sure the library is statically linked
    println!("cargo:rustc-link-lib=static=claude_ci_plugin");
}