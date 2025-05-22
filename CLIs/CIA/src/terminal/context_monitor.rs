//! Context monitoring for automatic terminal title updates
//!
//! This module provides context monitoring capabilities including directory change
//! detection and agent switching detection for automatic title updates.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use anyhow::Result;
use crate::terminal::TerminalTitleManager;

/// Context monitor for tracking changes that should trigger title updates
#[derive(Debug)]
pub struct ContextMonitor {
    last_directory: PathBuf,
    last_agent: Option<String>,
    title_manager: Arc<Mutex<TerminalTitleManager>>,
    monitoring_enabled: bool,
    check_interval: Duration,
}

impl ContextMonitor {
    /// Create a new context monitor with the given title manager
    pub fn new(title_manager: Arc<Mutex<TerminalTitleManager>>) -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        
        Self {
            last_directory: current_dir,
            last_agent: None,
            title_manager,
            monitoring_enabled: false,
            check_interval: Duration::from_secs(1), // Check every second
        }
    }
    
    /// Create a new context monitor with custom check interval
    pub fn with_interval(
        title_manager: Arc<Mutex<TerminalTitleManager>>,
        check_interval: Duration,
    ) -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        
        Self {
            last_directory: current_dir,
            last_agent: None,
            title_manager,
            monitoring_enabled: false,
            check_interval,
        }
    }
    
    /// Start monitoring in a background thread
    /// 
    /// This spawns a background thread that periodically checks for context changes
    /// and updates the terminal title accordingly.
    pub fn start_monitoring(&mut self) -> Result<()> {
        if self.monitoring_enabled {
            return Ok(()); // Already monitoring
        }
        
        self.monitoring_enabled = true;
        let title_manager = Arc::clone(&self.title_manager);
        let check_interval = self.check_interval;
        let mut last_directory = self.last_directory.clone();
        let mut last_agent = self.last_agent.clone();
        
        thread::spawn(move || {
            loop {
                // Check if monitoring is still enabled by trying to access title manager
                if let Ok(manager) = title_manager.try_lock() {
                    if !manager.is_update_enabled() {
                        break; // Stop monitoring if updates are disabled
                    }
                    drop(manager); // Release lock quickly
                } else {
                    // If we can't get the lock, just continue
                    thread::sleep(check_interval);
                    continue;
                }
                
                // Check for directory changes
                if let Ok(current_dir) = std::env::current_dir() {
                    if current_dir != last_directory {
                        if let Err(e) = Self::handle_directory_change(
                            &title_manager,
                            &current_dir,
                        ) {
                            eprintln!("Warning: Failed to handle directory change: {}", e);
                        }
                        last_directory = current_dir;
                    }
                }
                
                // Check for agent changes (this would need integration with agent state)
                // For now, we just track what we know
                if let Ok(manager) = title_manager.try_lock() {
                    let current_agent = manager.get_agent().cloned();
                    if current_agent != last_agent {
                        last_agent = current_agent;
                        // Agent change detected, title should already be updated by agent activation
                    }
                    drop(manager);
                }
                
                thread::sleep(check_interval);
            }
        });
        
        Ok(())
    }
    
    /// Stop monitoring
    pub fn stop_monitoring(&mut self) {
        self.monitoring_enabled = false;
    }
    
    /// Check for changes manually (single check)
    pub fn check_for_changes(&mut self) -> Result<bool> {
        let mut changed = false;
        
        // Check directory change
        if let Ok(current_dir) = std::env::current_dir() {
            if current_dir != self.last_directory {
                self.on_directory_change(&current_dir)?;
                self.last_directory = current_dir;
                changed = true;
            }
        }
        
        // Check agent change
        if let Ok(manager) = self.title_manager.try_lock() {
            let current_agent = manager.get_agent().cloned();
            if current_agent != self.last_agent {
                self.last_agent = current_agent;
                changed = true;
            }
        }
        
        Ok(changed)
    }
    
    /// Handle directory change event
    pub fn on_directory_change(&mut self, new_dir: &Path) -> Result<()> {
        Self::handle_directory_change(&self.title_manager, new_dir)
    }
    
    /// Handle agent change event
    pub fn on_agent_change(&mut self, agent: &str) -> Result<()> {
        if let Ok(mut manager) = self.title_manager.lock() {
            manager.update_agent(agent);
            manager.refresh_title();
            self.last_agent = Some(agent.to_string());
        }
        Ok(())
    }
    
    /// Internal helper to handle directory changes
    fn handle_directory_change(
        title_manager: &Arc<Mutex<TerminalTitleManager>>,
        new_dir: &Path,
    ) -> Result<()> {
        // Extract project name from directory
        let project_name = Self::extract_project_name(new_dir);
        
        if let Ok(mut manager) = title_manager.lock() {
            manager.update_project(&project_name);
            manager.refresh_title();
        }
        
        Ok(())
    }
    
    /// Extract project name from a directory path
    fn extract_project_name(dir: &Path) -> String {
        // Try different methods to get a meaningful project name
        
        // Method 1: Check if it's a git repository
        if let Some(git_name) = Self::get_git_project_name(dir) {
            return git_name;
        }
        
        // Method 2: Use directory name
        if let Some(dir_name) = dir.file_name() {
            if let Some(name) = dir_name.to_str() {
                if !name.is_empty() && name != "." && name != ".." {
                    return name.to_string();
                }
            }
        }
        
        // Method 3: Check parent directories for meaningful names
        let mut current = dir;
        while let Some(parent) = current.parent() {
            if let Some(parent_name) = parent.file_name() {
                if let Some(name) = parent_name.to_str() {
                    if Self::is_meaningful_project_name(name) {
                        return name.to_string();
                    }
                }
            }
            current = parent;
        }
        
        // Fallback: Use the directory name or "Project"
        dir.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Project")
            .to_string()
    }
    
    /// Try to get project name from git repository
    fn get_git_project_name(dir: &Path) -> Option<String> {
        // Check if we're in a git repository
        let mut current = dir;
        loop {
            let git_dir = current.join(".git");
            if git_dir.exists() {
                // Found git directory, use this directory name as project name
                return current.file_name()
                    .and_then(|name| name.to_str())
                    .map(|s| s.to_string());
            }
            
            if let Some(parent) = current.parent() {
                current = parent;
            } else {
                break;
            }
        }
        None
    }
    
    /// Check if a directory name looks like a meaningful project name
    fn is_meaningful_project_name(name: &str) -> bool {
        // Skip common non-project directories
        let skip_names = [
            "bin", "usr", "opt", "var", "tmp", "etc", "home", "root",
            "Applications", "Library", "System", "Users", "Volumes",
            "src", "lib", "include", "share", "docs", "test", "tests",
            "node_modules", "target", "build", "dist", ".git", ".svn",
        ];
        
        !skip_names.contains(&name) && 
        !name.starts_with('.') && 
        name.len() > 2
    }
    
    /// Get the current monitoring status
    pub fn is_monitoring(&self) -> bool {
        self.monitoring_enabled
    }
    
    /// Set the check interval for monitoring
    pub fn set_check_interval(&mut self, interval: Duration) {
        self.check_interval = interval;
    }
    
    /// Get the current check interval
    pub fn get_check_interval(&self) -> Duration {
        self.check_interval
    }
    
    /// Force a title refresh with current context
    pub fn refresh_title(&self) -> Result<()> {
        if let Ok(manager) = self.title_manager.lock() {
            manager.refresh_title();
        }
        Ok(())
    }
}

