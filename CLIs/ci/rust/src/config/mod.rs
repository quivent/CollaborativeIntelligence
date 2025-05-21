//! Configuration management for CI CLI
//!
//! This module handles loading, parsing, and validating configuration for the CI CLI.

use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use crate::error::{Error, Result};
use crate::helpers::path::PathHelpers;

/// CI CLI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Path to the CI repository
    pub ci_path: PathBuf,
    
    /// Default agents to use
    pub default_agents: Vec<String>,
    
    /// Default integration type
    pub default_integration: String,
    
    /// Enable fast activation by default
    pub fast_activation: bool,
}

impl Config {
    /// Create a new config with default values
    pub fn new() -> Self {
        Config {
            ci_path: PathBuf::new(),
            default_agents: vec!["Athena".to_string(), "ProjectArchitect".to_string()],
            default_integration: "embedded".to_string(),
            fast_activation: true,
        }
    }
    
    /// Load configuration
    pub fn load() -> Result<Config> {
        // First, try to find CI repository
        let ci_path = match PathHelpers::find_ci_repository() {
            Some(path) => path,
            None => return Err(Error::ConfigError("CI repository not found".to_string())),
        };
        
        // Create default config
        let mut config = Config::new();
        config.ci_path = ci_path;
        
        Ok(config)
    }
    
    /// Get path to AGENTS directory
    pub fn get_agents_path(&self) -> PathBuf {
        self.ci_path.join("AGENTS")
    }
    
    /// Get path to memory directory
    pub fn get_memory_path(&self) -> PathBuf {
        self.ci_path.join("memory")
    }
    
    /// Get path to .ci directory for a project
    pub fn get_ci_dir(&self, project_path: &Path) -> PathBuf {
        project_path.join(".ci")
    }
    
    /// Verify CI repository structure
    pub fn verify_ci_repository(&self) -> Result<()> {
        // Check if CI repository exists
        if !self.ci_path.exists() {
            return Err(Error::ConfigError(
                format!("CI repository not found at: {}", self.ci_path.display())
            ));
        }
        
        // Check for CLAUDE.md
        if !self.ci_path.join("CLAUDE.md").exists() {
            return Err(Error::ConfigError(
                format!("CLAUDE.md not found in CI repository: {}", self.ci_path.display())
            ));
        }
        
        // Check for AGENTS directory
        if !self.get_agents_path().exists() {
            return Err(Error::ConfigError(
                format!("AGENTS directory not found in CI repository: {}", self.ci_path.display())
            ));
        }
        
        Ok(())
    }
}