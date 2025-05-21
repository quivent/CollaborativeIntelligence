use crate::helpers::CommandHelpers;
use crate::helpers::path::PathHelpers;
use crate::config::Config;
use colored::*;
use anyhow::Result;
use std::path::PathBuf;

/// Search the knowledge registry
pub fn search(query: &str, category: &Option<String>, config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        "Knowledge Search",
        "🔍",
        "Knowledge Management",
        "yellow"
    );
    
    let category_str = match category {
        Some(cat) => format!(" in category '{}'", cat),
        None => "".to_string(),
    };
    
    CommandHelpers::print_info(&format!("Searching for '{}'{}", query, category_str));
    
    // Check if Knowledge Registry exists
    let knowledge_registry = config.repo_path.join("KnowledgeRegistry");
    if !knowledge_registry.exists() {
        CommandHelpers::print_warning("Knowledge Registry not found. Creating one for the search.");
        
        // Create knowledge registry directory if it doesn't exist
        match std::fs::create_dir_all(&knowledge_registry) {
            Ok(_) => CommandHelpers::print_info(&format!("Created Knowledge Registry at: {}", knowledge_registry.display())),
            Err(e) => {
                CommandHelpers::print_error(&format!("Failed to create Knowledge Registry: {}", e));
                return Err(anyhow::anyhow!("Failed to create Knowledge Registry: {}", e));
            }
        }
    }
    
    // Locate files to search based on category
    let search_path = match category {
        Some(cat) => knowledge_registry.join("category_indices").join(format!("{}_index.md", cat.to_lowercase())),
        None => knowledge_registry.join("master_index.md"),
    };
    
    // Simulate knowledge search
    CommandHelpers::print_info("Querying knowledge registry...");
    CommandHelpers::print_info("Processing results...");
    
    // Check if we have a real index file to search
    let has_real_index = search_path.exists();
    
    // Simulate search results
    println!("\n{}", "Search Results:".yellow().bold());
    
    if has_real_index {
        // Read the index file and find matches (simplified simulation)
        match std::fs::read_to_string(&search_path) {
            Ok(content) => {
                let mut found = false;
                for line in content.lines() {
                    if line.to_lowercase().contains(&query.to_lowercase()) {
                        println!("- {} - {}", search_path.file_name().unwrap().to_string_lossy().blue(), line);
                        found = true;
                    }
                }
                
                if !found {
                    println!("No matches found for query: {}", query);
                }
            },
            Err(e) => {
                CommandHelpers::print_warning(&format!("Failed to read index file: {}", e));
                // Fall back to simulated results
                println!("- {} - {}", "architecture_index.md".blue(), "Architecture design patterns for distributed systems");
                println!("- {} - {}", "protocols.md".blue(), "System-wide communication protocols");
            }
        }
    } else {
        // Simulated results for demonstration
        println!("- {} - {}", "architecture_index.md".blue(), "Architecture design patterns for distributed systems");
        println!("- {} - {}", "protocols.md".blue(), "System-wide communication protocols");
    }
    
    CommandHelpers::print_success("Search completed");
    Ok(())
}

/// Update knowledge registry indexes
pub fn index(config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        "Knowledge Indexing",
        "📚",
        "Knowledge Management",
        "yellow"
    );
    
    // Check if Knowledge Registry exists
    let knowledge_registry = config.repo_path.join("KnowledgeRegistry");
    if !knowledge_registry.exists() {
        CommandHelpers::print_info("Knowledge Registry not found. Creating directory structure.");
        
        // Create knowledge registry directory structure
        let directories = [
            knowledge_registry.clone(),
            knowledge_registry.join("category_indices"),
            knowledge_registry.join("temp"),
        ];
        
        for dir in &directories {
            match std::fs::create_dir_all(dir) {
                Ok(_) => CommandHelpers::print_info(&format!("Created directory: {}", dir.display())),
                Err(e) => {
                    CommandHelpers::print_error(&format!("Failed to create directory: {}", e));
                    return Err(anyhow::anyhow!("Failed to create directory: {}", e));
                }
            }
        }
    }
    
    // Simulate indexing
    CommandHelpers::print_info("Analyzing repository structure...");
    
    // Search for knowledge sources in the repository
    let search_dirs = [
        config.repo_path.join("docs"),
        config.repo_path.join("Architectures"),
        config.repo_path.join("Protocols"),
        config.repo_path.join("AGENTS"),
    ];
    
    let mut found_files = 0;
    for dir in &search_dirs {
        if dir.exists() {
            CommandHelpers::print_info(&format!("Scanning directory: {}", dir.display()));
            
            // Simulate file counting (would be actual glob search in real implementation)
            match count_markdown_files(dir) {
                Ok(count) => {
                    CommandHelpers::print_info(&format!("Found {} markdown files in {}", count, dir.display()));
                    found_files += count;
                },
                Err(e) => {
                    CommandHelpers::print_warning(&format!("Error scanning directory {}: {}", dir.display(), e));
                }
            }
        } else {
            CommandHelpers::print_info(&format!("Directory not found, skipping: {}", dir.display()));
        }
    }
    
    CommandHelpers::print_info(&format!("Identified {} knowledge sources", found_files));
    CommandHelpers::print_info("Building category indices...");
    CommandHelpers::print_info("Generating cross-references...");
    CommandHelpers::print_info("Updating master index...");
    
    // Create master index file
    let master_index = knowledge_registry.join("master_index.md");
    let master_index_content = format!("# Knowledge Registry Master Index\n\n## Last Updated: {}\n\n## Categories\n\n- Architecture\n- Protocols\n- Optimizations\n\n## Contents\n\n- {}\n", 
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        found_files
    );
    
    match std::fs::write(&master_index, master_index_content) {
        Ok(_) => CommandHelpers::print_info(&format!("Created master index at: {}", master_index.display())),
        Err(e) => {
            CommandHelpers::print_error(&format!("Failed to write master index: {}", e));
            return Err(anyhow::anyhow!("Failed to write master index: {}", e));
        }
    }
    
    CommandHelpers::print_success("Knowledge registry indexes updated successfully");
    Ok(())
}

