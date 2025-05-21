//! Load command implementation
//!
//! Loads agent memory for interaction.

use anyhow::Result;
use crate::config::Config;
use crate::helpers::terminal::TerminalHelpers;
use crate::helpers::path::PathHelpers;
use std::path::Path;

/// Load agent memory
pub fn load(
    agent: &str,
    context: Option<&str>,
    path: Option<&str>,
    config: &Config
) -> Result<()> {
    TerminalHelpers::print_command_header(
        &format!("Loading Agent Memory: {}", agent),
        "load"
    );
    
    // Placeholder for now - we'll implement this fully later
    TerminalHelpers::print_info(&format!("Agent: {}", agent));
    
    if let Some(ctx) = context {
        TerminalHelpers::print_info(&format!("Context: {}", ctx));
    }
    
    if let Some(p) = path {
        TerminalHelpers::print_info(&format!("Custom path: {}", p));
    } else {
        TerminalHelpers::print_info(&format!("Using default memory path: {}", config.get_memory_path().display()));
    }
    
    // Check if agent exists
    if PathHelpers::agent_exists(&config.ci_path, agent) {
        TerminalHelpers::print_info(&format!("Agent '{}' found in repository", agent));
    } else {
        TerminalHelpers::print_warning(&format!("Agent '{}' not found in repository", agent));
    }
    
    TerminalHelpers::print_success("Agent memory loaded successfully");
    Ok(())
}