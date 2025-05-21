//! CI CLI - Collaborative Intelligence Command Line Interface
//!
//! This is the main entry point for the CI CLI tool.

use clap::Parser;
use colored::*;

mod commands;
mod config;
mod error;
mod helpers;

use commands::{Cli, Commands};
use helpers::terminal::TerminalHelpers;

fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Set the terminal title to indicate CI CLI is active
    TerminalHelpers::set_terminal_title("CI CLI");
    
    // Handle empty command (show help)
    if cli.command.is_none() {
        TerminalHelpers::print_help_message();
        return Ok(());
    }
    
    // Load configuration
    let config = match config::Config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };
    
    // Process commands
    let result = match cli.command.unwrap() {
        Commands::Init { name, agents, integration, no_fast, ci_path } => {
            commands::init(&name, &agents, &integration, no_fast, ci_path.as_deref(), &config)
        },
        
        Commands::Integrate { path, agents, integration, no_fast, ci_path } => {
            commands::integrate(path.as_deref(), &agents, &integration, no_fast, ci_path.as_deref(), &config)
        },
        
        Commands::Status { path } => {
            commands::status(path.as_deref(), &config)
        },
        
        Commands::Fix { path, fix_type } => {
            commands::fix(path.as_deref(), fix_type.as_deref(), &config)
        },
        
        Commands::Verify { path } => {
            commands::verify(path.as_deref(), &config)
        },
        
        Commands::Load { agent, context, path } => {
            commands::load(&agent, context.as_deref(), path.as_deref(), &config)
        },
        
        Commands::Map => {
            commands::map(&config)
        }
    };
    
    // Handle any errors
    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
    
    Ok(())
}