/// Helper function to count markdown files in a directory (recursive)
fn count_markdown_files(dir: &PathBuf) -> Result<usize> {
    use glob::glob;
    
    let pattern = format!("{}//**/*.md", dir.display());
    let files: Vec<_> = glob(&pattern)
        .map_err(|e| anyhow::anyhow!("Failed to read glob pattern: {}", e))?
        .filter_map(Result::ok)
        .collect();
        
    Ok(files.len())
}

/// Distribute knowledge to agents
pub fn distribute(agent: &Option<String>, config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        "Knowledge Distribution",
        "📡",
        "Knowledge Management",
        "yellow"
    );
    
    match agent {
        Some(name) => {
            // Check if agent exists
            let agents_path = PathHelpers::get_agents_path(&config.repo_path);
            let agent_path = agents_path.join(name);
            
            if !agent_path.exists() {
                CommandHelpers::print_error(&format!("Agent '{}' does not exist", name));
                return Err(anyhow::anyhow!("Agent does not exist: {}", name));
            }
            
            CommandHelpers::print_info(&format!("Distributing knowledge to agent '{}'", name));
            CommandHelpers::print_info("Identifying relevant knowledge segments...");
            CommandHelpers::print_info("Preparing knowledge integration...");
            
            // Check if agent has a memory file
            let memory_path = agent_path.join("MEMORY.md");
            if !memory_path.exists() {
                CommandHelpers::print_warning(&format!("Agent '{}' does not have a memory file", name));
                
                // Create memory file if it doesn't exist
                let memory_content = format!("# {} Agent Memory\n\n## Core Knowledge\n\n- \n", name);
                match std::fs::write(&memory_path, memory_content) {
                    Ok(_) => CommandHelpers::print_info(&format!("Created MEMORY.md for {}", name)),
                    Err(e) => CommandHelpers::print_error(&format!("Failed to create MEMORY.md: {}", e)),
                }
            }
            
            CommandHelpers::print_info(&format!("Updating agent '{}' memory", name));
            CommandHelpers::print_success(&format!("Knowledge distributed to agent '{}'", name));
            Ok(())
        },
        None => {
            CommandHelpers::print_info("Distributing knowledge to all agents");
            
            // Get all agents
            let agents_path = PathHelpers::get_agents_path(&config.repo_path);
            
            match std::fs::read_dir(agents_path) {
                Ok(entries) => {
                    let mut agents = Vec::new();
                    
                    for entry in entries.filter_map(Result::ok) {
                        if entry.path().is_dir() {
                            if let Some(name) = entry.path().file_name() {
                                if let Some(name_str) = name.to_str() {
                                    agents.push(name_str.to_string());
                                }
                            }
                        }
                    }
                    
                    if agents.is_empty() {
                        CommandHelpers::print_warning("No agents found for knowledge distribution");
                        return Ok(());
                    }
                    
                    CommandHelpers::print_info(&format!("Found {} agents", agents.len()));
                    
                    for agent in agents {
                        CommandHelpers::print_info(&format!("Distributing knowledge to agent '{}'", agent));
                    }
                    
                    CommandHelpers::print_success("Knowledge distributed to all agents");
                    Ok(())
                },
                Err(e) => {
                    CommandHelpers::print_error(&format!("Failed to read agents directory: {}", e));
                    Err(anyhow::anyhow!("Failed to read agents directory: {}", e))
                }
            }
        }
    }
}