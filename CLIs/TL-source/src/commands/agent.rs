use crate::helpers::CommandHelpers;
use crate::helpers::path::PathHelpers;
use crate::config::Config;
use colored::*;
use std::fs;
use anyhow::Result;

/// Activate an agent by name
pub fn activate(name: &str, config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        &format!("Activating Agent: {}", name),
        "🧠",
        "Agent Management",
        "cyan"
    );
    
    // Check if agent exists
    let agents_path = PathHelpers::get_agents_path(&config.repo_path);
    let agent_path = agents_path.join(name);
    
    if !agent_path.exists() {
        CommandHelpers::print_error(&format!("Agent '{}' does not exist", name));
        return Err(anyhow::anyhow!("Agent does not exist: {}", name));
    }
    
    // Update terminal title
    CommandHelpers::set_terminal_title(&format!("Agent: {}", name));
    
    // Simulate agent activation
    CommandHelpers::print_info(&format!("Loading configuration for agent '{}'", name));
    CommandHelpers::print_info(&format!("Loading memory for agent '{}'", name));
    
    // Check for key agent files
    let readme_path = agent_path.join("README.md");
    let memory_path = agent_path.join("MEMORY.md");
    let learning_path = agent_path.join("ContinuousLearning.md");
    
    if readme_path.exists() {
        CommandHelpers::print_info(&format!("README file found: {}", readme_path.display()));
    } else {
        CommandHelpers::print_warning(&format!("README file missing: {}", readme_path.display()));
    }
    
    if memory_path.exists() {
        CommandHelpers::print_info(&format!("Memory file found: {}", memory_path.display()));
    } else {
        CommandHelpers::print_warning(&format!("Memory file missing: {}", memory_path.display()));
    }
    
    if learning_path.exists() {
        CommandHelpers::print_info(&format!("Learning file found: {}", learning_path.display()));
    } else {
        CommandHelpers::print_warning(&format!("Learning file missing: {}", learning_path.display()));
    }
    
    CommandHelpers::print_success(&format!("Agent '{}' activated successfully", name));
    Ok(())
}

/// List all available agents
pub fn list(config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        "Available Agents",
        "📋",
        "Agent Management",
        "cyan"
    );
    
    // Get the agents directory
    let agents_path = PathHelpers::get_agents_path(&config.repo_path);
    
    match fs::read_dir(agents_path) {
        Ok(entries) => {
            let mut found = false;
            let mut agents = Vec::new();
            
            for entry in entries.filter_map(Result::ok) {
                if entry.path().is_dir() {
                    if let Some(name) = entry.path().file_name() {
                        if let Some(name_str) = name.to_str() {
                            agents.push(name_str.to_string());
                            found = true;
                        }
                    }
                }
            }
            
            if found {
                // Sort agents alphabetically
                agents.sort();
                
                // Display sorted agents
                for agent in agents {
                    println!("- {}", agent.blue());
                }
            } else {
                CommandHelpers::print_warning("No agents found");
            }
        },
        Err(e) => {
            CommandHelpers::print_error(&format!("Failed to read agents directory: {}", e));
            return Err(anyhow::anyhow!("Failed to read agents directory: {}", e));
        }
    }
    
    Ok(())
}

/// Create a new agent
pub fn create(name: &str, category: &Option<String>, config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        &format!("Creating New Agent: {}", name),
        "➕",
        "Agent Management",
        "cyan"
    );
    
    // Check if agent already exists
    let agents_path = PathHelpers::get_agents_path(&config.repo_path);
    let agent_path = agents_path.join(name);
    
    if agent_path.exists() {
        CommandHelpers::print_error(&format!("Agent '{}' already exists", name));
        return Err(anyhow::anyhow!("Agent already exists: {}", name));
    }
    
    // Determine agent category
    let category = match category {
        Some(cat) => cat.clone(),
        None => {
            // Default to "General" category
            "General".to_string()
        }
    };
    
    // Create agent directory
    match fs::create_dir_all(&agent_path) {
        Ok(_) => {
            CommandHelpers::print_info(&format!("Created agent directory: {}", agent_path.display()));
            
            // Create default agent files
            let readme_path = agent_path.join("README.md");
            let readme_content = format!("# {} Agent\n\n## Purpose\n\nThis agent is responsible for...\n\n## Capabilities\n\n- \n", name);
            
            match fs::write(&readme_path, readme_content) {
                Ok(_) => CommandHelpers::print_info(&format!("Created README.md for {}", name)),
                Err(e) => CommandHelpers::print_error(&format!("Failed to create README.md: {}", e)),
            }
            
            // Create memory file
            let memory_path = agent_path.join("MEMORY.md");
            let memory_content = format!("# {} Agent Memory\n\n## Core Knowledge\n\n- \n", name);
            
            match fs::write(&memory_path, memory_content) {
                Ok(_) => CommandHelpers::print_info(&format!("Created MEMORY.md for {}", name)),
                Err(e) => CommandHelpers::print_error(&format!("Failed to create MEMORY.md: {}", e)),
            }
            
            // Create continuous learning file
            let learning_path = agent_path.join("ContinuousLearning.md");
            let learning_content = format!("# {} Agent Learning\n\n## Recent Developments\n\n- \n", name);
            
            match fs::write(&learning_path, learning_content) {
                Ok(_) => CommandHelpers::print_info(&format!("Created ContinuousLearning.md for {}", name)),
                Err(e) => CommandHelpers::print_error(&format!("Failed to create ContinuousLearning.md: {}", e)),
            }
            
            // Create Sessions directory
            let sessions_path = agent_path.join("Sessions");
            match fs::create_dir_all(&sessions_path) {
                Ok(_) => CommandHelpers::print_info(&format!("Created Sessions directory for {}", name)),
                Err(e) => CommandHelpers::print_error(&format!("Failed to create Sessions directory: {}", e)),
            }
            
            // Success message
            CommandHelpers::print_success(&format!("Agent '{}' created successfully in category '{}'", name, category));
            Ok(())
        },
        Err(e) => {
            CommandHelpers::print_error(&format!("Failed to create agent directory: {}", e));
            Err(anyhow::anyhow!("Failed to create agent directory: {}", e))
        }
    }
}