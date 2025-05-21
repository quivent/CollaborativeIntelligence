//! Path utility functions for TL CLI
//!
//! This module provides utility functions for working with file paths.

use std::path::PathBuf;

/// Get the path to the AGENTS directory
#[allow(dead_code)]
pub fn get_agents_path() -> PathBuf {
    let repo_path = crate::helpers::path::PathHelpers::find_repository_root(
        &std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))).unwrap_or_else(|| PathBuf::from("."));
    repo_path.join("AGENTS")
}

/// Check if an agent exists
#[allow(dead_code)]
pub fn agent_exists(name: &str) -> bool {
    let agents_path = get_agents_path();
    agents_path.join(name).exists()
}

/// Get agent categories from directory structure
#[allow(dead_code)]
pub fn get_agent_categories() -> Vec<String> {
    let agents_path = get_agents_path();
    let entries = match std::fs::read_dir(agents_path) {
        Ok(entries) => entries,
        Err(_) => return vec![],
    };
    
    let mut categories = Vec::new();
    for entry in entries.filter_map(Result::ok) {
        if entry.path().is_dir() {
            if let Some(name) = entry.path().file_name() {
                if let Some(name_str) = name.to_str() {
                    categories.push(name_str.to_string());
                }
            }
        }
    }
    
    categories.sort();
    categories
}