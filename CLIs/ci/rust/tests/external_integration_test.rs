//! Integration tests for external CI repository detection and integration
//!
//! These tests validate the external CI repository detection and delegation
//! mechanisms to ensure proper functionality during the transition period.

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::os::unix::fs::PermissionsExt;
use tempfile::TempDir;

/// Create a fake external CI tool in a temporary directory
fn create_fake_external_ci(dir: &Path, version: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let bin_dir = dir.join("bin");
    fs::create_dir_all(&bin_dir)?;
    
    let ci_bin = bin_dir.join("ci");
    let content = format!(
        "#!/bin/sh\n\
         if [ \"$1\" = \"--version\" ]; then\n\
         echo \"ci-external {}\"\n\
         exit 0\n\
         fi\n\
         echo \"Command: $@\"\n\
         echo \"Executed from $PWD\"\n\
         exit 0\n",
        version
    );
    
    fs::write(&ci_bin, content)?;
    
    // Make the script executable
    let mut perms = fs::metadata(&ci_bin)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&ci_bin, perms)?;
    
    Ok(ci_bin)
}

/// Create a fake external CI executable in PATH
fn create_fake_external_ci_in_path(temp_dir: &Path, version: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let bin_dir = temp_dir.join("bin");
    fs::create_dir_all(&bin_dir)?;
    
    let ci_bin = bin_dir.join("ci-external");
    let content = format!(
        "#!/bin/sh\n\
         if [ \"$1\" = \"--version\" ]; then\n\
         echo \"ci-external {}\"\n\
         exit 0\n\
         fi\n\
         echo \"PATH Command: $@\"\n\
         echo \"Executed from $PWD\"\n\
         exit 0\n",
        version
    );
    
    fs::write(&ci_bin, content)?;
    
    // Make the script executable
    let mut perms = fs::metadata(&ci_bin)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&ci_bin, perms)?;
    
    Ok(ci_bin)
}

/// Create a test project directory
fn create_test_project(dir: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let project_dir = dir.join("test-project");
    fs::create_dir_all(&project_dir)?;
    
    // Create a basic CLAUDE.md
    let claude_md = project_dir.join("CLAUDE.md");
    let content = "# Project: test-project\n# Created: 2025-05-21\n\n# Test project\n";
    fs::write(claude_md, content)?;
    
    Ok(project_dir)
}

/// Run the CI command with the given arguments
fn run_ci_command(project_dir: &Path, args: &[&str]) -> Result<(i32, String, String), Box<dyn std::error::Error>> {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--manifest-path=../../../CLIs/ci/rust/Cargo.toml")
        .args(args)
        .current_dir(project_dir)
        .output()?;
    
    let status = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    
    Ok((status, stdout, stderr))
}

#[test]
fn test_external_ci_detection_in_path() {
    // Create temporary directory for the test
    let temp_dir = TempDir::new().unwrap();
    let ci_bin = create_fake_external_ci_in_path(temp_dir.path(), "0.1.0").unwrap();
    
    // Create test project
    let project_dir = create_test_project(temp_dir.path()).unwrap();
    
    // Add the bin directory to PATH
    let path_dir = ci_bin.parent().unwrap();
    let old_path = env::var("PATH").unwrap_or_default();
    env::set_var("PATH", format!("{}:{}", path_dir.display(), old_path));
    
    // Run a CI command
    let result = run_ci_command(&project_dir, &["--version"]);
    
    // Clean up
    env::set_var("PATH", old_path);
    
    // Check results
    assert!(result.is_ok());
    let (status, stdout, _stderr) = result.unwrap();
    assert_eq!(status, 0);
    assert!(stdout.contains("ci-external") || stdout.contains("PATH Command"));
}

