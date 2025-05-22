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
    
    /// Get project name from a directory path
    /// 
    /// This attempts to extract a meaningful project name using various strategies:
    /// 1. Git repository name
    /// 2. Directory name (if meaningful)
    /// 3. Parent directory name (if current is not meaningful)
    /// 4. Fallback to "Project"
    pub fn get_project_name(dir: &Path) -> Option<String> {
        // Try git repository name first
        if let Some(git_name) = Self::get_git_project_name(dir) {
            return Some(git_name);
        }
        
        // Try current directory name
        if let Some(dir_name) = dir.file_name() {
            if let Some(name) = dir_name.to_str() {
                if Self::is_meaningful_project_name(name) {
                    return Some(name.to_string());
                }
            }
        }
        
        // Try parent directories
        let mut current = dir;
        while let Some(parent) = current.parent() {
            if let Some(parent_name) = parent.file_name() {
                if let Some(name) = parent_name.to_str() {
                    if Self::is_meaningful_project_name(name) {
                        return Some(name.to_string());
                    }
                }
            }
            current = parent;
        }
        
        // Final fallback
        dir.file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
    }
    
    /// Get project name from git repository
    fn get_git_project_name(dir: &Path) -> Option<String> {
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
            "cache", "temp", "logs", "output", "public", "static",
        ];
        
        !skip_names.contains(&name) && 
        !name.starts_with('.') && 
        name.len() > 2 &&
        !name.chars().all(|c| c.is_ascii_digit()) // Skip purely numeric names
    }
}