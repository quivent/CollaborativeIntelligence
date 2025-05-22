//! Task management commands for CIA CLI
//!
//! This module provides commands for managing terminal title tasks including
//! starting, completing, and monitoring tasks with timeout support.

use crate::config::Config;
use crate::helpers::terminal::TerminalHelpers;
use crate::terminal;
use anyhow::Result;

/// Start a new task
pub fn start(description: &str, _config: &Config) -> Result<()> {
    TerminalHelpers::print_fancy_header(
        "Task Management",
        "📝",
        "Start Task",
        crate::helpers::terminal::ThemeColors::TASK
    );
    
    // Set the task in the terminal title system
    if let Err(e) = terminal::set_current_task(description) {
        TerminalHelpers::print_warning(&format!("Failed to update terminal title: {}", e));
    }
    
    TerminalHelpers::print_success(&format!("Started task: '{}'", description));
    println!("Task will automatically timeout after 60 minutes if not completed.");
    
    Ok(())
}

/// Complete the current task
pub fn complete(_config: &Config) -> Result<()> {
    TerminalHelpers::print_fancy_header(
        "Task Management",
        "✅",
        "Complete Task",
        crate::helpers::terminal::ThemeColors::TASK
    );
    
    // Get current task status first
    match terminal::get_task_status() {
        Ok(status) => {
            if status.contains("No active task") {
                TerminalHelpers::print_warning("No active task to complete");
                return Ok(());
            }
            
            println!("Completing: {}", status);
        },
        Err(e) => {
            TerminalHelpers::print_warning(&format!("Failed to get task status: {}", e));
        }
    }
    
    // Clear the task from the terminal title system
    if let Err(e) = terminal::clear_current_task() {
        TerminalHelpers::print_warning(&format!("Failed to update terminal title: {}", e));
    }
    
    TerminalHelpers::print_success("Task completed successfully!");
    
    Ok(())
}

/// Clear the current task without marking as complete
pub fn clear(_config: &Config) -> Result<()> {
    TerminalHelpers::print_fancy_header(
        "Task Management",
        "🧹",
        "Clear Task",
        crate::helpers::terminal::ThemeColors::TASK
    );
    
    // Get current task status first
    match terminal::get_task_status() {
        Ok(status) => {
            if status.contains("No active task") {
                TerminalHelpers::print_warning("No active task to clear");
                return Ok(());
            }
            
            println!("Clearing: {}", status);
        },
        Err(e) => {
            TerminalHelpers::print_warning(&format!("Failed to get task status: {}", e));
        }
    }
    
    // Clear the task from the terminal title system
    if let Err(e) = terminal::clear_current_task() {
        TerminalHelpers::print_warning(&format!("Failed to update terminal title: {}", e));
    }
    
    TerminalHelpers::print_info("Task cleared from terminal title");
    
    Ok(())
}

/// Show current task status
pub fn status(_config: &Config) -> Result<()> {
    TerminalHelpers::print_fancy_header(
        "Task Management",
        "📊",
        "Task Status",
        crate::helpers::terminal::ThemeColors::TASK
    );
    
    match terminal::get_task_status() {
        Ok(status) => {
            if status.contains("No active task") {
                TerminalHelpers::print_info("No active task");
            } else if status.contains("TIMEOUT WARNING") {
                TerminalHelpers::print_warning(&status);
                println!("⚠️  Task is approaching timeout - consider completing or extending");
            } else {
                TerminalHelpers::print_success(&status);
            }
        },
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to get task status: {}", e));
        }
    }
    
    Ok(())
}

/// Check for task timeout and clear if necessary
pub fn check(_config: &Config) -> Result<()> {
    TerminalHelpers::print_fancy_header(
        "Task Management",
        "⏰",
        "Timeout Check",
        crate::helpers::terminal::ThemeColors::TASK
    );
    
    match terminal::check_task_timeout() {
        Ok(was_cleared) => {
            if was_cleared {
                TerminalHelpers::print_warning("Task was cleared due to timeout");
                println!("Tasks automatically timeout after 60 minutes of inactivity");
            } else {
                match terminal::get_task_status() {
                    Ok(status) => {
                        if status.contains("No active task") {
                            TerminalHelpers::print_info("No active task to check");
                        } else {
                            TerminalHelpers::print_success("Task is within timeout limits");
                            println!("Current: {}", status);
                        }
                    },
                    Err(e) => {
                        TerminalHelpers::print_warning(&format!("Failed to get task status: {}", e));
                    }
                }
            }
        },
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to check task timeout: {}", e));
        }
    }
    
    Ok(())
}