impl Drop for ContextMonitor {
    fn drop(&mut self) {
        self.stop_monitoring();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use crate::terminal::{TerminalTitleManager, TitleConfig};
    
    #[test]
    fn test_context_monitor_creation() {
        let title_manager = Arc::new(Mutex::new(TerminalTitleManager::new()));
        let monitor = ContextMonitor::new(title_manager);
        
        assert!(!monitor.is_monitoring());
        assert_eq!(monitor.get_check_interval(), Duration::from_secs(1));
    }
    
    #[test]
    fn test_project_name_extraction() {
        let path = Path::new("/Users/test/Projects/MyProject");
        let project_name = ContextMonitor::extract_project_name(path);
        assert_eq!(project_name, "MyProject");
        
        let path = Path::new("/tmp");
        let project_name = ContextMonitor::extract_project_name(path);
        assert_eq!(project_name, "tmp"); // fallback to directory name
    }
    
    #[test]
    fn test_meaningful_project_name() {
        assert!(ContextMonitor::is_meaningful_project_name("MyProject"));
        assert!(ContextMonitor::is_meaningful_project_name("CollaborativeIntelligence"));
        
        assert!(!ContextMonitor::is_meaningful_project_name("bin"));
        assert!(!ContextMonitor::is_meaningful_project_name("usr"));
        assert!(!ContextMonitor::is_meaningful_project_name(".git"));
        assert!(!ContextMonitor::is_meaningful_project_name("node_modules"));
    }
    
    #[test]
    fn test_directory_change_handling() {
        let title_manager = Arc::new(Mutex::new(TerminalTitleManager::new()));
        let mut monitor = ContextMonitor::new(Arc::clone(&title_manager));
        
        let test_path = Path::new("/Users/test/Projects/TestProject");
        monitor.on_directory_change(test_path).unwrap();
        
        let manager = title_manager.lock().unwrap();
        assert_eq!(manager.get_project(), Some(&"TestProject".to_string()));
    }
    
    #[test]
    fn test_agent_change_handling() {
        let title_manager = Arc::new(Mutex::new(TerminalTitleManager::new()));
        let mut monitor = ContextMonitor::new(Arc::clone(&title_manager));
        
        monitor.on_agent_change("TestAgent").unwrap();
        
        let manager = title_manager.lock().unwrap();
        assert_eq!(manager.get_agent(), Some(&"TestAgent".to_string()));
    }
}