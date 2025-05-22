//! Task lifecycle tracking and management
//!
//! This module provides task tracking capabilities including lifecycle management,
//! timeout handling, and persistence across sessions.

use std::time::{SystemTime, Duration};
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Status of a tracked task
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Active,
    Paused,
    Completed,
}

/// Context information for a tracked task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskContext {
    pub id: String,
    pub description: String,
    pub agent: String,
    pub start_time: SystemTime,
    pub status: TaskStatus,
    pub timeout_minutes: Option<u32>,
    pub last_activity: SystemTime,
}

impl TaskContext {
    /// Create a new task context
    pub fn new(id: String, description: String, agent: String) -> Self {
        let now = SystemTime::now();
        Self {
            id,
            description,
            agent,
            start_time: now,
            status: TaskStatus::Active,
            timeout_minutes: Some(60), // Default 60 minute timeout
            last_activity: now,
        }
    }
    
    /// Create a new task context with custom timeout
    pub fn with_timeout(
        id: String,
        description: String,
        agent: String,
        timeout_minutes: Option<u32>,
    ) -> Self {
        let now = SystemTime::now();
        Self {
            id,
            description,
            agent,
            start_time: now,
            status: TaskStatus::Active,
            timeout_minutes,
            last_activity: now,
        }
    }
    
    /// Update the last activity time to current time
    pub fn update_activity(&mut self) {
        self.last_activity = SystemTime::now();
    }
    
    /// Check if the task has timed out
    pub fn is_timed_out(&self) -> bool {
        if let Some(timeout_minutes) = self.timeout_minutes {
            if let Ok(elapsed) = self.last_activity.elapsed() {
                return elapsed > Duration::from_secs(timeout_minutes as u64 * 60);
            }
        }
        false
    }
    
    /// Get the elapsed time since task start
    pub fn elapsed_time(&self) -> Option<Duration> {
        self.start_time.elapsed().ok()
    }
    
    /// Get the time since last activity
    pub fn time_since_activity(&self) -> Option<Duration> {
        self.last_activity.elapsed().ok()
    }
    
    /// Mark task as completed
    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
        self.update_activity();
    }
    
    /// Mark task as paused
    pub fn pause(&mut self) {
        self.status = TaskStatus::Paused;
        self.update_activity();
    }
    
    /// Resume a paused task
    pub fn resume(&mut self) {
        if self.status == TaskStatus::Paused {
            self.status = TaskStatus::Active;
            self.update_activity();
        }
    }
    
    /// Get a human-readable status description
    pub fn status_description(&self) -> String {
        match self.status {
            TaskStatus::Active => {
                if let Some(elapsed) = self.elapsed_time() {
                    format!("Active ({})", format_duration(elapsed))
                } else {
                    "Active".to_string()
                }
            }
            TaskStatus::Paused => {
                if let Some(elapsed) = self.elapsed_time() {
                    format!("Paused ({})", format_duration(elapsed))
                } else {
                    "Paused".to_string()
                }
            }
            TaskStatus::Completed => "Completed".to_string(),
        }
    }
}

/// Task tracker for managing multiple tasks
#[derive(Debug, Default)]
pub struct TaskTracker {
    active_tasks: Vec<TaskContext>,
    current_focus: Option<String>, // ID of currently focused task
    completed_tasks: Vec<TaskContext>,
    max_active_tasks: usize,
    max_completed_history: usize,
}

impl TaskTracker {
    /// Create a new task tracker
    pub fn new() -> Self {
        Self {
            active_tasks: Vec::new(),
            current_focus: None,
            completed_tasks: Vec::new(),
            max_active_tasks: 10,
            max_completed_history: 50,
        }
    }
    
    /// Create a new task tracker with limits
    pub fn with_limits(max_active: usize, max_completed: usize) -> Self {
        Self {
            active_tasks: Vec::new(),
            current_focus: None,
            completed_tasks: Vec::new(),
            max_active_tasks: max_active,
            max_completed_history: max_completed,
        }
    }
    
