use crate::helpers::CommandHelpers;
use crate::helpers::path::PathHelpers;
use crate::config::Config;
use crate::agent_sync::AgentSyncTool;
use colored::*;
use std::fs;
use anyhow::Result;

// Import enhanced agent creation modules
use crate::agent_creation::{create_agent_interactive, validate_agent};

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

/// Create a new agent using the enhanced template system
pub fn create(
    name: &str,
    role: &Option<String>,
    purpose: &Option<String>,
    quick: bool,
    config: &Config,
) -> Result<()> {
    create_agent_interactive(name, role, purpose, quick, config)
}

/// Validate an existing agent's structure
pub fn validate(name: &str, config: &Config) -> Result<()> {
    validate_agent(name, config)
}

/// Synchronize agents-overview.md and agent-index.md
pub fn sync(config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        "Synchronizing Agent Index",
        "🔄",
        "Agent Management",
        "cyan"
    );
    
    let repo_path = config.repo_path.to_string_lossy().to_string();
    let tool = AgentSyncTool::with_base_dir(repo_path);
    
    match tool.sync_indices() {
        Ok(_) => {
            CommandHelpers::print_success("Agent indices synchronized successfully");
            Ok(())
        }
        Err(e) => {
            CommandHelpers::print_error(&format!("Failed to sync agent indices: {}", e));
            Err(e)
        }
    }
}

/// Add a new agent to the agent registry
pub fn add_to_registry(name: &str, role: &str, description: &str, config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        &format!("Adding Agent to Registry: {}", name),
        "📝",
        "Agent Management",
        "cyan"
    );
    
    let repo_path = config.repo_path.to_string_lossy().to_string();
    let tool = AgentSyncTool::with_base_dir(repo_path);
    
    match tool.add_agent(name, role, description) {
        Ok(_) => {
            CommandHelpers::print_success(&format!("Agent '{}' added to registry successfully", name));
            Ok(())
        }
        Err(e) => {
            CommandHelpers::print_error(&format!("Failed to add agent to registry: {}", e));
            Err(e)
        }
    }
}

/// Check agent synchronization status
pub fn check_sync(config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        "Checking Agent Synchronization",
        "🔍",
        "Agent Management",
        "cyan"
    );
    
    let repo_path = config.repo_path.to_string_lossy().to_string();
    let tool = AgentSyncTool::with_base_dir(repo_path);
    
    let agents = tool.parse_agents_file()?;
    let index_agents = tool.parse_agent_index()?;
    
    CommandHelpers::print_info(&format!("Found {} agents in agents-overview.md", agents.len()));
    CommandHelpers::print_info(&format!("Found {} agents in agent-index.md", index_agents.len()));
    
    let discrepancies = tool.check_discrepancies(&agents, &index_agents);
    
    if discrepancies.is_empty() {
        CommandHelpers::print_success("No discrepancies found - files are synchronized");
    } else {
        CommandHelpers::print_warning(&format!("Found {} discrepancies:", discrepancies.len()));
        for discrepancy in discrepancies {
            println!("  • {}", discrepancy.yellow());
        }
    }
    
    Ok(())
}