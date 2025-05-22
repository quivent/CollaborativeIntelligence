//! Terminal title tracking and management for CIA CLI
//!
//! This module provides comprehensive terminal title tracking capabilities including:
//! - Dynamic title updates based on project, agent, and task context
//! - Task lifecycle management with timeout handling
//! - Context monitoring for automatic updates
//! - Integration with agent activation and task management

pub mod title_manager;
pub mod task_tracker;
pub mod context_monitor;

// Re-export commonly used types and functions
pub use title_manager::TerminalTitleManager;
#[allow(unused_imports)]
pub use title_manager::{TitleConfig, ProjectDetectionMethod};
#[allow(unused_imports)]
pub use task_tracker::{TaskTracker, TaskContext, TaskStatus};
#[allow(unused_imports)]
pub use context_monitor::ContextMonitor;

// Global state management
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

static TITLE_MANAGER: Lazy<Arc<Mutex<TerminalTitleManager>>> = 
    Lazy::new(|| Arc::new(Mutex::new(TerminalTitleManager::new())));

/// Get the global terminal title manager instance
/// 
/// This provides thread-safe access to the shared title manager
/// used throughout the CIA CLI application.
pub fn get_title_manager() -> Arc<Mutex<TerminalTitleManager>> {
    TITLE_MANAGER.clone()
}

/// Initialize the terminal title system
/// 
/// This should be called early in the application lifecycle to set up
/// the terminal title tracking system with appropriate defaults.
pub fn initialize_title_system() -> anyhow::Result<()> {
    let title_manager = get_title_manager();
    let mut manager = title_manager.lock().map_err(|e| {
        anyhow::anyhow!("Failed to acquire title manager lock: {}", e)
    })?;
    
    // Set initial project context
    if let Ok(current_dir) = std::env::current_dir() {
        if let Some(project_name) = crate::helpers::path::PathHelpers::get_project_name(&current_dir) {
            manager.update_project(&project_name);
        }
    }
    
    // Set initial title
    manager.refresh_title();
    
    Ok(())
}

/// Update terminal title with current context
/// 
/// Convenience function for updating the terminal title with the current
/// project, agent, and task context.
pub fn update_title() -> anyhow::Result<()> {
    let title_manager = get_title_manager();
    let manager = title_manager.lock().map_err(|e| {
        anyhow::anyhow!("Failed to acquire title manager lock: {}", e)
    })?;
    
    manager.refresh_title();
    Ok(())
}

/// Update the active agent in the terminal title
/// 
/// Convenience function for updating just the agent portion of the title.
pub fn set_active_agent(agent_name: &str) -> anyhow::Result<()> {
    let title_manager = get_title_manager();
    let mut manager = title_manager.lock().map_err(|e| {
        anyhow::anyhow!("Failed to acquire title manager lock: {}", e)
    })?;
    
    manager.update_agent(agent_name);
    manager.refresh_title();
    Ok(())
}

/// Update the current task in the terminal title
/// 
/// Convenience function for updating just the task portion of the title.
pub fn set_current_task(task_description: &str) -> anyhow::Result<()> {
    let title_manager = get_title_manager();
    let mut manager = title_manager.lock().map_err(|e| {
        anyhow::anyhow!("Failed to acquire title manager lock: {}", e)
    })?;
    
    manager.update_task(task_description);
    manager.refresh_title();
    Ok(())
}

/// Clear the current task from the terminal title
/// 
/// Convenience function for clearing the task portion of the title.
pub fn clear_current_task() -> anyhow::Result<()> {
    let title_manager = get_title_manager();
    let mut manager = title_manager.lock().map_err(|e| {
        anyhow::anyhow!("Failed to acquire title manager lock: {}", e)
    })?;
    
    manager.clear_task();
    manager.refresh_title();
    Ok(())
}