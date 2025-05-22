//! Command execution and UI helpers for CIA CLI
//!
//! This module provides helper functions for command output formatting,
//! command execution, and standard command operations.

use std::path::Path;
use std::process::Command;
use anyhow::{Result, Context};
use crate::helpers::terminal::{TerminalHelpers, ThemeColors};

/// Helper functions for common CIA command operations and UI
pub struct CommandHelpers;

// Allow dead code for helpers intended for backwards compatibility
#[allow(dead_code)]
impl CommandHelpers {
    /// Print a standard command header with category and emoji
    /// 
    /// This is kept for backward compatibility but delegates to the new TerminalHelpers
    pub fn print_command_header(title: &str, _emoji: &str, category: &str, color: &str) {
        let category_type = match category.to_lowercase().as_str() {
            "agent management" => "agent",
            "system operations" | "system utilities" => "system",
            "memory management" => "memory",
            "knowledge base" => "knowledge",
            "debug utilities" | "debug & diagnostics" => "debug",
            _ => {
                // Use the color to determine category if possible
                match color {
                    "cyan" => "agent",
                    "blue" => "system",
                    "magenta" => "memory",
                    "green" => "knowledge",
                    "yellow" => "debug",
                    _ => "system" // Default to system
                }
            }
        };
        
        TerminalHelpers::print_command_header(title, category_type);
    }
    
    /// Print a divider line with equals signs
    /// 
    /// This is kept for backward compatibility but delegates to the new TerminalHelpers
    pub fn print_divider(color: &str) {
        let theme_color = match color {
            "cyan" => ThemeColors::SECONDARY,
            "green" => ThemeColors::SUCCESS,
            "yellow" => ThemeColors::WARNING,
            "blue" => ThemeColors::PRIMARY,
            _ => ThemeColors::PRIMARY,
        };
        
        TerminalHelpers::print_divider(theme_color);
    }
    
    /// Print a success message with checkmark
    /// 
    /// This is kept for backward compatibility but delegates to the new TerminalHelpers
    pub fn print_success(message: &str) {
        TerminalHelpers::print_success(message);
    }
    
    /// Print an error message with cross
    /// 
    /// This is kept for backward compatibility but delegates to the new TerminalHelpers
    pub fn print_error(message: &str) {
        TerminalHelpers::print_error(message);
    }
    
    /// Print a warning message with warning symbol
    /// 
    /// This is kept for backward compatibility but delegates to the new TerminalHelpers
    pub fn print_warning(message: &str) {
        TerminalHelpers::print_warning(message);
    }
    
    /// Print an info message with info symbol
    /// 
    /// This is kept for backward compatibility but delegates to the new TerminalHelpers
    pub fn print_info(message: &str) {
        TerminalHelpers::print_info(message);
    }
    
    /// Print a step message with number and total
    /// 
    /// This is kept for backward compatibility but delegates to the new TerminalHelpers
    pub fn print_step(step: usize, total: usize, message: &str) {
        TerminalHelpers::print_step(step, total, message);
    }
    
    /// Print a section header with title and underline
    /// 
    /// This is kept for backward compatibility but delegates to the new TerminalHelpers
    pub fn print_section(title: &str) {
        TerminalHelpers::print_section(title, ThemeColors::PRIMARY);
    }
    
    /// Print a status message with bullet point
    /// 
    /// This is kept for backward compatibility but delegates to the new TerminalHelpers
    pub fn print_status(message: &str) {
        TerminalHelpers::print_status(message);
    }
    
    /// Prompt user for confirmation
    /// 
    /// This is kept for backward compatibility but delegates to the new TerminalHelpers
    pub fn prompt_confirmation(message: &str) -> bool {
        TerminalHelpers::prompt_confirmation(message)
    }
    
    /// Run a command and capture output
    pub fn run_command(command: &str, args: &[&str]) -> Result<String> {
        let output = Command::new(command)
            .args(args)
            .output()
            .with_context(|| format!("Failed to run {}: {}", command, args.join(" ")))?;
            
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(anyhow::anyhow!("Command failed: {}", error))
        }
    }
    
    /// Run a command in a specific directory and capture output
    pub fn run_command_in_dir(command: &str, args: &[&str], dir: &Path) -> Result<String> {
        let output = Command::new(command)
            .args(args)
            .current_dir(dir)
            .output()
            .with_context(|| format!("Failed to run {} in {}: {}", command, dir.display(), args.join(" ")))?;
            
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(anyhow::anyhow!("Command failed: {}", error))
        }
    }
    
    /// Check if a command exists in the system path
    pub fn command_exists(command: &str) -> bool {
        match std::env::consts::OS {
            "windows" => {
                Command::new("where")
                    .arg(command)
                    .output()
                    .map(|output| output.status.success())
                    .unwrap_or(false)
            },
            _ => {
                Command::new("which")
                    .arg(command)
                    .output()
                    .map(|output| output.status.success())
                    .unwrap_or(false)
            }
        }
    }
    
    /// Get the current terminal window title
    /// 
    /// This is kept for backward compatibility but delegates to the new TerminalHelpers
    pub fn set_terminal_title(title: &str) {
        TerminalHelpers::set_terminal_title(title);
    }
}