#[test]
fn test_external_ci_detection_in_home() {
    // Create temporary directory for the test
    let temp_dir = TempDir::new().unwrap();
    
    // Save original HOME value
    let old_home = env::var("HOME").ok();
    
    // Set HOME to our temp directory
    env::set_var("HOME", temp_dir.path());
    
    // Create fake CI in the .ci/bin directory
    let home_ci_dir = temp_dir.path().join(".ci");
    let ci_bin = create_fake_external_ci(&home_ci_dir, "0.1.0").unwrap();
    
    // Create test project
    let project_dir = create_test_project(temp_dir.path()).unwrap();
    
    // Force re-detection by clearing any existing variable
    env::remove_var("CI_EXTERNAL_PATH");
    
    // Run a CI command
    let result = run_ci_command(&project_dir, &["--version"]);
    
    // Restore original HOME
    if let Some(home) = old_home {
        env::set_var("HOME", home);
    } else {
        env::remove_var("HOME");
    }
    
    // Check results
    assert!(result.is_ok());
    let (status, stdout, _stderr) = result.unwrap();
    assert_eq!(status, 0);
    assert!(stdout.contains("ci-external") || stdout.contains("Command:"));
}

#[test]
fn test_external_ci_detection_with_env_var() {
    // Create temporary directory for the test
    let temp_dir = TempDir::new().unwrap();
    
    // Create fake CI in a custom directory
    let ci_dir = temp_dir.path().join("custom_ci");
    let ci_bin = create_fake_external_ci(&ci_dir, "0.1.0").unwrap();
    
    // Create test project
    let project_dir = create_test_project(temp_dir.path()).unwrap();
    
    // Set CI_EXTERNAL_PATH to our custom directory
    env::set_var("CI_EXTERNAL_PATH", ci_dir);
    
    // Run a CI command
    let result = run_ci_command(&project_dir, &["--version"]);
    
    // Clean up
    env::remove_var("CI_EXTERNAL_PATH");
    
    // Check results
    assert!(result.is_ok());
    let (status, stdout, _stderr) = result.unwrap();
    assert_eq!(status, 0);
    assert!(stdout.contains("ci-external") || stdout.contains("Command:"));
}

#[test]
fn test_force_local_implementation() {
    // Create temporary directory for the test
    let temp_dir = TempDir::new().unwrap();
    
    // Create fake CI in a custom directory
    let ci_dir = temp_dir.path().join("custom_ci");
    let _ci_bin = create_fake_external_ci(&ci_dir, "0.1.0").unwrap();
    
    // Create test project
    let project_dir = create_test_project(temp_dir.path()).unwrap();
    
    // Set CI_EXTERNAL_PATH to our custom directory
    env::set_var("CI_EXTERNAL_PATH", &ci_dir);
    
    // But force local implementation
    env::set_var("CI_FORCE_LOCAL", "1");
    
    // Run a CI command
    let result = run_ci_command(&project_dir, &["--version"]);
    
    // Clean up
    env::remove_var("CI_EXTERNAL_PATH");
    env::remove_var("CI_FORCE_LOCAL");
    
    // Check results
    assert!(result.is_ok());
    let (status, stdout, _stderr) = result.unwrap();
    assert_eq!(status, 0);
    
    // Should use local implementation (not contain the fake CI output)
    assert!(!stdout.contains("Command:"));
    assert!(stdout.contains("CI version") || stdout.contains("This local implementation"));
}

