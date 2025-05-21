//! Repository helpers for TL CLI
//!
//! This module provides helper functions for working with the repository.

use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use std::process::Command;

/// Repository status information
#[derive(Debug, Clone)]
pub struct RepositoryStatus {
    /// Repository root path
    #[allow(dead_code)]
    pub root_path: PathBuf,
    
    /// Current branch name
    #[allow(dead_code)]
    pub current_branch: String,
    
    /// Number of modified files
    pub modified_files: usize,
    
    /// Number of untracked files
    pub untracked_files: usize,
    
    /// Number of staged files
    pub staged_files: usize,
    
    /// Is the repository clean (no changes)
    pub is_clean: bool,
}

/// Helper functions for repository operations
pub struct RepositoryHelpers;

impl RepositoryHelpers {
    /// Check if a path is a git repository
    pub fn is_git_repository(path: &Path) -> Result<bool> {
        Ok(path.join(".git").exists())
    }
    
    /// Get the current git branch name
    pub fn get_current_branch(repo_path: &Path) -> Result<String> {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(repo_path)
            .output()
            .context("Failed to execute git command")?;
            
        if output.status.success() {
            let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(branch)
        } else {
            Err(anyhow::anyhow!("Failed to get current branch"))
        }
    }
    
    /// Get a summary of the repository status
    pub fn get_repository_status(repo_path: &Path) -> Result<RepositoryStatus> {
        // Check if the path is a git repository
        if !Self::is_git_repository(repo_path)? {
            return Err(anyhow::anyhow!("Not a git repository: {}", repo_path.display()));
        }
        
        // Get current branch
        let current_branch = Self::get_current_branch(repo_path)?;
        
        // Get status information
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(repo_path)
            .output()
            .context("Failed to execute git status command")?;
            
        if !output.status.success() {
            return Err(anyhow::anyhow!("Failed to get git status"));
        }
        
        let status_output = String::from_utf8_lossy(&output.stdout).to_string();
        let status_lines: Vec<&str> = status_output.lines().collect();
        
        // Count different types of changes
        let mut modified_files = 0;
        let mut untracked_files = 0;
        let mut staged_files = 0;
        
        for line in &status_lines {
            if line.len() < 2 {
                continue;
            }
            
            let status_code = &line[0..2];
            match status_code {
                "??" => untracked_files += 1,
                " M" => modified_files += 1,
                "MM" => {
                    modified_files += 1;
                    staged_files += 1;
                },
                "M " => staged_files += 1,
                "A " => staged_files += 1,
                "D " => staged_files += 1,
                "R " => staged_files += 1,
                "C " => staged_files += 1,
                _ => {}
            }
        }
        
        // Determine if the repository is clean
        let is_clean = status_lines.is_empty();
        
        Ok(RepositoryStatus {
            root_path: repo_path.to_path_buf(),
            current_branch,
            modified_files,
            untracked_files,
            staged_files,
            is_clean,
        })
    }
    
    /// Get the latest commit message
    #[allow(dead_code)]
    pub fn get_latest_commit_message(repo_path: &Path) -> Result<String> {
        let output = Command::new("git")
            .args(["log", "-1", "--pretty=%B"])
            .current_dir(repo_path)
            .output()
            .context("Failed to execute git log command")?;
            
        if output.status.success() {
            let message = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(message)
        } else {
            Err(anyhow::anyhow!("Failed to get latest commit message"))
        }
    }
    
    /// Check for the existence of CLAUDE.md
    pub fn check_claude_md(repo_path: &Path) -> bool {
        repo_path.join("CLAUDE.md").exists()
    }
    
    /// Check for the existence of AGENTS directory
    pub fn check_agents_directory(repo_path: &Path) -> bool {
        repo_path.join("AGENTS").exists() && repo_path.join("AGENTS").is_dir()
    }
    
    /// Verify repository structure
    pub fn verify_repository_structure(repo_path: &Path) -> Result<Vec<String>> {
        let mut issues = Vec::new();
        
        // Check for required directories and files
        if !Self::check_claude_md(repo_path) {
            issues.push("CLAUDE.md file is missing".to_string());
        }
        
        if !Self::check_agents_directory(repo_path) {
            issues.push("AGENTS directory is missing".to_string());
        }
        
        if !repo_path.join("CLIs").exists() {
            issues.push("CLIs directory is missing".to_string());
        }
        
        if !repo_path.join("README.md").exists() {
            issues.push("README.md file is missing".to_string());
        }
        
        Ok(issues)
    }
    
    /// Get all agent directories
    #[allow(dead_code)]
    pub fn get_agent_directories(repo_path: &Path) -> Result<Vec<PathBuf>> {
        let agents_dir = repo_path.join("AGENTS");
        
        if !agents_dir.exists() || !agents_dir.is_dir() {
            return Err(anyhow::anyhow!("AGENTS directory not found"));
        }
        
        let entries = std::fs::read_dir(agents_dir)
            .context("Failed to read AGENTS directory")?;
            
        let mut directories = Vec::new();
        
        for entry in entries {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            
            if path.is_dir() {
                directories.push(path);
            }
        }
        
        Ok(directories)
    }
}