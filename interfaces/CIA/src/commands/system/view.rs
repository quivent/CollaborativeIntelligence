use crate::helpers::terminal::{TerminalHelpers, ThemeColors};
use crate::config::Config;
use colored::*;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FileCategory {
    pub name: &'static str,
    pub description: &'static str,
    pub color: &'static str,
    pub icon: &'static str,
}

impl FileCategory {
    pub const CORE_CONFIG: FileCategory = FileCategory {
        name: "Core Configuration",
        description: "Essential project configuration files",
        color: "blue",
        icon: "⚙️",
    };
    
    pub const DOCUMENTATION: FileCategory = FileCategory {
        name: "Documentation",
        description: "Project documentation and specifications",
        color: "green",
        icon: "📚",
    };
    
    pub const AGENTS: FileCategory = FileCategory {
        name: "Agent System",
        description: "Agent directories and related files",
        color: "cyan",
        icon: "🤖",
    };
    
    pub const ANALYSIS: FileCategory = FileCategory {
        name: "Analysis & Reports",
        description: "Analysis files, benchmarks, and reports",
        color: "yellow",
        icon: "📊",
    };
    
    pub const CLI_TOOLS: FileCategory = FileCategory {
        name: "CLI & Tools",
        description: "Command-line interfaces and development tools",
        color: "magenta",
        icon: "🔧",
    };
    
    pub const SCRIPTS: FileCategory = FileCategory {
        name: "Scripts & Automation",
        description: "Automation scripts and utilities",
        color: "white",
        icon: "📜",
    };
    
    pub const RUST_CODE: FileCategory = FileCategory {
        name: "Rust Applications",
        description: "Rust source code and compiled artifacts",
        color: "red",
        icon: "🦀",
    };
    
    pub const CACHE_DATA: FileCategory = FileCategory {
        name: "Cache & Data",
        description: "Cached data, temporary files, and storage",
        color: "bright_black",
        icon: "💾",
    };
    
    pub const LEGACY: FileCategory = FileCategory {
        name: "Legacy & Migration",
        description: "Legacy files and migration artifacts",
        color: "bright_yellow",
        icon: "🗂️",
    };
    
    pub const SYSTEM_FILES: FileCategory = FileCategory {
        name: "System Files",
        description: "System configuration and operational files",
        color: "bright_blue",
        icon: "⚡",
    };
}

/// Categorize a file based on its name and characteristics
fn categorize_file(file_name: &str) -> &'static FileCategory {
    // Convert to lowercase for easier matching
    let name_lower = file_name.to_lowercase();
    
    // Core configuration files
    if matches!(name_lower.as_str(), 
        "claude.md" | "claude.local.md" | "claude.local.md.template" | 
        "readme.md" | "makefile" | "package.json" | ".gitignore"
    ) {
        return &FileCategory::CORE_CONFIG;
    }
    
    // Documentation
    if name_lower.ends_with(".md") && (
        name_lower.contains("spec") || 
        name_lower.contains("doc") ||
        name_lower.contains("guide") ||
        name_lower.contains("plan") ||
        name_lower.contains("vision") ||
        file_name.starts_with("SOFIA_") ||
        file_name.starts_with("TAOAI_")
    ) {
        return &FileCategory::DOCUMENTATION;
    }
    
    // Agent system
    if matches!(name_lower.as_str(), "agents" | "agent_index.md" | "agent_backlog.md" | "claude_agents.md") ||
       name_lower.contains("agent") {
        return &FileCategory::AGENTS;
    }
    
    // Analysis and reports
    if matches!(name_lower.as_str(), 
        "analyses" | "benchmarks" | "reports" | "architectures"
    ) || name_lower.contains("report") || 
       name_lower.contains("analysis") || 
       name_lower.contains("benchmark") ||
       name_lower.contains("metric") {
        return &FileCategory::ANALYSIS;
    }
    
    // CLI and tools
    if matches!(name_lower.as_str(), 
        "clis" | "cli" | "tools" | "ci-tools"
    ) || name_lower.starts_with("ci-") ||
       name_lower.contains("cli") {
        return &FileCategory::CLI_TOOLS;
    }
    
    // Scripts and automation
    if name_lower.ends_with(".sh") || 
       name_lower.ends_with(".js") ||
       matches!(name_lower.as_str(), "scripts" | "claude-init" | "terminal-launcher.sh") ||
       name_lower.contains("script") {
        return &FileCategory::SCRIPTS;
    }
    
    // Rust code
    if name_lower.ends_with(".rs") ||
       name_lower.contains("cargo") ||
       matches!(name_lower.as_str(), "api" | "agent_cache") {
        return &FileCategory::RUST_CODE;
    }
    
    // Cache and data
    if matches!(name_lower.as_str(), 
        "cache" | "data" | "kanji" | "test_logs" | "test-output.txt"
    ) || name_lower.contains("cache") ||
       name_lower.contains("temp") {
        return &FileCategory::CACHE_DATA;
    }
    
    // Legacy and migration
    if name_lower.contains("backup") ||
       name_lower.contains("migration") ||
       name_lower.contains("legacy") ||
       name_lower.contains(".original") ||
       name_lower.starts_with("backup_") ||
       matches!(name_lower.as_str(), "ci.original" | "remove_cir_references_dry_run.sh") {
        return &FileCategory::LEGACY;
    }
    
    // System files
    if name_lower.starts_with("notification_") ||
       name_lower.starts_with("protocol_") ||
       name_lower.starts_with("system_") ||
       name_lower.contains("critical") ||
       matches!(name_lower.as_str(), 
        "sessions" | "protocols" | "optimizations" | "priority" | 
        "implementations" | "implemented-solutions" | "examples" | "extensions"
    ) {
        return &FileCategory::SYSTEM_FILES;
    }
    
    // Default to documentation for unknown .md files, system files for others
    if name_lower.ends_with(".md") {
        &FileCategory::DOCUMENTATION
    } else {
        &FileCategory::SYSTEM_FILES
    }
}