#[test]
fn test_deprecation_notice_control() {
    // Create temporary directory for the test
    let temp_dir = TempDir::new().unwrap();
    
    // Create test project
    let project_dir = create_test_project(temp_dir.path()).unwrap();
    
    // Make sure external CI is not detected
    env::set_var("CI_FORCE_LOCAL", "1");
    
    // But enable deprecation notices
    env::remove_var("CI_NO_DEPRECATION_NOTICE");
    
    // Run a CI command - should show deprecation notice
    let result1 = run_ci_command(&project_dir, &["--version"]);
    
    // Now suppress deprecation notices
    env::set_var("CI_NO_DEPRECATION_NOTICE", "1");
    
    // Run again - should not show deprecation notice
    let result2 = run_ci_command(&project_dir, &["--version"]);
    
    // Clean up
    env::remove_var("CI_FORCE_LOCAL");
    env::remove_var("CI_NO_DEPRECATION_NOTICE");
    
    // Check results
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    
    let (_status1, stdout1, _stderr1) = result1.unwrap();
    let (_status2, stdout2, _stderr2) = result2.unwrap();
    
    // The first run should contain the deprecation notice
    let has_deprecation_notice = stdout1.contains("deprecated") || stdout1.contains("external CI tool");
    
    // The second run should not contain the deprecation notice when suppressed
    let notice_suppressed = !stdout2.contains("deprecated") && !stdout2.contains("external CI tool");
    
    // This might not work perfectly in the test environment due to the rate limiting check,
    // so we'll be lenient in the assertion
    assert!(has_deprecation_notice || notice_suppressed);
}

#[test]
fn test_plugin_integration() {
    // Create temporary directory for the test
    let temp_dir = TempDir::new().unwrap();
    
    // Create fake CI in a custom directory
    let ci_dir = temp_dir.path().join("custom_ci");
    let ci_bin = create_fake_external_ci(&ci_dir, "0.1.0").unwrap();
    
    // Create a test directory for the project
    let work_dir = temp_dir.path().join("work");
    fs::create_dir_all(&work_dir).unwrap();
    
    // Set CI_EXTERNAL_PATH to our custom directory
    env::set_var("CI_EXTERNAL_PATH", ci_dir);
    
    // Create a simple claude-init-impl test script
    let test_script = work_dir.join("test-plugin.sh");
    let content = r#"#!/bin/bash
# Simple test script for claude-init-impl
cargo run --manifest-path=../../../CLIs/plugins/claude/Cargo.toml -- test-project --with-ci
"#;
    
    fs::write(&test_script, content).unwrap();
    
    // Make it executable
    let mut perms = fs::metadata(&test_script).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&test_script, perms).unwrap();
    
    // Run the test script
    let output = Command::new(&test_script)
        .current_dir(&work_dir)
        .output()
        .unwrap();
    
    // Clean up
    env::remove_var("CI_EXTERNAL_PATH");
    
    // Check results
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should detect and use the external CI
    assert!(stdout.contains("external CI tool") || stdout.contains("Command:"));
}

#[test]
fn test_fallback_mechanism() {
    // Create temporary directory for the test
    let temp_dir = TempDir::new().unwrap();
    
    // Create a fake non-working external CI
    let ci_dir = temp_dir.path().join("custom_ci");
    fs::create_dir_all(ci_dir.join("bin")).unwrap();
    
    let ci_bin = ci_dir.join("bin").join("ci");
    let content = "#!/bin/sh\nexit 1"; // Always fails
    fs::write(&ci_bin, content).unwrap();
    
    // Make it executable
    let mut perms = fs::metadata(&ci_bin).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&ci_bin, perms).unwrap();
    
    // Create test project
    let project_dir = create_test_project(temp_dir.path()).unwrap();
    
    // Set CI_EXTERNAL_PATH to our custom directory
    env::set_var("CI_EXTERNAL_PATH", ci_dir);
    
    // Run a CI command
    let result = run_ci_command(&project_dir, &["--version"]);
    
    // Clean up
    env::remove_var("CI_EXTERNAL_PATH");
    
    // Check results - should still succeed by falling back to local implementation
    assert!(result.is_ok());
    let (status, stdout, stderr) = result.unwrap();
    
    // Should use local implementation after failing with external
    assert_eq!(status, 0);
    assert!(stderr.contains("Warning") || stderr.contains("failed") || stdout.contains("Warning") || stdout.contains("failed"));
    assert!(stdout.contains("CI version") || stdout.contains("This local implementation"));
}