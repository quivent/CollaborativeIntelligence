use crate::helpers::terminal::{TerminalHelpers, ThemeColors};
use crate::helpers::repository::RepositoryHelpers;
use crate::config::Config;
use colored::*;
use anyhow::Result;

pub mod view;

/// Clean temporary files
pub fn clean(cache: bool, config: &Config) -> Result<()> {
    TerminalHelpers::print_fancy_header(
        "System Cleanup",
        "🧹",
        "System Maintenance",
        ThemeColors::SYSTEM
    );
    
    TerminalHelpers::print_info("Cleaning temporary files...");
    
    // Temporary files locations
    let temp_dirs = [
        config.repo_path.join("test_logs"),
        config.repo_path.join(".cache").join("temp"),
    ];
    
    // Clean temporary files
    for dir in &temp_dirs {
        if dir.exists() {
            TerminalHelpers::print_info(&format!("Cleaning directory: {}", dir.display()));
            
            match std::fs::read_dir(dir) {
                Ok(entries) => {
                    let mut removed = 0;
                    
                    for entry in entries.filter_map(Result::ok) {
                        if let Err(e) = std::fs::remove_file(entry.path()) {
                            TerminalHelpers::print_warning(&format!("Failed to remove {}: {}", entry.path().display(), e));
                        } else {
                            removed += 1;
                        }
                    }
                    
                    TerminalHelpers::print_info(&format!("Removed {} files from {}", removed, dir.display()));
                },
                Err(e) => {
                    TerminalHelpers::print_warning(&format!("Failed to read directory {}: {}", dir.display(), e));
                }
            }
        }
    }
    
    // Clean cache if requested
    if cache {
        TerminalHelpers::print_info("Cleaning agent cache...");
        
        let cache_dir = config.repo_path.join(".cache").join("agents");
        
        if cache_dir.exists() {
            match std::fs::remove_dir_all(&cache_dir) {
                Ok(_) => {
                    TerminalHelpers::print_info(&format!("Removed cache directory: {}", cache_dir.display()));
                    
                    // Recreate the cache directory structure
                    match std::fs::create_dir_all(&cache_dir) {
                        Ok(_) => TerminalHelpers::print_info("Recreated empty cache directory"),
                        Err(e) => TerminalHelpers::print_warning(&format!("Failed to recreate cache directory: {}", e)),
                    }
                },
                Err(e) => {
                    TerminalHelpers::print_warning(&format!("Failed to remove cache directory: {}", e));
                }
            }
        } else {
            TerminalHelpers::print_info("No agent cache found, creating directory structure");
            
            // Create cache directory if it doesn't exist
            match std::fs::create_dir_all(&cache_dir) {
                Ok(_) => TerminalHelpers::print_info(&format!("Created cache directory: {}", cache_dir.display())),
                Err(e) => TerminalHelpers::print_warning(&format!("Failed to create cache directory: {}", e)),
            }
        }
        
        TerminalHelpers::print_info("Cache cleaned successfully");
    }
    
    TerminalHelpers::print_success("System cleanup completed");
    Ok(())
}

/// Verify repository structure
pub fn verify(config: &Config) -> Result<()> {
    TerminalHelpers::print_fancy_header(
        "Repository Verification",
        "✅",
        "System Maintenance",
        ThemeColors::SYSTEM
    );
    
    // Check for essential directories
    let essential_dirs = [
        "AGENTS",
        "CLIs",
        "docs",
        "scripts",
    ];
    
    TerminalHelpers::print_info("Checking essential directories...");
    let mut all_dirs_exist = true;
    
    for dir in essential_dirs.iter() {
        let dir_path = config.repo_path.join(dir);
        let exists = dir_path.exists() && dir_path.is_dir();
        println!("- {}: {}", dir, if exists { "✓".green() } else { "✗".red() });
        
        if !exists {
            all_dirs_exist = false;
        }
    }
    
    // Check for essential files
    let essential_files = [
        "README.md",
        "CLAUDE.md",
    ];
    
    TerminalHelpers::print_info("Checking essential files...");
    let mut all_files_exist = true;
    
    for file in essential_files.iter() {
        let file_path = config.repo_path.join(file);
        let exists = file_path.exists() && file_path.is_file();
        println!("- {}: {}", file, if exists { "✓".green() } else { "✗".red() });
        
        if !exists {
            all_files_exist = false;
        }
    }
    
    // Check git repository status
    TerminalHelpers::print_info("Checking git repository status...");
    
    if RepositoryHelpers::is_git_repository(&config.repo_path)? {
        println!("- Git repository: {}", "✓".green());
        
        // Check current branch
        match RepositoryHelpers::get_current_branch(&config.repo_path) {
            Ok(branch) => println!("- Current branch: {}", branch),
            Err(_) => println!("- Current branch: {}", "Unable to determine".red()),
        }
        
        // Check repository status
        match RepositoryHelpers::get_repository_status(&config.repo_path) {
            Ok(status) => {
                println!("- Repository state: {}", 
                    if status.is_clean { "Clean".green() } else { "Changes present".yellow() });
                
                if !status.is_clean {
                    println!("  • Modified files: {}", status.modified_files);
                    println!("  • Untracked files: {}", status.untracked_files);
                    println!("  • Staged files: {}", status.staged_files);
                }
            },
            Err(_) => println!("- Repository state: {}", "Unable to determine".red()),
        }
    } else {
        println!("- Git repository: {}", "✗".red());
        all_dirs_exist = false;
    }
    
    // Print summary
    if all_dirs_exist && all_files_exist {
        TerminalHelpers::print_success("Repository structure verified successfully");
    } else {
        TerminalHelpers::print_warning("Repository structure has issues");
    }
    
    Ok(())
}

/// Update terminal title
pub fn title(title: &Option<String>, config: &Config) -> Result<()> {
    TerminalHelpers::print_fancy_header(
        "Update Terminal Title",
        "📋",
        "System Maintenance",
        ThemeColors::SYSTEM
    );
    
    match title {
        Some(custom_title) => {
            TerminalHelpers::set_terminal_title(custom_title);
            TerminalHelpers::print_success(&format!("Terminal title set to '{}'", custom_title));
        },
        None => {
            // Reset to default title
            let repo_name = config.repo_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("CollaborativeIntelligence");
                
            TerminalHelpers::set_terminal_title(repo_name);
            TerminalHelpers::print_success(&format!("Terminal title reset to '{}'", repo_name));
        }
    }
    
    Ok(())
}

/// Override terminal title with CIA contextual format
pub fn override_title(context: &Option<String>, config: &Config) -> Result<()> {
    // Initialize the terminal title system to get current state
    if let Err(e) = crate::terminal::initialize_title_system() {
        eprintln!("Warning: Failed to initialize terminal title system: {}", e);
    }
    
    // Update the terminal title using the contextual system
    match crate::terminal::update_title() {
        Ok(_) => {
            if let Some(ctx) = context {
                // If context is provided, this might be from Claude Code with task context
                if let Err(e) = crate::terminal::set_current_task(ctx) {
                    eprintln!("Warning: Failed to set task context: {}", e);
                }
            }
            
            // Silent operation - no output to avoid interfering with Claude Code
            // The title has been set via ANSI escape sequences
        },
        Err(e) => {
            eprintln!("Warning: Failed to override terminal title: {}", e);
        }
    }
    
    Ok(())
}