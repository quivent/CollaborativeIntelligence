use crate::helpers::CommandHelpers;
use crate::helpers::path::PathHelpers;
use crate::helpers::repository::RepositoryHelpers;
use crate::helpers::terminal::{TerminalHelpers, ThemeColors, Symbols};
use crate::config::Config;
use colored::*;
use std::env;
use anyhow::Result;
use std::thread;
use std::time::Duration;

/// Print repo path information
pub fn paths(config: &Config) -> Result<()> {
    TerminalHelpers::print_command_header(
        "Repository Path Information",
        "debug"
    );
    
    // Get and display repo root path
    println!("{}: {}", "Repository Root".green().bold(), config.repo_path.display());
    
    // Display AGENTS directory path
    let agents_path = PathHelpers::get_agents_path(&config.repo_path);
    println!("{}: {}", "AGENTS Directory".green().bold(), agents_path.display());
    
    // Display interfaces directory path
    let interfaces_path = PathHelpers::get_cli_path(&config.repo_path);
    println!("{}: {}", "Interfaces Directory".green().bold(), interfaces_path.display());
    
    // Display current working directory
    let current_dir = env::current_dir()
        .map_err(|e| anyhow::anyhow!("Failed to get current directory: {}", e))?;
    println!("{}: {}", "Current Directory".green().bold(), current_dir.display());
    
    // Display git information if available
    if RepositoryHelpers::is_git_repository(&config.repo_path)? {
        println!("\n{}", "Git Information:".yellow().bold());
        
        // Get current branch
        match RepositoryHelpers::get_current_branch(&config.repo_path) {
            Ok(branch) => println!("- Current branch: {}", branch.cyan()),
            Err(_) => println!("- Current branch: {}", "Unable to determine".red()),
        }
        
        // Get repository status
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
    }
    
    TerminalHelpers::print_success("Path information displayed");
    Ok(())
}

/// Check system configuration
pub fn config(config: &Config) -> Result<()> {
    TerminalHelpers::print_command_header(
        "System Configuration",
        "system"
    );
    
    // Display CLI information
    println!("{}", "CLI Information:".cyan().bold());
    println!("- CIA CLI Version: {}", env!("CARGO_PKG_VERSION"));
    println!("- Rust Version: {}", env!("CARGO_PKG_RUST_VERSION", "unknown"));
    
    // Check for repository structure
    println!("\n{}", "Repository Structure:".yellow().bold());
    
    let issues = RepositoryHelpers::verify_repository_structure(&config.repo_path)?;
    
    if issues.is_empty() {
        println!("✓ {}", "Repository structure is valid".green());
    } else {
        println!("⚠ {}", "Repository structure has issues:".yellow());
        for issue in issues {
            println!("  • {}", issue.yellow());
        }
    }
    
    // Display environment information
    println!("\n{}", "Environment Information:".cyan().bold());
    println!("- Operating System: {}", env::consts::OS);
    println!("- Architecture: {}", env::consts::ARCH);
    
    // Check for command availability
    println!("\n{}", "Command Availability:".cyan().bold());
    println!("- git: {}", if CommandHelpers::command_exists("git") { "✓".green() } else { "✗".red() });
    println!("- cargo: {}", if CommandHelpers::command_exists("cargo") { "✓".green() } else { "✗".red() });
    println!("- rustc: {}", if CommandHelpers::command_exists("rustc") { "✓".green() } else { "✗".red() });
    
    TerminalHelpers::print_success("Configuration check completed");
    Ok(())
}

