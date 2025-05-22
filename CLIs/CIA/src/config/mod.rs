//! Configuration module for CIA CLI
//!
//! This module provides structures and functions for managing CIA CLI configuration.

use crate::error::Error;
use std::path::PathBuf;
use std::time::Duration;
use crate::helpers::path::PathHelpers;

/// Project detection method for terminal titles
#[derive(Clone, Debug)]
pub enum ProjectDetectionMethod {
    /// Use directory name
    DirectoryName,
    /// Use git repository name
    GitRepository,
    /// Use configuration file
    ConfigFile,
}

impl Default for ProjectDetectionMethod {
    fn default() -> Self {
        ProjectDetectionMethod::GitRepository
    }
}

/// Terminal title configuration
#[derive(Clone, Debug)]
pub struct TitleConfig {
    /// Whether terminal title updates are enabled
    pub enabled: bool,
    /// Format template for terminal title
    pub format_template: String,
    /// Project detection method
    pub detection_method: ProjectDetectionMethod,
    /// Task timeout duration (after which tasks are cleared from title)
    pub task_timeout: Duration,
    /// Context monitoring check interval
    pub monitor_interval: Duration,
}

impl Default for TitleConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            format_template: "[{project}] | [{agent}] | [{task}]".to_string(),
            detection_method: ProjectDetectionMethod::default(),
            task_timeout: Duration::from_secs(3600), // 1 hour
            monitor_interval: Duration::from_secs(2), // 2 seconds
        }
    }
}

/// Core global configuration for CIA CLI
#[derive(Clone, Debug)]
pub struct Config {
    /// Path to the repository root
    pub repo_path: PathBuf,
    /// Terminal title configuration
    pub title_config: TitleConfig,
}

impl Config {
    /// Load the global configuration
    pub fn load() -> Result<Self, Error> {
        // Try to get repository path from current directory
        let current_dir = std::env::current_dir()
            .map_err(|e| Error::IoError(e))?;
            
        // Find repository root
        if let Some(repo_path) = PathHelpers::find_repository_root(&current_dir) {
            return Ok(Config { 
                repo_path,
                title_config: TitleConfig::default(),
            });
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
                return Ok(Config { 
                    repo_path: path,
                    title_config: TitleConfig::default(),
                });
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
    
    /// Update title configuration
    pub fn set_title_config(&mut self, title_config: TitleConfig) {
        self.title_config = title_config;
    }
    
    /// Get a reference to the title configuration
    pub fn get_title_config(&self) -> &TitleConfig {
        &self.title_config
    }
    
    /// Get a mutable reference to the title configuration
    pub fn get_title_config_mut(&mut self) -> &mut TitleConfig {
        &mut self.title_config
    }
}