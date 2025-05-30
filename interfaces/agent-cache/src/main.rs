use std::path::PathBuf;
use std::process;
use clap::{Parser, Subcommand};
use agent_cache::{
    AgentCacheManager,
    AgentCacheError
};

/// Agent Cache System CLI
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional cache directory (defaults to CACHE/SESSIONS in project root)
    #[arg(short, long)]
    cache_dir: Option<PathBuf>,
    
    /// Persistence interval in seconds
    #[arg(short, long, default_value = "60")]
    interval: u64,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Activate an agent
    Activate {
        /// Agent name
        agent_name: String,
        
        /// Working directory (defaults to current directory)
        #[arg(short, long)]
        directory: Option<String>,
        
        /// Initializer (who or what activated the agent, defaults to "CLI")
        #[arg(short, long, default_value = "CLI")]
        initializer: String,
        
        /// Intent (why the agent was activated)
        #[arg(short = 't', long, default_value = "Agent activation via CLI")]
        intent: String,
    },
    
    /// Deactivate an agent
    Deactivate {
        /// Agent name
        agent_name: String,
    },
    
    /// List active agents
    List,
    
    /// Get information about an agent
    Info {
        /// Agent name
        agent_name: String,
    },
    
    /// Clean up old cache files
    Cleanup {
        /// Remove files older than this many days
        #[arg(default_value = "7")]
        days: u64,
    },
}

fn main() {
    let cli = Cli::parse();
    
    // Determine cache directory
    let cache_dir = match &cli.cache_dir {
        Some(dir) => dir.clone(),
        None => {
            // Default to CACHE/SESSIONS in the project root
            let mut dir = std::env::current_dir().unwrap_or_else(|err| {
                eprintln!("Error getting current directory: {}", err);
                process::exit(1);
            });
            dir.push("CACHE");
            dir.push("SESSIONS");
            dir
        }
    };
    
    // Create cache manager
    let mut cache_manager = AgentCacheManager::new(&cache_dir, cli.interval)
        .unwrap_or_else(|err| {
            eprintln!("Error creating cache manager: {}", err);
            process::exit(1);
        });
    
    // Process command
    let result = match &cli.command {
        Commands::Activate { agent_name, directory, initializer, intent } => {
            println!("Activating agent: {}", agent_name);
            
            // Use current directory if none specified
            let dir = directory.clone().unwrap_or_else(|| {
                std::env::current_dir()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned()
            });
            
            cache_manager.create_agent_cache(agent_name, &dir, initializer, intent)
                .map(|id| {
                    println!("Agent activated successfully (ID: {})", id);
                    cache_manager.force_persist().unwrap_or_else(|err| {
                        eprintln!("Warning: Failed to persist cache: {}", err);
                    });
                })
        },
        
        Commands::Deactivate { agent_name } => {
            println!("Deactivating agent: {}", agent_name);
            cache_manager.deactivate_agent(agent_name)
                .map(|_| println!("Agent deactivated successfully"))
        },
        
        Commands::List => {
            let active_agents = cache_manager.get_active_agents();
            
            if active_agents.is_empty() {
                println!("No active agents found");
            } else {
                println!("Active agents:");
                for agent in active_agents {
                    println!("  - {}", agent);
                }
            }
            
            Ok(())
        },
        
        Commands::Info { agent_name } => {
            match cache_manager.get_entry_for_agent(agent_name) {
                Some(entry) => {
                    println!("Agent: {}", entry.agent_name);
                    println!("Working Directory: {}", entry.directory);
                    println!("Initializer: {}", entry.initializer);
                    println!("Intent: {}", entry.intent);
                    println!("Status: {}", entry.status);
                    Ok(())
                },
                None => Err(AgentCacheError::AgentNotFound(agent_name.clone()))
            }
        },
        
        Commands::Cleanup { days } => {
            println!("Cleaning up inactive cache files older than {} days", days);
            cache_manager.cleanup_old_cache(*days)
                .map(|count| println!("Removed {} old cache files", count))
        },
    };
    
    // Handle any errors
    if let Err(err) = result {
        match err {
            AgentCacheError::AgentNotFound(agent) => {
                eprintln!("Error: Agent not found: {}", agent);
                process::exit(1);
            },
            _ => {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
        }
    }
}