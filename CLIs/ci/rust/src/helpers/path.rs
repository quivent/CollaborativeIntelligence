//! Path helper utilities for CI CLI
//!
//! This module provides helper functions for working with file paths and directories.

use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use crate::error::Error;

/// Helper functions for common path operations
pub struct PathHelpers;

#[allow(dead_code)]
impl PathHelpers {
    /// Get the absolute path from a potentially relative path
    pub fn absolute_path(path: &Path) -> Result<PathBuf> {
        let canonical = path.canonicalize()
            .with_context(|| format!("Failed to get absolute path for: {}", path.display()))?;
        Ok(canonical)
    }
    
    /// Get the CI repository path using environment variables or default locations
    pub fn find_ci_repository() -> Option<PathBuf> {
        // Try environment variable first
        if let Ok(path) = std::env::var("CI_PATH") {
            let path_buf = PathBuf::from(path);
            if path_buf.exists() && path_buf.is_dir() {
                return Some(path_buf);
            }
        }
        
        // Try common locations
        let home_dir = dirs::home_dir()?;
        
        let common_locations = [
            home_dir.join("CollaborativeIntelligence"),
            home_dir.join("ci"),
            home_dir.join("Projects/CollaborativeIntelligence"),
            home_dir.join("Documents/CollaborativeIntelligence"),
            home_dir.join("Development/CollaborativeIntelligence"),
        ];
        
        for location in common_locations {
            if location.exists() && location.is_dir() && 
               location.join("CLAUDE.md").exists() {
                return Some(location);
            }
        }
        
        None
    }
    
    /// Check if a path contains a CI-integrated project
    pub fn is_ci_project(path: &Path) -> bool {
        let claude_md = path.join("CLAUDE.md");
        claude_md.exists() && claude_md.is_file()
    }
    
    /// Get the path to the agent directory inside a CI repository
    pub fn get_agents_path(repo_path: &Path) -> PathBuf {
        repo_path.join("AGENTS")
    }
    
    /// Check if an agent exists in the CI repository
    pub fn agent_exists(repo_path: &Path, agent_name: &str) -> bool {
        let agents_path = Self::get_agents_path(repo_path);
        agents_path.join(agent_name).exists()
    }
    
    /// Get the path to the memory directory inside a CI repository
    pub fn get_memory_path(repo_path: &Path) -> PathBuf {
        repo_path.join("memory")
    }
    
    /// Get the path to the .ci directory inside a project
    pub fn get_ci_dir(project_path: &Path) -> PathBuf {
        project_path.join(".ci")
    }
    
    /// Get a list of all agent names in the CI repository
    pub fn get_agent_names(repo_path: &Path) -> Result<Vec<String>, Error> {
        let agents_path = Self::get_agents_path(repo_path);
        
        if !agents_path.exists() {
            return Err(Error::ConfigError(format!(
                "AGENTS directory not found at: {}", 
                agents_path.display()
            )));
        }
        
        let entries = std::fs::read_dir(&agents_path)
            .map_err(|e| Error::IoError(e))?;
            
        let mut agent_names = Vec::new();
        
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name() {
                        if let Some(name_str) = name.to_str() {
                            agent_names.push(name_str.to_string());
                        }
                    }
                }
            }
        }
        
        agent_names.sort();
        Ok(agent_names)
    }
}