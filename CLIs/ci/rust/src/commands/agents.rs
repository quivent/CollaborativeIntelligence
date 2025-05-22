//! Agents command implementation
//!
//! Displays available Collaborative Intelligence agents with their information.

use anyhow::{Result, Context};
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::config::Config;
use crate::helpers::terminal::{TerminalHelpers, ThemeColors};

/// Agent information structure
#[derive(Debug, Clone)]
pub struct Agent {
    pub name: String,
    pub role: String,
    pub expertise: String,
    pub focus: String,
    pub perspective: String,
    pub usage_count: u32,
}

/// Parse the AGENTS.md file and extract agent information
fn parse_agents_file(agents_file_path: &Path) -> Result<Vec<Agent>> {
    let content = fs::read_to_string(agents_file_path)
        .with_context(|| format!("Failed to read AGENTS.md from {}", agents_file_path.display()))?;
    
    let mut agents = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i].trim();
        
        // Look for agent section headers like "### AgentName - Role Description"
        if line.starts_with("### ") && line.contains(" - ") {
            if let Some(agent) = parse_agent_section(&lines, &mut i) {
                agents.push(agent);
            }
        }
        i += 1;
    }
    
    Ok(agents)
}

/// Parse a single agent section
fn parse_agent_section(lines: &[&str], index: &mut usize) -> Option<Agent> {
    let header_line = lines[*index].trim();
    
    // Parse the header: "### AgentName - Role Description"
    if let Some(dash_pos) = header_line.find(" - ") {
        let name_part = &header_line[4..dash_pos]; // Remove "### "
        let role_part = &header_line[dash_pos + 3..]; // Remove " - "
        
        let mut agent = Agent {
            name: name_part.to_string(),
            role: role_part.to_string(),
            expertise: String::new(),
            focus: String::new(),
            perspective: String::new(),
            usage_count: 0,
        };
        
        // Parse the following lines for additional information
        *index += 1;
        while *index < lines.len() {
            let line = lines[*index].trim();
            
            // Stop if we hit the next agent section or major section
            if line.starts_with("### ") || line.starts_with("## ") {
                *index -= 1; // Back up one so the outer loop can process this line
                break;
            }
            
            // Parse specific fields
            if line.starts_with("- **Expertise**:") {
                agent.expertise = line[16..].trim().to_string();
            } else if line.starts_with("- **Focus**:") {
                agent.focus = line[12..].trim().to_string();
            } else if line.starts_with("- **Perspective**:") {
                agent.perspective = line[18..].trim().to_string();
            } else if line.contains("Used ") && line.contains(" times") {
                // Extract usage count from lines like "Used 5 times"
                if let Some(used_pos) = line.find("Used ") {
                    if let Some(times_pos) = line.find(" times") {
                        let count_str = &line[used_pos + 5..times_pos];
                        if let Ok(count) = count_str.parse::<u32>() {
                            agent.usage_count = count;
                        }
                    }
                }
            }
            
            *index += 1;
        }
        
        Some(agent)
    } else {
        None
    }
}

/// Display agents in summary format (like the current behavior)
fn display_agent_summary(agents: &[Agent], filter: Option<&String>) -> Result<()> {
    TerminalHelpers::print_command_header(
        "Available Collaborative Intelligence Agents",
        "agents"
    );
    
    // Check if we should show fallback message
    if agents.is_empty() {
        println!("Agent's toolkit does not contain AGENTS.md, falling back to source repository.");
    }
    
    let filtered_agents: Vec<&Agent> = if let Some(filter_str) = filter {
        agents.iter()
            .filter(|agent| {
                agent.name.to_lowercase().contains(&filter_str.to_lowercase()) ||
                agent.role.to_lowercase().contains(&filter_str.to_lowercase()) ||
                agent.expertise.to_lowercase().contains(&filter_str.to_lowercase())
            })
            .collect()
    } else {
        agents.iter().collect()
    };
    
    println!();
    for agent in filtered_agents {
        let usage_text = if agent.usage_count > 0 {
            format!("  Used {} times", agent.usage_count).dimmed().to_string()
        } else {
            String::new()
        };
        
        println!("• {} - {}", 
            agent.name.cyan().bold(), 
            agent.role.white()
        );
        
        if !agent.expertise.is_empty() {
            println!("  {}{}", agent.expertise.dimmed(), usage_text);
        } else if !usage_text.is_empty() {
            println!("{}", usage_text);
        }
    }
    
    println!();
    println!("To use agents in your project:");
    println!("  ci init <project-name> --agents=Agent1,Agent2");
    println!("  ci integrate --agents=Agent1,Agent2");
    println!();
    println!("For more information on a specific agent:");
    println!("  ci load <agent-name>");
    println!("  ci load <agent-name> --context=<context>");
    
    Ok(())
}

