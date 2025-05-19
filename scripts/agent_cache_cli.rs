use std::env;
use std::error::Error;
use std::path::Path;
use std::process;

mod agent_cache_system;
use agent_cache_system::AgentCacheManager;

// Simple CLI wrapper for the agent cache system
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }
    
    let cache_dir = Path::new("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CACHE/SESSIONS");
    let mut cache_manager = AgentCacheManager::new(cache_dir, 60)?;
    
    match args[1].as_str() {
        "activate" => {
            if args.len() < 5 {
                println!("Error: Not enough arguments for activate command");
                print_usage();
                process::exit(1);
            }
            
            let agent_name = &args[2];
            let directory = &args[3];
            let initializer = &args[4];
            let intent = if args.len() > 5 { &args[5] } else { "No specific intent provided" };
            
            let entry_id = cache_manager.create_agent_cache(agent_name, directory, initializer, intent)?;
            cache_manager.force_persist()?;
            
            println!("Activated agent: {} (Entry ID: {})", agent_name, entry_id);
        },
        "deactivate" => {
            if args.len() < 3 {
                println!("Error: Not enough arguments for deactivate command");
                print_usage();
                process::exit(1);
            }
            
            let agent_name = &args[2];
            cache_manager.deactivate_agent(agent_name)?;
            
            println!("Deactivated agent: {}", agent_name);
        },
        "list" => {
            let active_agents = cache_manager.get_active_agents();
            
            if active_agents.is_empty() {
                println!("No active agents found.");
            } else {
                for agent in active_agents {
                    println!("- {}", agent);
                }
            }
        },
        "info" => {
            if args.len() < 3 {
                println!("Error: Not enough arguments for info command");
                print_usage();
                process::exit(1);
            }
            
            let agent_name = &args[2];
            if let Some(entry) = cache_manager.get_entry_for_agent(agent_name) {
                println!("Agent: {}", entry.agent_name);
                println!("Working Directory: {}", entry.directory);
                println!("Initializer: {}", entry.initializer);
                println!("Intent: {}", entry.intent);
                println!("Status: {:?}", entry.status);
            } else {
                println!("No active entry found for agent: {}", agent_name);
            }
        },
        "cleanup" => {
            let days = if args.len() > 2 { 
                args[2].parse().unwrap_or(7) 
            } else { 
                7 
            };
            
            let removed = cache_manager.cleanup_old_cache(days)?;
            println!("Cleaned up {} old cache files.", removed);
        },
        _ => {
            println!("Error: Unknown command '{}'", args[1]);
            print_usage();
            process::exit(1);
        }
    }
    
    Ok(())
}

fn print_usage() {
    println!("Agent Cache System CLI");
    println!("Usage:");
    println!("  agent_cache_cli activate <agent_name> <directory> <initializer> [intent]");
    println!("  agent_cache_cli deactivate <agent_name>");
    println!("  agent_cache_cli list");
    println!("  agent_cache_cli info <agent_name>");
    println!("  agent_cache_cli cleanup [days=7]");
    println!("");
    println!("Examples:");
    println!("  agent_cache_cli activate Athena /home/user/project User \"Memory research task\"");
    println!("  agent_cache_cli deactivate Athena");
    println!("  agent_cache_cli list");
    println!("  agent_cache_cli info Scholar");
    println!("  agent_cache_cli cleanup 14");
}