use std::error::Error;
use std::path::{Path, PathBuf};
use std::env;
use colored::Colorize;

// Terminal styling utilities

/// Prints a success message with green checkmark
pub fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

/// Prints an error message with red cross
pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message);
}

/// Prints a warning message with yellow exclamation
pub fn print_warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message);
}

/// Prints an info message with blue info symbol
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

/// Prints a step indicator with counter
pub fn print_step(step: usize, total: usize, message: &str) {
    println!("{} {}", format!("[{}/{}]", step, total).cyan().bold(), message);
}

// Path utilities

/// Gets the repository root path
pub fn get_repo_path() -> Result<PathBuf, Box<dyn Error>> {
    // Try current directory
    let current_dir = env::current_dir()?;
    
    // Walk up the directory tree looking for .ci or .git
    let mut path = current_dir.clone();
    loop {
        if path.join(".ci").exists() || path.join(".git").exists() {
            return Ok(path);
        }
        
        // Try parent directory
        if let Some(parent) = path.parent() {
            path = parent.to_path_buf();
        } else {
            // Reached the root, use current directory as fallback
            return Ok(current_dir);
        }
    }
}

/// Gets the agent directory path
pub fn get_agent_dir(agent_name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let repo_path = get_repo_path()?;
    let agent_dir = repo_path.join("AGENTS").join(agent_name);
    
    if !agent_dir.exists() {
        return Err(format!("Agent directory not found: {}", agent_dir.display()).into());
    }
    
    Ok(agent_dir)
}

/// Gets the cache directory path
pub fn get_cache_dir() -> Result<PathBuf, Box<dyn Error>> {
    let repo_path = get_repo_path()?;
    let cache_dir = repo_path.join("CACHE");
    
    // Create if it doesn't exist
    if !cache_dir.exists() {
        std::fs::create_dir_all(&cache_dir)?;
    }
    
    Ok(cache_dir)
}

/// Gets the knowledge registry path
pub fn get_knowledge_dir() -> Result<PathBuf, Box<dyn Error>> {
    let repo_path = get_repo_path()?;
    let knowledge_dir = repo_path.join("KnowledgeRegistry");
    
    if !knowledge_dir.exists() {
        return Err(format!("Knowledge registry not found: {}", knowledge_dir.display()).into());
    }
    
    Ok(knowledge_dir)
}

// Terminal title management

/// Updates the terminal title
pub fn update_terminal_title(title: &str) {
    if atty::is(atty::Stream::Stdout) {
        print!("\u{001b}]0;{}\u{0007}", title);
    }
}

/// Formats an agent response with bracketed headers
pub fn format_agent_response(agent_name: &str, message: &str) -> String {
    let agent_upper = agent_name.to_uppercase();
    format!("[{}]: {}\n\n-- [{}]", agent_upper, message, agent_upper)
}