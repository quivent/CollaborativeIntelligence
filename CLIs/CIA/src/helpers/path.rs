//! Path helpers for CIA CLI
//!
//! This module provides helper functions for working with file paths and directories.

use std::path::{Path, PathBuf};
use anyhow::{Result, Context};

/// Helper functions for common path operations
pub struct PathHelpers;

impl PathHelpers {
    /// Get the absolute path from a potentially relative path
    #[allow(dead_code)]
    pub fn absolute_path(path: &Path) -> Result<PathBuf> {
        let canonical = path.canonicalize()
            .with_context(|| format!("Failed to get absolute path for: {}", path.display()))?;
        Ok(canonical)
    }
    
    /// Get the repository root path by walking up the directory tree
    pub fn find_repository_root(start_path: &Path) -> Option<PathBuf> {
        let mut current = start_path.to_path_buf();
        
        loop {
            // Check for common markers of a repository root
            let git_dir = current.join(".git");
            let agents_dir = current.join("AGENTS");
            let claude_md = current.join("CLAUDE.md");
            
            if git_dir.exists() && (agents_dir.exists() || claude_md.exists()) {
                return Some(current);
            }
            
            // Move up one directory
            if !current.pop() {
                // We've reached the root and didn't find a marker
                break;
            }
        }
        
        None
    }
    
    /// Get the agents directory path
    pub fn get_agents_path(repo_root: &Path) -> PathBuf {
        repo_root.join("AGENTS")
    }
    
    /// Get the CLI directory path
    pub fn get_cli_path(repo_root: &Path) -> PathBuf {
        repo_root.join("CLIs")
    }
    
    /// Get agent directories
    #[allow(dead_code)]
    pub fn get_agent_directories(repo_root: &Path) -> Result<Vec<PathBuf>> {
        let agents_dir = Self::get_agents_path(repo_root);
        
        if !agents_dir.exists() {
            return Err(anyhow::anyhow!("AGENTS directory not found at: {}", agents_dir.display()));
        }
        
        let dirs = std::fs::read_dir(&agents_dir)
            .with_context(|| format!("Failed to read AGENTS directory: {}", agents_dir.display()))?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let path = e.path();
                    if path.is_dir() {
                        Some(path)
                    } else {
                        None
                    }
                })
            })
            .collect();
            
        Ok(dirs)
    }
    
    /// Get agent names
    #[allow(dead_code)]
    pub fn get_agent_names(repo_root: &Path) -> Result<Vec<String>> {
        Self::get_agent_directories(repo_root)
            .map(|dirs| {
                dirs.iter()
                    .filter_map(|dir| {
                        dir.file_name()
                            .and_then(|name| name.to_str())
                            .map(|s| s.to_string())
                    })
                    .collect()
            })
    }
}