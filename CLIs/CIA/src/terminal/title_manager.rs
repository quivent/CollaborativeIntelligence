//! Terminal title management and formatting
//!
//! This module provides the core TerminalTitleManager for tracking and updating
//! terminal window titles with project, agent, and task context.

use std::io::{self, Write};

/// Configuration for terminal title behavior
#[derive(Debug, Clone)]
pub struct TitleConfig {
    pub format_template: String,
    pub update_enabled: bool,
    pub project_detection: ProjectDetectionMethod,
    pub task_timeout_minutes: Option<u32>,
    pub max_title_length: usize,
}

impl Default for TitleConfig {
    fn default() -> Self {
        Self {
            format_template: "[{project}] | [{agent}] | [{task}]".to_string(),
            update_enabled: true,
            project_detection: ProjectDetectionMethod::DirectoryName,
            task_timeout_minutes: Some(60),
            max_title_length: 100,
        }
    }
}

/// Methods for detecting the current project name
#[derive(Debug, Clone)]
pub enum ProjectDetectionMethod {
    DirectoryName,
    GitRepository,
    ConfigFile,
}

/// Core terminal title manager
/// 
/// Manages the current project, agent, and task context and updates
/// the terminal window title accordingly.
#[derive(Debug)]
pub struct TerminalTitleManager {
    current_project: Option<String>,
    current_agent: Option<String>,
    current_task: Option<String>,
    config: TitleConfig,
    update_enabled: bool,
}

impl TerminalTitleManager {
    /// Create a new terminal title manager with default configuration
    pub fn new() -> Self {
        Self {
            current_project: None,
            current_agent: None,
            current_task: None,
            config: TitleConfig::default(),
            update_enabled: true,
        }
    }
    
    /// Create a new terminal title manager with custom configuration
    pub fn with_config(config: TitleConfig) -> Self {
        Self {
            current_project: None,
            current_agent: None,
            current_task: None,
            update_enabled: config.update_enabled,
            config,
        }
    }
    
    /// Update the current project name
    pub fn update_project(&mut self, project: &str) {
        self.current_project = Some(project.to_string());
    }
    
    /// Update the current agent name
    pub fn update_agent(&mut self, agent: &str) {
        self.current_agent = Some(agent.to_string());
    }
    
    /// Update the current task description
    pub fn update_task(&mut self, task: &str) {
        self.current_task = Some(task.to_string());
    }
    
    /// Clear the current task
    pub fn clear_task(&mut self) {
        self.current_task = None;
    }
    
    /// Clear the current agent
    pub fn clear_agent(&mut self) {
        self.current_agent = None;
    }
    
    /// Clear all context (project, agent, task)
    pub fn clear_all(&mut self) {
        self.current_project = None;
        self.current_agent = None;
        self.current_task = None;
    }
    
    /// Get the current project name
    pub fn get_project(&self) -> Option<&String> {
        self.current_project.as_ref()
    }
    
    /// Get the current agent name
    pub fn get_agent(&self) -> Option<&String> {
        self.current_agent.as_ref()
    }
    
    /// Get the current task description
    pub fn get_task(&self) -> Option<&String> {
        self.current_task.as_ref()
    }
    
    /// Enable or disable title updates
    pub fn set_update_enabled(&mut self, enabled: bool) {
        self.update_enabled = enabled;
    }
    
    /// Check if title updates are enabled
    pub fn is_update_enabled(&self) -> bool {
        self.update_enabled && self.config.update_enabled
    }
    
    /// Get the current formatted title string
    /// 
    /// Formats the title according to the configured template, handling
    /// missing context gracefully.
    pub fn get_current_title(&self) -> String {
        let project = self.current_project.as_deref().unwrap_or("Project");
        let agent = self.current_agent.as_deref().unwrap_or("CLI");
        let task = self.current_task.as_deref().unwrap_or("Ready");
        
        let title = self.config.format_template
            .replace("{project}", project)
            .replace("{agent}", agent)
            .replace("{task}", task);
        
        // Truncate if too long
        if title.len() > self.config.max_title_length {
            let truncated = &title[..self.config.max_title_length.saturating_sub(3)];
            format!("{}...", truncated)
        } else {
            title
        }
    }
    
