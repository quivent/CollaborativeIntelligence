use tokio;
mod instruction_cache;
mod parallel_loader;
mod deduplication_engine; 
mod cache_persistence;
mod cache_validator;
mod benchmark_suite;
mod integration_demo;
mod claude_startup;

use claude_startup::*;
use integration_demo::run_integration_demo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🚀 Claude Code Instruction Cache System");
    println!("======================================");
    
    let args: Vec<String> = std::env::args().collect();
    
    match args.get(1).map(|s| s.as_str()) {
        Some("demo") => {
            println!("Running integration demo...\n");
            run_integration_demo().await?;
        }
        Some("init") => {
            println!("Initializing Claude with optimization...\n");
            let result = initialize_claude_with_optimization().await?;
            println!("{}", result);
        }
        Some("validate") => {
            println!("Validating cache health...\n");
            let report = validate_cache_health().await?;
            println!("{}", report);
        }
        Some("refresh") => {
            println!("Refreshing cache...\n");
            let result = refresh_cache().await?;
            println!("{}", result);
        }
        Some("help") | Some("-h") | Some("--help") => {
            print_help();
        }
        _ => {
            println!("🎯 Quick Start - Optimized Claude Initialization\n");
            let result = initialize_claude_with_optimization().await?;
            println!("{}", result);
        }
    }
    
    Ok(())
}

fn print_help() {
    println!("USAGE:");
    println!("  cargo run [COMMAND]");
    println!();
    println!("COMMANDS:");
    println!("  init      Initialize Claude with caching optimization");
    println!("  demo      Run complete integration demonstration");
    println!("  validate  Check cache health and validation status");
    println!("  refresh   Force refresh of instruction cache");
    println!("  help      Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("  cargo run                 # Quick optimized initialization");
    println!("  cargo run demo           # Full system demonstration");
    println!("  cargo run validate       # Check cache status");
    println!("  cargo run refresh        # Force cache rebuild");
}