/// Display repository files categorized by type
pub fn view(config: &Config) -> Result<()> {
    TerminalHelpers::print_fancy_header(
        "Repository Structure View",
        "🗂️",
        "System Analysis",
        ThemeColors::SYSTEM
    );
    
    TerminalHelpers::print_info("Analyzing top-level repository structure for refactoring insights...");
    println!();
    
    // Read the top-level directory
    let entries = std::fs::read_dir(&config.repo_path)?;
    
    // Categorize all files and directories
    let mut categories: HashMap<String, Vec<String>> = HashMap::new();
    let mut file_count = 0;
    
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        
        // Skip hidden files and common uninteresting files
        if file_name.starts_with('.') && file_name != ".gitignore" {
            continue;
        }
        
        let category = categorize_file(&file_name);
        categories.entry(category.name.to_string())
            .or_insert_with(Vec::new)
            .push(file_name);
        
        file_count += 1;
    }
    
    // Sort categories by importance for refactoring insights
    let category_order = [
        FileCategory::CORE_CONFIG.name,
        FileCategory::AGENTS.name,
        FileCategory::CLI_TOOLS.name,
        FileCategory::RUST_CODE.name,
        FileCategory::SCRIPTS.name,
        FileCategory::ANALYSIS.name,
        FileCategory::DOCUMENTATION.name,
        FileCategory::CACHE_DATA.name,
        FileCategory::SYSTEM_FILES.name,
        FileCategory::LEGACY.name,
    ];
    
    let all_categories = [
        &FileCategory::CORE_CONFIG,
        &FileCategory::AGENTS,
        &FileCategory::CLI_TOOLS,
        &FileCategory::RUST_CODE,
        &FileCategory::SCRIPTS,
        &FileCategory::ANALYSIS,
        &FileCategory::DOCUMENTATION,
        &FileCategory::CACHE_DATA,
        &FileCategory::SYSTEM_FILES,
        &FileCategory::LEGACY,
    ];
    
    // Display each category
    for category_name in &category_order {
        if let Some(files) = categories.get(*category_name) {
            let category = all_categories.iter()
                .find(|c| c.name == *category_name)
                .unwrap();
            
            // Print category header
            let color = match category.color {
                "blue" => "blue",
                "green" => "green",
                "cyan" => "cyan",
                "yellow" => "yellow",
                "magenta" => "magenta",
                "white" => "white",
                "red" => "red",
                "bright_black" => "bright_black",
                "bright_yellow" => "bright_yellow",
                "bright_blue" => "bright_blue",
                _ => "white",
            };
            
            println!("{} {} {} ({})", 
                category.icon,
                category.name.color(color).bold(),
                format!("({} items)", files.len()).color("bright_black"),
                category.description.italic()
            );
            
            // Sort and display files
            let mut sorted_files = files.clone();
            sorted_files.sort();
            
            for (i, file) in sorted_files.iter().enumerate() {
                let prefix = if i == sorted_files.len() - 1 { "└─" } else { "├─" };
                println!("  {} {}", prefix.color("bright_black"), file);
            }
            println!();
        }
    }
    
    // Summary
    TerminalHelpers::print_success(&format!(
        "Repository analysis complete: {} items categorized across {} categories", 
        file_count, 
        categories.len()
    ));
    
    // Refactoring insights
    println!();
    TerminalHelpers::print_info("🔍 Refactoring Insights:");
    
    if categories.get(FileCategory::LEGACY.name).map_or(0, |v| v.len()) > 0 {
        println!("  • {} Legacy files detected - consider cleanup or archival", 
            "⚠️".yellow());
    }
    
    if categories.get(FileCategory::SCRIPTS.name).map_or(0, |v| v.len()) > 5 {
        println!("  • {} Multiple scripts detected - consider consolidation into scripts/ directory", 
            "📝".cyan());
    }
    
    if categories.get(FileCategory::RUST_CODE.name).map_or(0, |v| v.len()) > 3 {
        println!("  • {} Multiple Rust projects - consider workspace organization", 
            "🦀".red());
    }
    
    if categories.get(FileCategory::CLI_TOOLS.name).map_or(0, |v| v.len()) > 0 {
        println!("  • {} CLI tools available - ensure consistent interface design", 
            "🔧".magenta());
    }
    
    println!("  • Use 'cia system clean' to remove temporary files");
    println!("  • Use 'cia system verify' to check structural integrity");
    
    Ok(())
}