    /// Update the terminal window title immediately
    /// 
    /// Uses ANSI escape sequences to set the terminal window title.
    /// Fails gracefully if the terminal doesn't support title setting.
    pub fn refresh_title(&self) {
        if !self.is_update_enabled() {
            return;
        }
        
        let title = self.get_current_title();
        
        // Use ANSI escape sequence to set terminal title
        // ESC ] 0 ; title BEL
        print!("\x1B]0;{}\x07", title);
        
        // Ensure the escape sequence is written immediately
        if let Err(e) = io::stdout().flush() {
            // Log error but don't fail - terminal title is not critical
            eprintln!("Warning: Failed to update terminal title: {}", e);
        }
    }
    
    /// Set the terminal title to a specific string (bypass formatting)
    /// 
    /// This is provided for backward compatibility and emergency use.
    /// Normal usage should use the context-aware methods.
    pub fn set_title_direct(&self, title: &str) {
        if !self.is_update_enabled() {
            return;
        }
        
        print!("\x1B]0;{}\x07", title);
        if let Err(e) = io::stdout().flush() {
            eprintln!("Warning: Failed to update terminal title: {}", e);
        }
    }
    
    /// Update configuration
    pub fn update_config(&mut self, config: TitleConfig) {
        self.update_enabled = config.update_enabled;
        self.config = config;
    }
    
    /// Get current configuration
    pub fn get_config(&self) -> &TitleConfig {
        &self.config
    }
    
    /// Create a title with specific context (for testing/preview)
    pub fn format_title_with_context(
        &self,
        project: Option<&str>,
        agent: Option<&str>,
        task: Option<&str>,
    ) -> String {
        let project = project.unwrap_or("Project");
        let agent = agent.unwrap_or("CLI");
        let task = task.unwrap_or("Ready");
        
        let title = self.config.format_template
            .replace("{project}", project)
            .replace("{agent}", agent)
            .replace("{task}", task);
        
        if title.len() > self.config.max_title_length {
            let truncated = &title[..self.config.max_title_length.saturating_sub(3)];
            format!("{}...", truncated)
        } else {
            title
        }
    }
}

impl Default for TerminalTitleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_title_formatting() {
        let manager = TerminalTitleManager::new();
        
        // Test with all context
        let title = manager.format_title_with_context(
            Some("TestProject"),
            Some("TestAgent"),
            Some("TestTask"),
        );
        assert_eq!(title, "[TestProject] | [TestAgent] | [TestTask]");
        
        // Test with defaults
        let title = manager.format_title_with_context(None, None, None);
        assert_eq!(title, "[Project] | [CLI] | [Ready]");
    }
    
    #[test]
    fn test_title_truncation() {
        let mut config = TitleConfig::default();
        config.max_title_length = 20;
        let manager = TerminalTitleManager::with_config(config);
        
        let title = manager.format_title_with_context(
            Some("VeryLongProjectName"),
            Some("VeryLongAgentName"),
            Some("VeryLongTaskName"),
        );
        
        assert!(title.len() <= 20);
        assert!(title.ends_with("..."));
    }
    
    #[test]
    fn test_context_updates() {
        let mut manager = TerminalTitleManager::new();
        
        manager.update_project("TestProject");
        manager.update_agent("TestAgent");
        manager.update_task("TestTask");
        
        assert_eq!(manager.get_project(), Some(&"TestProject".to_string()));
        assert_eq!(manager.get_agent(), Some(&"TestAgent".to_string()));
        assert_eq!(manager.get_task(), Some(&"TestTask".to_string()));
        
        manager.clear_task();
        assert_eq!(manager.get_task(), None);
        
        manager.clear_all();
        assert_eq!(manager.get_project(), None);
        assert_eq!(manager.get_agent(), None);
        assert_eq!(manager.get_task(), None);
    }
    
    #[test]
    fn test_update_enabled() {
        let mut manager = TerminalTitleManager::new();
        
        assert!(manager.is_update_enabled());
        
        manager.set_update_enabled(false);
        assert!(!manager.is_update_enabled());
        
        manager.set_update_enabled(true);
        assert!(manager.is_update_enabled());
    }
}