/// Display detailed agent information
fn display_detailed_agents(agents: &[Agent], filter: Option<&String>) -> Result<()> {
    TerminalHelpers::print_command_header(
        "Detailed Agent Information",
        "agents"
    );
    
    let filtered_agents: Vec<&Agent> = if let Some(filter_str) = filter {
        agents.iter()
            .filter(|agent| {
                agent.name.to_lowercase().contains(&filter_str.to_lowercase()) ||
                agent.role.to_lowercase().contains(&filter_str.to_lowercase()) ||
                agent.expertise.to_lowercase().contains(&filter_str.to_lowercase())
            })
            .collect()
    } else {
        agents.iter().collect()
    };
    
    for (i, agent) in filtered_agents.iter().enumerate() {
        if i > 0 {
            println!();
        }
        
        println!("{}", format!("=== {} ===", agent.name).cyan().bold());
        println!("{}: {}", "Role".yellow().bold(), agent.role);
        
        if !agent.expertise.is_empty() {
            println!("{}: {}", "Expertise".yellow().bold(), agent.expertise);
        }
        
        if !agent.focus.is_empty() {
            println!("{}: {}", "Focus".yellow().bold(), agent.focus);
        }
        
        if !agent.perspective.is_empty() {
            println!("{}: {}", "Perspective".yellow().bold(), agent.perspective);
        }
        
        if agent.usage_count > 0 {
            println!("{}: {} times", "Usage".yellow().bold(), agent.usage_count);
        }
    }
    
    Ok(())
}

/// Display specific agent information
fn display_specific_agent(agents: &[Agent], agent_name: &str) -> Result<()> {
    let agent = agents.iter()
        .find(|a| a.name.to_lowercase() == agent_name.to_lowercase())
        .with_context(|| format!("Agent '{}' not found", agent_name))?;
    
    TerminalHelpers::print_command_header(
        &format!("Agent: {}", agent.name),
        "agents"
    );
    
    println!("{}: {}", "Role".yellow().bold(), agent.role);
    
    if !agent.expertise.is_empty() {
        println!("{}: {}", "Expertise".yellow().bold(), agent.expertise);
    }
    
    if !agent.focus.is_empty() {
        println!("{}: {}", "Focus".yellow().bold(), agent.focus);
    }
    
    if !agent.perspective.is_empty() {
        println!("{}: {}", "Perspective".yellow().bold(), agent.perspective);
    }
    
    if agent.usage_count > 0 {
        println!("{}: {} times", "Usage".yellow().bold(), agent.usage_count);
    }
    
    println!();
    println!("To load this agent:");
    println!("  ci load {}", agent.name);
    println!("  ci load {} --context=<context>", agent.name);
    
    Ok(())
}

/// Main agents command handler
pub fn agents(
    detailed: bool,
    filter: Option<String>,
    agent_name: Option<String>,
    config: &Config,
) -> Result<()> {
    // Try to find AGENTS.md file
    let agents_file = config.ci_path.join("AGENTS.md");
    
    if !agents_file.exists() {
        // Show fallback message and try to read from source repository
        println!("Agent's toolkit does not contain AGENTS.md, falling back to source repository.");
        println!("Source repository path: {}", config.ci_path.display());
        println!("Reading agents from: {}", agents_file.display());
        println!();
        
        // For now, show the raw file content as a fallback
        if let Ok(content) = fs::read_to_string(&agents_file) {
            println!("{}", content);
            return Ok(());
        } else {
            return Err(anyhow::anyhow!("Could not find AGENTS.md file at {}", agents_file.display()));
        }
    }
    
    let agents = parse_agents_file(&agents_file)?;
    
    match (agent_name, detailed) {
        (Some(name), _) => display_specific_agent(&agents, &name),
        (None, true) => display_detailed_agents(&agents, filter.as_ref()),
        (None, false) => display_agent_summary(&agents, filter.as_ref()),
    }
}