use std::error::Error;
use std::path::PathBuf;
use clap::{App, Arg, ArgMatches, SubCommand};
use memory_system::{
    MemorySystem,
    memory_segment_parser::MemorySegmentParser,
    memory_loader::{MemoryLoader, MemoryLoaderConfig},
    memory_activation_model::MemoryActivationModel,
    memory_updater::{MemoryUpdater, MemoryUpdaterConfig},
    memory_checksum_cache::{ChecksumCache, ChecksumCacheConfig}
};
use std::sync::{Arc, Mutex};
use colored::*;

// Import our agent activator
mod agent_activator;
use agent_activator::AgentActivator;

fn main() -> Result<(), Box<dyn Error>> {
    // Set up custom logger
    env_logger::init();

    // Parse command line arguments
    let matches = App::new("CI Agent")
        .version("1.0.0")
        .author("Collaborative Intelligence Team")
        .about("Agent management for Collaborative Intelligence")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Print verbose information"))
        .subcommand(SubCommand::with_name("activate")
            .about("Activate an agent for the current session")
            .arg(Arg::with_name("AGENT")
                .help("Agent name to activate")
                .required(true)
                .index(1))
            .arg(Arg::with_name("CONTEXT")
                .help("Optional context for activation")
                .index(2)))
        .subcommand(SubCommand::with_name("list")
            .about("List all available agents"))
        .subcommand(SubCommand::with_name("info")
            .about("Show detailed information about an agent")
            .arg(Arg::with_name("AGENT")
                .help("Agent name to show information about")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("create")
            .about("Create a new agent")
            .arg(Arg::with_name("AGENT")
                .help("Name for the new agent")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("enable")
            .about("Enable an agent in the current project")
            .arg(Arg::with_name("AGENT")
                .help("Agent name to enable")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("disable")
            .about("Disable an agent in the current project")
            .arg(Arg::with_name("AGENT")
                .help("Agent name to disable")
                .required(true)
                .index(1)))
        .get_matches();

    // Handle commands
    match matches.subcommand() {
        ("activate", Some(sub_matches)) => {
            activate_agent_command(sub_matches)?;
        }
        ("list", Some(_)) => {
            list_agents_command()?;
        }
        ("info", Some(sub_matches)) => {
            show_agent_info_command(sub_matches)?;
        }
        ("create", Some(sub_matches)) => {
            create_agent_command(sub_matches)?;
        }
        ("enable", Some(sub_matches)) => {
            enable_agent_command(sub_matches)?;
        }
        ("disable", Some(sub_matches)) => {
            disable_agent_command(sub_matches)?;
        }
        _ => {
            // Show help by default
            println!("No command specified. Use --help for available commands.");
        }
    }

    Ok(())
}

// Activate agent command
fn activate_agent_command(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let agent_name = matches.value_of("AGENT").unwrap();
    let context = matches.value_of("CONTEXT");
    
    println!("Activating agent: {}", agent_name.green());
    
    // Create agent activator
    let activator = AgentActivator::new()?;
    
    // Activate agent
    let task = context.unwrap_or("General Task");
    match activator.activate_agent(agent_name, context, task) {
        Ok(result) => {
            // Show activation info
            activator.show_activation_info(&result);
            
            // Output memory for Claude
            activator.output_memory_for_claude(&result)?;
        }
        Err(e) => {
            eprintln!("{}: Failed to activate agent {}: {}", "Error".red().bold(), agent_name, e);
            return Err(e);
        }
    }
    
    Ok(())
}

// List agents command
fn list_agents_command() -> Result<(), Box<dyn Error>> {
    println!("{}", "Available Agents:".bold());
    
    // Create agent activator
    let activator = AgentActivator::new()?;
    
    // List agents
    match activator.list_agents() {
        Ok(agents) => {
            for (i, agent) in agents.iter().enumerate() {
                println!("  {}. {}", i + 1, agent);
            }
            println!("\nTotal agents: {}", agents.len());
        }
        Err(e) => {
            eprintln!("{}: Failed to list agents: {}", "Error".red().bold(), e);
            return Err(e);
        }
    }
    
    Ok(())
}

// Show agent info command
fn show_agent_info_command(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let agent_name = matches.value_of("AGENT").unwrap();
    
    // Create agent activator
    let activator = AgentActivator::new()?;
    
    // Show agent info
    if let Err(e) = activator.show_agent_info(agent_name) {
        eprintln!("{}: Failed to show agent information: {}", "Error".red().bold(), e);
        return Err(e);
    }
    
    Ok(())
}

// Create agent command
fn create_agent_command(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let agent_name = matches.value_of("AGENT").unwrap();
    
    println!("Creating new agent: {}", agent_name.green());
    
    // Implementation would use the memory system to create a new agent
    println!("This feature will be implemented in a future version");
    
    Ok(())
}

// Enable agent command
fn enable_agent_command(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let agent_name = matches.value_of("AGENT").unwrap();
    
    println!("Enabling agent: {}", agent_name.green());
    
    // Implementation would use the memory system to enable an agent
    println!("This feature will be implemented in a future version");
    
    Ok(())
}

// Disable agent command
fn disable_agent_command(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let agent_name = matches.value_of("AGENT").unwrap();
    
    println!("Disabling agent: {}", agent_name.green());
    
    // Implementation would use the memory system to disable an agent
    println!("This feature will be implemented in a future version");
    
    Ok(())
}