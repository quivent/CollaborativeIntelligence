use anyhow::Result;
use crate::database::{CIDatabaseAPI, internal::CreateAgentRequest, Agent};
use prettytable::{Row, Cell};
use crate::output::{OutputFormat, OutputFormatter, TableDisplay};
use crate::commands::AgentCommands;

impl TableDisplay for Agent {
    fn to_table_row(&self) -> Row {
        Row::new(vec![
            Cell::new(&self.name),
            Cell::new(&self.agent_type),
            Cell::new(&self.status),
            Cell::new(&self.activation_count.to_string()),
            Cell::new(&self.created_at.format("%Y-%m-%d").to_string()),
        ])
    }
}

pub async fn handle_command(
    cmd: AgentCommands,
    api: &CIDatabaseAPI,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        AgentCommands::List { status, agent_type } => {
            let agents = api.list_agents().await?;
            
            // Apply filters
            let filtered: Vec<_> = agents.into_iter()
                .filter(|a| status.as_ref().map_or(true, |s| &a.status == s))
                .filter(|a| agent_type.as_ref().map_or(true, |t| &a.agent_type == t))
                .collect();
            
            match format {
                OutputFormat::Table => {
                    let mut table = OutputFormatter::table(vec![
                        "Name", "Type", "Status", "Activations", "Created"
                    ]);
                    
                    for agent in &filtered {
                        table.add_row(agent.to_table_row());
                    }
                    
                    table.printstd();
                    println!("\nTotal agents: {}", filtered.len());
                }
                _ => OutputFormatter::format(&filtered, format)?,
            }
        }
        
        AgentCommands::Show { name } => {
            let agent = api.get_agent(&name).await?;
            
            match format {
                OutputFormat::Table => {
                    println!("Agent: {}", agent.name);
                    println!("Type: {}", agent.agent_type);
                    println!("Status: {}", agent.status);
                    println!("Created: {}", agent.created_at);
                    if let Some(last_active) = agent.last_active {
                        println!("Last Active: {}", last_active);
                    }
                    println!("Activation Count: {}", agent.activation_count);
                    println!("Memory Version: {}", agent.memory_version);
                    if let Some(path) = &agent.memory_path {
                        println!("Memory Path: {}", path);
                    }
                }
                _ => OutputFormatter::format(&agent, format)?,
            }
        }
        
        AgentCommands::Create { name, agent_type, memory_path } => {
            let req = CreateAgentRequest {
                name: name.clone(),
                agent_type,
                memory_path,
            };
            
            let agent = api.create_agent(req).await?;
            OutputFormatter::success(&format!("Agent '{}' created", name));
            
            if matches!(format, OutputFormat::Json | OutputFormat::Yaml) {
                OutputFormatter::format(&agent, format)?;
            }
        }
        
        AgentCommands::Update { name, status, memory_version } => {
            if let Some(new_status) = status {
                if new_status == "active" {
                    api.activate_agent(&name).await?;
                } else if new_status == "inactive" {
                    api.deactivate_agent(&name).await?;
                } else {
                    return Err(anyhow::anyhow!("Invalid status. Use 'active' or 'inactive'"));
                }
                OutputFormatter::success(&format!("Agent '{}' status updated to '{}'", 
                    name, new_status));
            }
            
            if let Some(_version) = memory_version {
                // Memory version update would need to be implemented
                OutputFormatter::info("Memory version update not yet implemented");
            }
        }
        
        AgentCommands::Delete { name, force } => {
            if !force {
                println!("Are you sure you want to delete agent '{}'? (y/N)", name);
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    OutputFormatter::info("Deletion cancelled");
                    return Ok(());
                }
            }
            
            // Delete would need to be implemented in the API
            OutputFormatter::error("Agent deletion not yet implemented in API");
        }
    }
    
    Ok(())
}