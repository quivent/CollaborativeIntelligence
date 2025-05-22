use crate::helpers::CommandHelpers;
use crate::helpers::path::PathHelpers;
use crate::config::Config;
use crate::agent_sync::AgentSyncTool;
use colored::*;
use std::fs;
use anyhow::Result;

// Import enhanced agent creation modules
use crate::agent_creation::{create_agent_interactive, validate_agent};
// Import shared protocol instructions
use ci_shared::display_agent_protocol_instructions;

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
    
    // Display protocol compliance instructions
    display_agent_protocol_instructions(name);
    
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

/// Recommend the best agent for a specific task or query
pub fn recommend_for(query: &str, config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        &format!("Finding Agent for: \"{}\"", query),
        "🎯",
        "Agent Recommendation",
        "magenta"
    );
    
    // Format the query for the Recommender agent
    let recommender_prompt = format!(
        "Which agent would best handle the request: \"{}\"? \
        Please provide a specific agent recommendation with a brief explanation of why that agent is the best fit.",
        query
    );
    
    // Get recommendation using built-in logic (simulates Recommender agent)
    let recommendation = get_agent_recommendation(&recommender_prompt, config)?;
    
    // Format and display the recommendation
    CommandHelpers::print_info("🤖 Recommendation:");
    println!("\n{}", recommendation);
    
    // Extract agent name for quick activation hint
    if let Some(agent_name) = extract_agent_name(&recommendation) {
        println!("\n💡 Quick activation: {}", format!("cia agent activate {}", agent_name).cyan());
    }
    
    Ok(())
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

/// Extract agent name from recommendation text (helper function)
fn extract_agent_name(recommendation: &str) -> Option<&str> {
    // Simple pattern matching to extract agent names
    let agents = [
        "Athena", "Analyst", "Architect", "Developer", "Engineer", "Designer", 
        "Debugger", "Optimizer", "Fixer", "Solver", "Manager", "Documenter",
        "Tester", "Researcher", "Database", "UI", "UX", "Recommender",
        "Cartographer", "Inspector", "Linguist", "Benchmarker", "Streamliner",
        "Hermes", "Human", "Expert", "Marketer", "Automator", "Refactorer",
        "SolutionArchitect", "Consolidator", "Enforcer", "Planner", "Cacher",
        "Visionary", "Topologist", "Scholar", "Gaia", "Overviewer", "Pause",
        "Repairer", "Rustist", "Bypasser", "Basher", "Standardist", "CLIA",
        "CompilerAgent", "Resourcer", "Visualist", "Comparitor", "Applicationer"
    ];
    
    for agent in &agents {
        if recommendation.contains(agent) {
            return Some(agent);
        }
    }
    None
}

/// Get agent recommendation (built-in recommendation engine)
fn get_agent_recommendation(prompt: &str, _config: &Config) -> Result<String> {
    // This simulates the Recommender agent's logic
    let prompt_lower = prompt.to_lowercase();
    
    // Prioritize more specific patterns first to avoid incorrect matches
    let recommendation = if prompt_lower.contains("debug") || prompt_lower.contains("error") || prompt_lower.contains("fix") || prompt_lower.contains("broken") {
        "**Debugger** - Specialized in identifying and resolving technical errors and code-level issues."
    } else if prompt_lower.contains("analy") || prompt_lower.contains("pattern") || prompt_lower.contains("insight") {
        "**Analyst** - Best suited for comprehensive analysis, pattern recognition, and generating deep insights from complex data or situations."
    } else if prompt_lower.contains("design") || prompt_lower.contains("interface") || prompt_lower.contains("ui") {
        if prompt_lower.contains("user") || prompt_lower.contains("experience") {
            "**UX** - Perfect for user experience research, journey mapping, and holistic experience design."
        } else {
            "**Designer** - Excellent for visual design, interface creation, and aesthetic problem-solving."
        }
    } else if prompt_lower.contains("code") || prompt_lower.contains("implement") || prompt_lower.contains("develop") {
        "**Engineer** - Ideal for implementing functionality, solving technical problems, and building robust solutions."
    } else if prompt_lower.contains("architect") || prompt_lower.contains("structure") || prompt_lower.contains("system") {
        "**Architect** - Best for system design, architectural planning, and structural problem-solving."
    } else if prompt_lower.contains("optim") || prompt_lower.contains("perform") || prompt_lower.contains("speed") {
        "**Optimizer** - Specialized in performance improvements, code optimization, and efficiency enhancements."
    } else if prompt_lower.contains("test") || prompt_lower.contains("quality") || prompt_lower.contains("validation") {
        "**Tester** - Perfect for quality assurance, test creation, and validation strategies."
    } else if prompt_lower.contains("document") || prompt_lower.contains("explain") || prompt_lower.contains("guide") {
        "**Documenter** - Ideal for creating clear documentation, guides, and explanatory content."
    } else if prompt_lower.contains("research") || prompt_lower.contains("information") || prompt_lower.contains("find") {
        "**Researcher** - Excellent for gathering information, conducting research, and synthesizing knowledge."
    } else if prompt_lower.contains("manage") || prompt_lower.contains("organiz") || prompt_lower.contains("coordinate") {
        "**Manager** - Best for organizational tasks, process management, and coordination activities."
    } else if prompt_lower.contains("memory") || prompt_lower.contains("knowledge") || prompt_lower.contains("learn") {
        "**Athena** - Specialized in knowledge organization, memory architecture, and learning systems."
    } else if prompt_lower.contains("rust") || prompt_lower.contains("ownership") || prompt_lower.contains("borrow") {
        "**Rustist** - Expert in Rust programming language, memory safety, and performance optimization."
    } else if prompt_lower.contains("shell") || prompt_lower.contains("bash") || prompt_lower.contains("script") {
        "**Basher** - Specialized in shell scripting, command line automation, and system administration."
    } else if prompt_lower.contains("cli") || prompt_lower.contains("command") || prompt_lower.contains("terminal") {
        "**CLIA** - Expert in command-line interface design, implementation, and user experience."
    } else {
        "**Recommender** - For complex or multi-faceted requests, the Recommender agent can provide detailed analysis and agent selection guidance."
    };
    
    Ok(recommendation.to_string())
}