/// Display visual map of CLI commands
pub fn map(_config: &Config) -> Result<()> {
    TerminalHelpers::print_command_header(
        "CLI Command Structure Map",
        "debug"
    );
    
    // Define our command structure
    println!("\n{}", "Command Hierarchy:".yellow().bold());
    
    // Use box drawing characters to create a tree structure
    let agent_color = "cyan";
    let memory_color = "magenta";
    let knowledge_color = "green";
    let debug_color = "yellow";
    let system_color = "blue";
    
    // Show the main CIA command
    println!("{}", format!("{}  CIA", Symbols::ARROW).bold());
    
    // Agent commands branch
    println!("{}{} {}", 
        Symbols::T_RIGHT, 
        Symbols::HORIZONTAL,
        format!("{} {}", Symbols::AGENT, "Agent").color(agent_color).bold());
    println!("{}  {}{}  activate <name>  - Activate an agent", 
        Symbols::VERTICAL, Symbols::T_RIGHT, Symbols::HORIZONTAL);
    println!("{}  {}{}  list             - List all available agents", 
        Symbols::VERTICAL, Symbols::T_RIGHT, Symbols::HORIZONTAL);
    println!("{}  {}{}  create <name>    - Create a new agent", 
        Symbols::VERTICAL, Symbols::BOTTOM_LEFT, Symbols::HORIZONTAL);
    
    // Memory commands branch
    println!("{}{} {}", 
        Symbols::T_RIGHT, 
        Symbols::HORIZONTAL,
        format!("{} {}", Symbols::MEMORY, "Memory").color(memory_color).bold());
    println!("{}  {}{}  load <agent>     - Load agent memory", 
        Symbols::VERTICAL, Symbols::T_RIGHT, Symbols::HORIZONTAL);
    println!("{}  {}{}  optimize <agent> - Optimize agent memory", 
        Symbols::VERTICAL, Symbols::T_RIGHT, Symbols::HORIZONTAL);
    println!("{}  {}{}  cache <agent>    - Cache agent memory", 
        Symbols::VERTICAL, Symbols::BOTTOM_LEFT, Symbols::HORIZONTAL);
    
    // Knowledge commands branch
    println!("{}{} {}", 
        Symbols::T_RIGHT, 
        Symbols::HORIZONTAL,
        format!("{} {}", Symbols::KNOWLEDGE, "Knowledge").color(knowledge_color).bold());
    println!("{}  {}{}  search <query>   - Search knowledge registry", 
        Symbols::VERTICAL, Symbols::T_RIGHT, Symbols::HORIZONTAL);
    println!("{}  {}{}  index            - Update knowledge registry indexes", 
        Symbols::VERTICAL, Symbols::T_RIGHT, Symbols::HORIZONTAL);
    println!("{}  {}{}  distribute       - Distribute knowledge to agents", 
        Symbols::VERTICAL, Symbols::BOTTOM_LEFT, Symbols::HORIZONTAL);
    
    // Debug commands branch
    println!("{}{} {}", 
        Symbols::T_RIGHT, 
        Symbols::HORIZONTAL,
        format!("{} {}", Symbols::DEBUG, "Debug").color(debug_color).bold());
    println!("{}  {}{}  paths            - Print repository path information", 
        Symbols::VERTICAL, Symbols::T_RIGHT, Symbols::HORIZONTAL);
    println!("{}  {}{}  config           - Check system configuration", 
        Symbols::VERTICAL, Symbols::T_RIGHT, Symbols::HORIZONTAL);
    println!("{}  {}{}  format           - Test terminal formatting", 
        Symbols::VERTICAL, Symbols::T_RIGHT, Symbols::HORIZONTAL);
    println!("{}  {}{}  map              - Display this command map", 
        Symbols::VERTICAL, Symbols::BOTTOM_LEFT, Symbols::HORIZONTAL);
    
    // System commands branch
    println!("{}{} {}", 
        Symbols::BOTTOM_LEFT, 
        Symbols::HORIZONTAL,
        format!("{} {}", Symbols::SYSTEM, "System").color(system_color).bold());
    println!("   {}{}  clean [--cache]   - Clean temporary files", 
        Symbols::T_RIGHT, Symbols::HORIZONTAL);
    println!("   {}{}  verify            - Verify repository structure", 
        Symbols::T_RIGHT, Symbols::HORIZONTAL);
    println!("   {}{}  title [title]     - Update terminal title", 
        Symbols::BOTTOM_LEFT, Symbols::HORIZONTAL);
    
    println!();
    TerminalHelpers::print_section("Command Categories", ThemeColors::PRIMARY);
    
    // Create a more detailed description of each command category
    let categories = [
        (Symbols::AGENT, "Agent", agent_color, "Commands for creating, listing, and managing agents in the repository"),
        (Symbols::MEMORY, "Memory", memory_color, "Memory system commands for loading, optimizing, and caching agent memories"),
        (Symbols::KNOWLEDGE, "Knowledge", knowledge_color, "Knowledge management commands for searching, indexing, and distributing knowledge"),
        (Symbols::DEBUG, "Debug", debug_color, "Debug utilities for development, testing, and diagnostics"),
        (Symbols::SYSTEM, "System", system_color, "System maintenance commands for repository structure and configuration")
    ];
    
    // Print categories in a formatted table
    for (symbol, name, color, description) in categories.iter() {
        println!("{}  {} {}{}", 
            symbol,
            format!("{}", name).color(*color).bold(),
            " ".repeat(10 - name.len()),
            description);
    }
    
    TerminalHelpers::print_success("Command map displayed");
    Ok(())
}