    /// Start a new task and set it as the current focus
    pub fn start_task(
        &mut self,
        description: String,
        agent: String,
        timeout_minutes: Option<u32>,
    ) -> Result<String> {
        // Generate unique task ID
        let id = format!("task_{}", SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());
        
        let task = TaskContext::with_timeout(id.clone(), description, agent, timeout_minutes);
        
        // Remove oldest active task if we're at the limit
        if self.active_tasks.len() >= self.max_active_tasks {
            let oldest = self.active_tasks.remove(0);
            self.completed_tasks.push(oldest);
            self.trim_completed_history();
        }
        
        self.active_tasks.push(task);
        self.current_focus = Some(id.clone());
        
        Ok(id)
    }
    
    /// Complete a task by ID
    pub fn complete_task(&mut self, id: &str) -> Result<()> {
        if let Some(index) = self.active_tasks.iter().position(|t| t.id == id) {
            let mut task = self.active_tasks.remove(index);
            task.complete();
            
            // Clear focus if this was the focused task
            if self.current_focus.as_deref() == Some(id) {
                self.current_focus = None;
            }
            
            self.completed_tasks.push(task);
            self.trim_completed_history();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Task not found: {}", id))
        }
    }
    
    /// Complete the currently focused task
    pub fn complete_current_task(&mut self) -> Result<()> {
        if let Some(id) = self.current_focus.clone() {
            self.complete_task(&id)
        } else {
            Err(anyhow::anyhow!("No task currently focused"))
        }
    }
    
    /// Pause a task by ID
    pub fn pause_task(&mut self, id: &str) -> Result<()> {
        if let Some(task) = self.active_tasks.iter_mut().find(|t| t.id == id) {
            task.pause();
            
            // Clear focus if this was the focused task
            if self.current_focus.as_deref() == Some(id) {
                self.current_focus = None;
            }
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Task not found: {}", id))
        }
    }
    
    /// Resume a task by ID
    pub fn resume_task(&mut self, id: &str) -> Result<()> {
        if let Some(task) = self.active_tasks.iter_mut().find(|t| t.id == id) {
            task.resume();
            self.current_focus = Some(id.to_string());
            Ok(())
        } else {
            Err(anyhow::anyhow!("Task not found: {}", id))
        }
    }
    
    /// Set the current focus to a specific task
    pub fn set_focus(&mut self, id: &str) -> Result<()> {
        if self.active_tasks.iter().any(|t| t.id == id && t.status == TaskStatus::Active) {
            self.current_focus = Some(id.to_string());
            
            // Update activity time for focused task
            if let Some(task) = self.active_tasks.iter_mut().find(|t| t.id == id) {
                task.update_activity();
            }
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Task not found or not active: {}", id))
        }
    }
    
    /// Clear the current focus
    pub fn clear_focus(&mut self) {
        self.current_focus = None;
    }
    
    /// Get the currently focused task
    pub fn get_focused_task(&self) -> Option<&TaskContext> {
        if let Some(id) = &self.current_focus {
            self.active_tasks.iter().find(|t| t.id == *id)
        } else {
            None
        }
    }
    
    /// Get all active tasks
    pub fn get_active_tasks(&self) -> &[TaskContext] {
        &self.active_tasks
    }
    
    /// Get completed tasks
    pub fn get_completed_tasks(&self) -> &[TaskContext] {
        &self.completed_tasks
    }
    
    /// Get task by ID (active or completed)
    pub fn get_task(&self, id: &str) -> Option<&TaskContext> {
        self.active_tasks.iter().find(|t| t.id == id)
            .or_else(|| self.completed_tasks.iter().find(|t| t.id == id))
    }
    
