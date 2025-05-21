//! External CI repository detection and integration
//!
//! This module provides functionality to detect and use the external CI repository.

use std::path::PathBuf;
use std::process::Command;
use std::env;
use anyhow::Result;
use crate::config::Config;

/// Represents an external CI tool
pub struct ExternalCi {
    /// Path to the external CI executable
    pub path: PathBuf,
    /// Version of the external CI tool
    pub version: String,
}

/// Check if the external CI tool is available
pub fn is_external_ci_available() -> bool {
    detect_external_ci().is_some()
}

/// Detect the external CI tool
pub fn detect_external_ci() -> Option<ExternalCi> {
    // Check environment variable override
    if let Ok(force_local) = env::var("CI_FORCE_LOCAL") {
        if force_local == "1" || force_local.to_lowercase() == "true" {
            return None;
        }
    }
    
    // Check the following locations in order:
    // 1. ci-external in PATH
    // 2. $HOME/.ci/bin/ci
    // 3. $CI_EXTERNAL_PATH/bin/ci
    
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

/// Get the version of the external CI tool
fn get_external_ci_version(ci_path: &str) -> String {
    let output = Command::new(ci_path)
        .arg("--version")
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        },
        _ => "unknown".to_string(),
    }
}

/// Check if the deprecation notice should be shown
pub fn should_show_deprecation_notice() -> bool {
    // Check environment variable to disable notice
    if let Ok(no_notice) = env::var("CI_NO_DEPRECATION_NOTICE") {
        if no_notice == "1" || no_notice.to_lowercase() == "true" {
            return false;
        }
    }
    
    // Check if notice shown recently (once per day)
    if let Some(home_dir) = dirs::home_dir() {
        let notice_file = home_dir.join(".ci_migration_notice");
        
        if notice_file.exists() {
            if let Ok(metadata) = std::fs::metadata(&notice_file) {
                if let Ok(modified) = metadata.modified() {
                    let now = std::time::SystemTime::now();
                    if let Ok(duration) = now.duration_since(modified) {
                        // Only show notice once per day (86400 seconds)
                        if duration.as_secs() < 86400 {
                            return false;
                        }
                    }
                }
            }
        }
        
        // Touch the notice file to update timestamp
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(notice_file);
    }
    
    true
}

/// Execute the external CI tool with the given command and arguments
pub fn execute_external_ci(command: &str, args: &[String], config: &Config) -> Result<()> {
    if let Some(external_ci) = detect_external_ci() {
        // Show notice about external CI usage (once per day)
        if should_show_deprecation_notice() {
            println!("ℹ️  Using external CI tool: {}", external_ci.path.display());
            println!("ℹ️  See https://github.com/anthropics/ci-tools for more information");
        }
        
        // Build command with args
        let status = Command::new(&external_ci.path)
            .arg(command)
            .args(args)
            .status()?;
        
        // Exit with same status code
        std::process::exit(status.code().unwrap_or(0));
    } else {
        // Show deprecation warning after command execution (will be added to main.rs)
        Ok(())
    }
}

/// Show deprecation notice
pub fn show_deprecation_notice() {
    if should_show_deprecation_notice() {
        println!();
        println!("⚠️  This local implementation will be deprecated in future releases.");
        println!("⚠️  Please install the external CI tool for continued support:");
        println!("    npm install -g @anthropics/ci-tools");
        println!();
    }
}