/// Test terminal formatting
pub fn format(_config: &Config) -> Result<()> {
    TerminalHelpers::print_command_header(
        "Terminal Formatting Test",
        "debug"
    );
    
    // Test our new visual components
    
    // Section 1: Basic formatting
    TerminalHelpers::print_section("Basic Text Formatting", ThemeColors::PRIMARY);
    
    println!("{}", "Standard text".normal());
    println!("{}", "Bold text".bold());
    println!("{}", "Italic text".italic());
    println!("{}", "Underlined text".underline());
    
    // Test colors
    TerminalHelpers::print_section("Color Test", ThemeColors::ACCENT);
    println!("{}", "Red text".red());
    println!("{}", "Green text".green());
    println!("{}", "Yellow text".yellow());
    println!("{}", "Blue text".blue());
    println!("{}", "Magenta text".magenta());
    println!("{}", "Cyan text".cyan());
    println!("{}", "White text".white());
    
    // Test background colors
    TerminalHelpers::print_section("Background Color Test", ThemeColors::SECONDARY);
    println!("{}", "Red background".on_red());
    println!("{}", "Green background".on_green());
    println!("{}", "Yellow background".on_yellow());
    println!("{}", "Blue background".on_blue());
    println!("{}", "Magenta background".on_magenta());
    println!("{}", "Cyan background".on_cyan());
    println!("{}", "White background".on_white());
    
    // Status messages
    TerminalHelpers::print_section("Status Messages", ThemeColors::INFO);
    TerminalHelpers::print_success("This is a success message");
    TerminalHelpers::print_error("This is an error message");
    TerminalHelpers::print_warning("This is a warning message");
    TerminalHelpers::print_info("This is an info message");
    TerminalHelpers::print_status("This is a status message");
    
    // Headers for different command types
    TerminalHelpers::print_section("Command Headers", ThemeColors::INFO);
    
    // We'll use a loop and then sleep to showcase each header type
    let categories = vec!["agent", "system", "memory", "knowledge", "debug"];
    for category in categories {
        TerminalHelpers::print_command_header(
            &format!("Example {} Command", category.to_uppercase()),
            category
        );
        thread::sleep(Duration::from_millis(500));
    }
    
    // Test dividers
    TerminalHelpers::print_section("Dividers", ThemeColors::INFO);
    TerminalHelpers::print_divider(ThemeColors::PRIMARY);
    TerminalHelpers::print_divider(ThemeColors::SECONDARY);
    TerminalHelpers::print_divider(ThemeColors::ACCENT);
    TerminalHelpers::print_divider(ThemeColors::SUCCESS);
    TerminalHelpers::print_divider(ThemeColors::WARNING);
    
    // Test step indicator
    TerminalHelpers::print_section("Step Indicators", ThemeColors::INFO);
    for i in 1..=3 {
        TerminalHelpers::print_step(i, 3, &format!("Step {} of the process", i));
        thread::sleep(Duration::from_millis(500));
    }
    
    // Test spinner
    TerminalHelpers::print_section("Progress Spinner", ThemeColors::INFO);
    println!("Running a spinner for 2 seconds:");
    TerminalHelpers::show_spinner("Processing data...", 2000);
    TerminalHelpers::print_success("Operation completed!");
    
    // Test tables
    TerminalHelpers::print_section("Tables", ThemeColors::INFO);
    let headers = &["Agent", "Status", "Last Updated"];
    let rows = &[
        vec!["Architect".to_string(), "Active".to_string(), "2025-05-20 10:30".to_string()],
        vec!["Developer".to_string(), "Inactive".to_string(), "2025-05-19 15:45".to_string()],
        vec!["Analyst".to_string(), "Active".to_string(), "2025-05-20 09:15".to_string()],
    ];
    TerminalHelpers::print_table(headers, rows);
    
    // Test lists
    TerminalHelpers::print_section("Lists", ThemeColors::INFO);
    let items = vec![
        "First item in the list".to_string(),
        "Second item with some additional text".to_string(),
        "Third item in the list".to_string(),
    ];
    TerminalHelpers::print_list(&items, ThemeColors::PRIMARY);
    
    // Test result blocks
    TerminalHelpers::print_section("Result Blocks", ThemeColors::INFO);
    let result_content = "This is the content of a result block.\nIt can span multiple lines.\nAnd it's formatted nicely.";
    TerminalHelpers::print_result_block("Command Result", result_content, ThemeColors::SUCCESS);
    
    // Test user input prompts
    TerminalHelpers::print_section("Interactive Elements", ThemeColors::INFO);
    println!("Example of confirmation prompt (will not actually prompt):");
    println!("{} {} (y/n): ", Symbols::ARROW.blue().bold(), "Do you want to continue?");
    println!("\nExample of input prompt (will not actually prompt):");
    println!("{} {}: ", Symbols::ARROW.blue().bold(), "Enter your name");
    println!("\nExample of selection menu (will not actually prompt):");
    println!("{} {}", Symbols::ARROW.blue().bold(), "Select an option");
    println!();
    println!("  {}. {}", "1".blue().bold(), "First option");
    println!("  {}. {}", "2".blue().bold(), "Second option");
    println!("  {}. {}", "3".blue().bold(), "Third option");
    
    TerminalHelpers::print_success("Formatting test completed");
    Ok(())
}