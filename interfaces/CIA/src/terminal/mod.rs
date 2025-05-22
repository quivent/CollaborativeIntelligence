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

// Global context monitor for background monitoring
static CONTEXT_MONITOR: Lazy<Arc<Mutex<Option<ContextMonitor>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(None)));

/// Get the global context monitor instance
pub fn get_context_monitor() -> Arc<Mutex<Option<ContextMonitor>>> {
    CONTEXT_MONITOR.clone()
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
    
    // Drop the manager lock before creating context monitor
    drop(manager);
    
    // Initialize context monitoring
    initialize_context_monitoring()?;
    
    Ok(())
}

/// Initialize context monitoring for automatic title updates
pub fn initialize_context_monitoring() -> anyhow::Result<()> {
    let title_manager = get_title_manager();
    let context_monitor_ref = get_context_monitor();
    
    let mut context_monitor_guard = context_monitor_ref.lock().map_err(|e| {
        anyhow::anyhow!("Failed to acquire context monitor lock: {}", e)
    })?;
    
    // Create new context monitor if not already initialized
    if context_monitor_guard.is_none() {
        let mut monitor = ContextMonitor::new(Arc::clone(&title_manager));
        
        // Start background monitoring
        monitor.start_monitoring()?;
        
        // Store the monitor
        *context_monitor_guard = Some(monitor);
    }
    
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

/// Script compatibility layer for update-terminal-title.sh
/// 
/// This function provides compatibility with the existing bash script format
/// for updating terminal titles.
pub fn update_title_script_compat(agent_name: &str, task: Option<&str>) -> anyhow::Result<()> {
    let title_manager = get_title_manager();
    let mut manager = title_manager.lock().map_err(|e| {
        anyhow::anyhow!("Failed to acquire title manager lock: {}", e)
    })?;
    
    // Set agent name
    manager.update_agent(agent_name);
    
    // Set task if provided, otherwise use "Active Session" as default
    let task_description = task.unwrap_or("Active Session");
    manager.update_task(task_description);
    
    // Refresh title immediately
    manager.refresh_title();
    
    Ok(())
}

/// Check for task timeouts and clear if necessary
/// 
/// This function can be called periodically to enforce task timeout policies.
pub fn check_task_timeout() -> anyhow::Result<bool> {
    let title_manager = get_title_manager();
    let mut manager = title_manager.lock().map_err(|e| {
        anyhow::anyhow!("Failed to acquire title manager lock: {}", e)
    })?;
    
    let was_cleared = manager.clear_task_if_timed_out();
    if was_cleared {
        manager.refresh_title();
    }
    
    Ok(was_cleared)
}

/// Get current task status including elapsed time
/// 
/// Returns a formatted string with task status information.
pub fn get_task_status() -> anyhow::Result<String> {
    let title_manager = get_title_manager();
    let manager = title_manager.lock().map_err(|e| {
        anyhow::anyhow!("Failed to acquire title manager lock: {}", e)
    })?;
    
    match manager.get_task() {
        Some(task) => {
            let elapsed = manager.get_task_elapsed_time()
                .map(|d| format!(" ({}m {}s)", d.as_secs() / 60, d.as_secs() % 60))
                .unwrap_or_default();
            
            let approaching_timeout = if manager.is_task_approaching_timeout() {
                " ⚠️ TIMEOUT WARNING"
            } else {
                ""
            };
            
            Ok(format!("Task: {}{}{}", task, elapsed, approaching_timeout))
        },
        None => Ok("No active task".to_string()),
    }
}