//! Execution helpers for TL CLI
//!
//! This module provides helpers for executing commands and managing processes.

use std::process::{Command, Stdio};
use std::path::Path;
use anyhow::{Result, Context};

/// Helper functions for command execution
#[allow(dead_code)]
pub struct ExecutionHelpers;

impl ExecutionHelpers {
    /// Run a command and capture output
    #[allow(dead_code)]
    pub fn run_command(command: &str, args: &[&str]) -> Result<String> {
        let output = Command::new(command)
            .args(args)
            .output()
            .with_context(|| format!("Failed to run command: {} {}", command, args.join(" ")))?;
            
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(anyhow::anyhow!("Command failed: {}", error))
        }
    }
    
    /// Run a command in a specific directory and capture output
    #[allow(dead_code)]
    pub fn run_command_in_dir(command: &str, args: &[&str], dir: &Path) -> Result<String> {
        let output = Command::new(command)
            .args(args)
            .current_dir(dir)
            .output()
            .with_context(|| format!("Failed to run {} in {}: {}", 
                command, dir.display(), args.join(" ")))?;
            
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(anyhow::anyhow!("Command failed: {}", error))
        }
    }
    
    /// Run a command and stream its output
    #[allow(dead_code)]
    pub fn run_command_with_output(command: &str, args: &[&str]) -> Result<()> {
        let status = Command::new(command)
            .args(args)
            .status()
            .with_context(|| format!("Failed to run command: {} {}", command, args.join(" ")))?;
            
        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Command failed with exit code: {:?}", status.code()))
        }
    }
    
    /// Run a command in a specific directory and stream its output
    #[allow(dead_code)]
    pub fn run_command_in_dir_with_output(command: &str, args: &[&str], dir: &Path) -> Result<()> {
        let status = Command::new(command)
            .args(args)
            .current_dir(dir)
            .status()
            .with_context(|| format!("Failed to run {} in {}: {}", 
                command, dir.display(), args.join(" ")))?;
            
        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Command failed with exit code: {:?}", status.code()))
        }
    }
    
    /// Check if a command exists in the system path
    #[allow(dead_code)]
    pub fn command_exists(command: &str) -> bool {
        match std::env::consts::OS {
            "windows" => {
                Command::new("where")
                    .arg(command)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .map(|status| status.success())
                    .unwrap_or(false)
            },
            _ => {
                Command::new("which")
                    .arg(command)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .map(|status| status.success())
                    .unwrap_or(false)
            }
        }
    }
    
    /// Run a command with input from a string
    #[allow(dead_code)]
    pub fn run_command_with_input(command: &str, args: &[&str], input: &str) -> Result<String> {
        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| format!("Failed to spawn command: {} {}", command, args.join(" ")))?;
            
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(input.as_bytes())
                .with_context(|| "Failed to write to stdin")?;
            // stdin closes when dropped
        }
        
        let output = child.wait_with_output()
            .with_context(|| "Failed to wait for command")?;
            
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(anyhow::anyhow!("Command failed: {}", error))
        }
    }
}