    /// Remove timed out tasks
    pub fn cleanup_timed_out_tasks(&mut self) -> Vec<TaskContext> {
        let mut timed_out = Vec::new();
        
        // Find timed out tasks
        let mut i = 0;
        while i < self.active_tasks.len() {
            if self.active_tasks[i].is_timed_out() {
                let mut task = self.active_tasks.remove(i);
                task.status = TaskStatus::Completed;
                
                // Clear focus if this was the focused task
                if self.current_focus.as_deref() == Some(&task.id) {
                    self.current_focus = None;
                }
                
                timed_out.push(task.clone());
                self.completed_tasks.push(task);
            } else {
                i += 1;
            }
        }
        
        self.trim_completed_history();
        timed_out
    }
    
    /// Get a summary of current task status
    pub fn get_status_summary(&self) -> String {
        let active_count = self.active_tasks.len();
        let completed_count = self.completed_tasks.len();
        
        if let Some(focused) = self.get_focused_task() {
            format!(
                "Active: {} | Completed: {} | Current: {}",
                active_count, completed_count, focused.description
            )
        } else if active_count > 0 {
            format!("Active: {} | Completed: {} | No focus", active_count, completed_count)
        } else {
            format!("No active tasks | Completed: {}", completed_count)
        }
    }
    
    /// Trim completed task history to stay within limits
    fn trim_completed_history(&mut self) {
        if self.completed_tasks.len() > self.max_completed_history {
            let excess = self.completed_tasks.len() - self.max_completed_history;
            self.completed_tasks.drain(0..excess);
        }
    }
}

/// Format a duration in a human-readable way
fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    
    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration as StdDuration;
    
    #[test]
    fn test_task_creation() {
        let task = TaskContext::new(
            "test_id".to_string(),
            "Test task".to_string(),
            "TestAgent".to_string(),
        );
        
        assert_eq!(task.id, "test_id");
        assert_eq!(task.description, "Test task");
        assert_eq!(task.agent, "TestAgent");
        assert_eq!(task.status, TaskStatus::Active);
        assert_eq!(task.timeout_minutes, Some(60));
    }
    
    #[test]
    fn test_task_lifecycle() {
        let mut task = TaskContext::new(
            "test_id".to_string(),
            "Test task".to_string(),
            "TestAgent".to_string(),
        );
        
        assert_eq!(task.status, TaskStatus::Active);
        
        task.pause();
        assert_eq!(task.status, TaskStatus::Paused);
        
        task.resume();
        assert_eq!(task.status, TaskStatus::Active);
        
        task.complete();
        assert_eq!(task.status, TaskStatus::Completed);
    }
    
    #[test]
    fn test_task_tracker() {
        let mut tracker = TaskTracker::new();
        
        // Start a task
        let id1 = tracker.start_task(
            "Task 1".to_string(),
            "Agent1".to_string(),
            None,
        ).unwrap();
        
        assert_eq!(tracker.get_active_tasks().len(), 1);
        assert_eq!(tracker.current_focus, Some(id1.clone()));
        
        // Start another task
        let id2 = tracker.start_task(
            "Task 2".to_string(),
            "Agent2".to_string(),
            None,
        ).unwrap();
        
        assert_eq!(tracker.get_active_tasks().len(), 2);
        assert_eq!(tracker.current_focus, Some(id2.clone()));
        
        // Complete first task
        tracker.complete_task(&id1).unwrap();
        assert_eq!(tracker.get_active_tasks().len(), 1);
        assert_eq!(tracker.get_completed_tasks().len(), 1);
        
        // Current focus should still be id2
        assert_eq!(tracker.current_focus, Some(id2));
    }
    
    #[test]
    fn test_task_timeout() {
        let mut task = TaskContext::with_timeout(
            "test_id".to_string(),
            "Test task".to_string(),
            "TestAgent".to_string(),
            Some(0), // 0 minute timeout for testing
        );
        
        // Task should not be timed out immediately
        assert!(!task.is_timed_out());
        
        // Sleep a bit and check timeout
        thread::sleep(StdDuration::from_millis(1));
        assert!(task.is_timed_out());
    }
    
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(30)), "30s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m");
    }
}