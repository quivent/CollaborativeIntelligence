//! Configuration module for CIA CLI
//!
//! This module provides structures and functions for managing CIA CLI configuration.

use crate::error::Error;
use std::path::PathBuf;
use crate::helpers::path::PathHelpers;

/// Core global configuration for CIA CLI
#[derive(Clone, Debug)]
pub struct Config {
    /// Path to the repository root
    pub repo_path: PathBuf,
}

impl Config {
    /// Load the global configuration
    pub fn load() -> Result<Self, Error> {
        // Try to get repository path from current directory
        let current_dir = std::env::current_dir()
            .map_err(|e| Error::IoError(e))?;
            
        // Find repository root
        if let Some(repo_path) = PathHelpers::find_repository_root(&current_dir) {
            return Ok(Config { repo_path });
        }
        
        // If not found, try common locations
        let home_dir = dirs::home_dir()
            .ok_or_else(|| Error::ConfigError("Home directory not found".to_string()))?;
            
        let common_paths = vec![
            home_dir.join("Documents/Projects/CollaborativeIntelligence"),
            home_dir.join("Projects/CollaborativeIntelligence"),
            PathBuf::from("/Users/Shared/CollaborativeIntelligence"),
        ];
        
        for path in common_paths {
            if path.exists() && path.join("CLAUDE.md").exists() {
                return Ok(Config { repo_path: path });
            }
        }
        
        Err(Error::ConfigError("Repository not found".to_string()))
    }
    
    /// Get the agents directory path
    #[allow(dead_code)]
    pub fn get_agents_path(&self) -> PathBuf {
        self.repo_path.join("AGENTS")
    }
    
    /// Get the CLIs directory path
    #[allow(dead_code)]
    pub fn get_clis_path(&self) -> PathBuf {
        self.repo_path.join("CLIs")
    }
}