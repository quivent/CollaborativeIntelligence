//! State persistence for terminal title system
//!
//! This module handles saving and loading terminal title state between CLI invocations.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Persistent state for terminal title system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleState {
    pub project: Option<String>,
    pub agent: Option<String>,
    pub task: Option<String>,
    pub task_start_time: Option<u64>, // Unix timestamp
    pub last_updated: u64,
}

impl Default for TitleState {
    fn default() -> Self {
        Self {
            project: None,
            agent: None,
            task: None,
            task_start_time: None,
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl TitleState {
    /// Create a new title state with current timestamp
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update the project name
    pub fn set_project(&mut self, project: &str) {
        self.project = Some(project.to_string());
        self.update_timestamp();
    }
    
    /// Update the agent name
    pub fn set_agent(&mut self, agent: &str) {
        self.agent = Some(agent.to_string());
        self.update_timestamp();
    }
    
    /// Update the task description and set start time
    pub fn set_task(&mut self, task: &str) {
        self.task = Some(task.to_string());
        self.task_start_time = Some(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs());
        self.update_timestamp();
    }
    
    /// Clear the task
    pub fn clear_task(&mut self) {
        self.task = None;
        self.task_start_time = None;
        self.update_timestamp();
    }
    
    /// Clear the agent
    pub fn clear_agent(&mut self) {
        self.agent = None;
        self.update_timestamp();
    }
    
    /// Clear all state
    pub fn clear_all(&mut self) {
        self.project = None;
        self.agent = None;
        self.task = None;
        self.task_start_time = None;
        self.update_timestamp();
    }
    
    /// Check if task has timed out (default 60 minutes)
    pub fn is_task_timed_out(&self, timeout_minutes: u32) -> bool {
        if let (Some(_), Some(start_time)) = (&self.task, self.task_start_time) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let timeout_secs = timeout_minutes as u64 * 60;
            return now - start_time > timeout_secs;
        }
        false
    }
    
    /// Get task elapsed time in seconds
    pub fn get_task_elapsed_seconds(&self) -> Option<u64> {
        if let Some(start_time) = self.task_start_time {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            Some(now - start_time)
        } else {
            None
        }
    }
    
    /// Update the last updated timestamp
    fn update_timestamp(&mut self) {
        self.last_updated = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

/// Get the path to the state file
pub fn get_state_file_path() -> PathBuf {
    // Store in user's cache directory
    if let Some(cache_dir) = dirs::cache_dir() {
        cache_dir.join("cia").join("title_state.json")
    } else {
        // Fallback to current directory
        PathBuf::from(".cia_title_state.json")
    }
}

/// Load title state from disk
pub fn load_state() -> Result<TitleState> {
    let state_path = get_state_file_path();
    
    if !state_path.exists() {
        return Ok(TitleState::new());
    }
    
    let contents = fs::read_to_string(&state_path)?;
    let mut state: TitleState = serde_json::from_str(&contents)?;
    
    // Check if task has timed out
    if state.is_task_timed_out(60) {
        state.clear_task();
        // Save the updated state
        let _ = save_state(&state);
    }
    
    Ok(state)
}

/// Save title state to disk
pub fn save_state(state: &TitleState) -> Result<()> {
    let state_path = get_state_file_path();
    
    // Create parent directory if it doesn't exist
    if let Some(parent) = state_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let contents = serde_json::to_string_pretty(state)?;
    fs::write(&state_path, contents)?;
    
    Ok(())
}

/// Load state and apply it to the terminal title manager
pub fn restore_state_to_manager(manager: &mut crate::terminal::TerminalTitleManager) -> Result<()> {
    let state = load_state()?;
    
    if let Some(project) = &state.project {
        manager.update_project(project);
    }
    
    if let Some(agent) = &state.agent {
        manager.update_agent(agent);
    }
    
    if let Some(task) = &state.task {
        manager.update_task(task);
    }
    
    Ok(())
}

/// Save the current manager state to disk
pub fn save_manager_state(manager: &crate::terminal::TerminalTitleManager) -> Result<()> {
    let mut state = TitleState::new();
    
    if let Some(project) = manager.get_project() {
        state.set_project(project);
    }
    
    if let Some(agent) = manager.get_agent() {
        state.set_agent(agent);
    }
    
    if let Some(task) = manager.get_task() {
        state.set_task(task);
    }
    
    save_state(&state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_state_creation() {
        let state = TitleState::new();
        assert!(state.project.is_none());
        assert!(state.agent.is_none());
        assert!(state.task.is_none());
    }
    
    #[test]
    fn test_state_updates() {
        let mut state = TitleState::new();
        
        state.set_project("TestProject");
        state.set_agent("TestAgent");
        state.set_task("TestTask");
        
        assert_eq!(state.project, Some("TestProject".to_string()));
        assert_eq!(state.agent, Some("TestAgent".to_string()));
        assert_eq!(state.task, Some("TestTask".to_string()));
        assert!(state.task_start_time.is_some());
    }
    
    #[test]
    fn test_task_timeout() {
        let mut state = TitleState::new();
        state.set_task("Test Task");
        
        // Should not be timed out immediately
        assert!(!state.is_task_timed_out(60));
        
        // Manually set an old timestamp for testing
        state.task_start_time = Some(0);
        assert!(state.is_task_timed_out(1));
    }
    
    #[test]
    fn test_serialization() {
        let mut state = TitleState::new();
        state.set_project("TestProject");
        state.set_agent("TestAgent");
        state.set_task("TestTask");
        
        let json = serde_json::to_string(&state).unwrap();
        let restored: TitleState = serde_json::from_str(&json).unwrap();
        
        assert_eq!(state.project, restored.project);
        assert_eq!(state.agent, restored.agent);
        assert_eq!(state.task, restored.task);
    }
}