//! CIA CLI - Collaborative Intelligence Adaption CLI for CollaborativeIntelligence repository intelligence adaption
//!
//! This is the main entry point for the CIA CLI tool.

use clap::{Parser, Subcommand};
use colored::*;

mod commands;
mod utils;
mod config;
mod helpers;
mod error;
mod agent_sync;
mod agent_creation;
mod terminal;
mod ci_integration;
mod security;

#[derive(Parser)]
#[clap(name = "CIA")]
#[clap(about = "Collaborative Intelligence Adaption CLI for CollaborativeIntelligence repository intelligence adaption", long_about = None)]
#[clap(version)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Agent management commands
    #[clap(subcommand)]
    Agent(AgentCommands),
    
    /// Memory system commands
    #[clap(subcommand)]
    Memory(MemoryCommands),
    
    /// Knowledge management commands
    #[clap(subcommand)]
    Knowledge(KnowledgeCommands),
    
    /// Debug utilities for development
    #[clap(subcommand)]
    Debug(DebugCommands),
    
    /// System maintenance commands
    #[clap(subcommand)]
    System(SystemCommands),
    
    /// Task management commands
    #[clap(subcommand)]
    Task(TaskCommands),
    
    /// Repository topology and multi-repository management
    #[clap(subcommand)]
    Topologist(TopologistCommands),
    
    /// CI integration and agent management
    #[clap(subcommand)]
    Ci(CiCommands),
    
    /// Security and authentication commands
    #[clap(subcommand)]
    Security(SecurityCommands),
}

#[derive(Subcommand)]
enum AgentCommands {
    /// Activate an agent
    Activate {
        /// Name of the agent to activate
        #[clap(value_parser)]
        name: String,
    },
    
    /// List all available agents
    List,
    
    /// Create a new agent with interactive prompts
    Create {
        /// Name of the agent to create
        #[clap(value_parser)]
        name: String,
        
        /// Role/specialization of the agent
        #[clap(long, short)]
        role: Option<String>,
        
        /// Purpose description of the agent
        #[clap(long, short)]
        purpose: Option<String>,
        
        /// Skip interactive prompts and use defaults
        #[clap(long)]
        quick: bool,
    },
    
    /// Synchronize agent registry files
    Sync,
    
    /// Validate an existing agent structure
    Validate {
        /// Name of the agent to validate
        #[clap(value_parser)]
        name: String,
    },
    
    /// Recommend the best agent for a specific task or query
    For {
        /// The task, query, or prompt to find an agent for
        #[clap(value_parser)]
        query: String,
    },
}

#[derive(Subcommand)]
enum MemoryCommands {
    /// Load agent memory
    Load {
        /// Name of the agent to load memory for
        #[clap(value_parser)]
        agent: String,
    },
    
    /// Optimize agent memory
    Optimize {
        /// Name of the agent to optimize memory for
        #[clap(value_parser)]
        agent: String,
    },
    
    /// Cache agent memory
    Cache {
        /// Name of the agent to cache memory for
        #[clap(value_parser)]
        agent: String,
    },
    
    /// Validate agent memory files with checksum verification
    Validate {
        /// Name of the agent to validate memory for
        #[clap(value_parser)]
        agent: String,
    },
    
    /// Show memory statistics for an agent
    Stats {
        /// Name of the agent to show stats for
        #[clap(value_parser)]
        agent: String,
    },
}

#[derive(Subcommand)]
enum KnowledgeCommands {
    /// Search the knowledge registry
    Search {
        /// Query to search for
        #[clap(value_parser)]
        query: String,
        
        /// Optional category to limit search to
        #[clap(value_parser)]
        category: Option<String>,
    },
    
    /// Update knowledge registry indexes
    Index,
    
    /// Distribute knowledge to agents
    Distribute {
        /// Optional agent to distribute to, otherwise distributes to all
        #[clap(value_parser)]
        agent: Option<String>,
    },
    
    /// Run complete knowledge distribution workflow
    Workflow {
        /// Minimum relevance threshold (0.0-1.0)
        #[clap(long, short)]
        threshold: Option<f64>,
        
        /// Generate reports without making changes
        #[clap(long)]
        report_only: bool,
    },
}

#[derive(Subcommand)]
enum DebugCommands {
    /// Print repo path information
    Paths,
    
    /// Check system configuration
    Config,
    
    /// Test terminal formatting
    Format,
    
    /// Display visual map of CLI commands
    Map,
}

#[derive(Subcommand)]
enum SystemCommands {
    /// Clean temporary files
    Clean {
        /// Specify if agent cache should be cleaned
        #[clap(long)]
        cache: bool,
    },
    
    /// Verify repository structure
    Verify,
    
    /// View categorized repository structure for refactoring insights
    View,
    
    /// Update terminal title
    Title {
        /// Custom title to set
        #[clap(value_parser)]
        title: Option<String>,
    },
    
    /// Override terminal title with CIA contextual format
    Override {
        /// Context information for the override
        #[clap(value_parser)]
        context: Option<String>,
    },
}

#[derive(Subcommand)]
enum TaskCommands {
    /// Start a new task
    Start {
        /// Description of the task
        #[clap(value_parser)]
        description: String,
    },
    
    /// Complete the current task
    Complete,
    
    /// Clear the current task
    Clear,
    
    /// Show current task status
    Status,
    
    /// Check for task timeout
    Check,
}

#[derive(Subcommand)]
enum TopologistCommands {
    /// Initialize multi-repository environment
    Init {
        /// Root directory containing multiple repositories
        #[clap(short, long)]
        root_path: String,
        
        /// Generate configuration file
        #[clap(short, long)]
        config: bool,
    },
    
    /// Execute sequential commit protocol
    Commit {
        /// Root directory containing repositories
        #[clap(short, long)]
        root_path: Option<String>,
        
        /// Custom commit message
        #[clap(short, long)]
        message: Option<String>,
        
        /// Dry run - show what would be committed
        #[clap(short, long)]
        dry_run: bool,
        
        /// Skip dependency validation
        #[clap(long)]
        skip_deps: bool,
    },
    
    /// Analyze repository topology and dependencies
    Analyze {
        /// Root directory containing repositories
        #[clap(short, long)]
        root_path: String,
        
        /// Output format (json, table)
        #[clap(short, long, default_value = "table")]
        format: String,
    },
}

#[derive(Subcommand)]
enum CiCommands {
    /// Show CI agents status and available agents
    Agents,
    
    /// Show CI integration status
    Status,
    
    /// Test and establish CI connection
    Connect,
    
    /// Activate a specific CI agent
    Activate {
        /// Name of the agent to activate
        #[clap(value_parser)]
        name: String,
    },
    
    /// Check CI integration health
    Health,
    
    /// Validate CI integration configuration
    Validate,
    
    /// Repair CI integration issues
    Repair {
        /// Perform a dry run without making changes
        #[clap(long)]
        dry_run: bool,
    },
    
    /// Monitor CI integration health continuously
    Monitor {
        /// Monitoring interval in seconds
        #[clap(long, default_value = "30")]
        interval: u64,
    },
}

#[derive(Subcommand)]
enum SecurityCommands {
    /// Verify agent identity (emergency mode - all unverified)
    VerifyAgent {
        /// Name of the agent to verify
        #[clap(value_parser)]
        name: String,
    },
    
    /// Challenge agent identity with authentication request
    ChallengeIdentity {
        /// Name of the agent to challenge
        #[clap(value_parser)]
        name: String,
    },
    
    /// Show security system status
    SystemStatus,
    
    /// Show emergency mode information
    EmergencyMode,
    
    /// Generate security incident report
    Report,
}


fn main() {
    // CRITICAL: Enforce CI title override before any other operations
    if let Ok(detector) = ci_integration::CIIntegrationDetector::new() {
        if detector.is_ci_integrated() {
            println!("✓ CI integration detected - enforcing title override");
            if let Err(e) = detector.enforce_title_override() {
                eprintln!("Warning: CI title override enforcement failed: {}", e);
            } else {
                println!("✓ CI title override enforcement completed successfully");
            }
        } else {
            println!("ⓘ No CI integration detected");
        }
    } else {
        println!("⚠ CI integration detector failed to initialize");
    }

    // Check for empty arguments to show custom help
    if std::env::args().len() <= 1 {
        // Set the terminal title using contextual system
        if let Err(e) = terminal::initialize_title_system() {
            eprintln!("Warning: Failed to initialize terminal title system: {}", e);
        }
        
        // Show our custom styled help message
        helpers::terminal::TerminalHelpers::print_help_message();
        return;
    }
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Initialize terminal title system
    if let Err(e) = terminal::initialize_title_system() {
        eprintln!("Warning: Failed to initialize terminal title system: {}", e);
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
    let result = match &cli.command {
        Commands::Agent(agent_cmd) => match agent_cmd {
            AgentCommands::Activate { name } => commands::agent::activate(name, &config),
            AgentCommands::List => commands::agent::list(&config),
            AgentCommands::Create { name, role, purpose, quick } => commands::agent::create(name, role, purpose, *quick, &config),
            AgentCommands::Sync => commands::agent::sync(&config),
            AgentCommands::Validate { name } => commands::agent::validate(name, &config),
            AgentCommands::For { query } => commands::agent::recommend_for(query, &config),
        },
        
        Commands::Memory(memory_cmd) => match memory_cmd {
            MemoryCommands::Load { agent } => commands::memory::load(agent, &config),
            MemoryCommands::Optimize { agent } => commands::memory::optimize(agent, &config),
            MemoryCommands::Cache { agent } => commands::memory::cache(agent, &config),
            MemoryCommands::Validate { agent } => commands::memory::validate(agent, &config),
            MemoryCommands::Stats { agent } => commands::memory::stats(agent, &config),
        },
        
        Commands::Knowledge(knowledge_cmd) => match knowledge_cmd {
            KnowledgeCommands::Search { query, category } => commands::knowledge::search(query, category, &config),
            KnowledgeCommands::Index => commands::knowledge::index(&config),
            KnowledgeCommands::Distribute { agent } => commands::knowledge::distribute(agent, &config),
            KnowledgeCommands::Workflow { threshold, report_only } => commands::knowledge::workflow(&config, *threshold, *report_only),
        },
        
        Commands::Debug(debug_cmd) => match debug_cmd {
            DebugCommands::Paths => commands::debug::paths(&config),
            DebugCommands::Config => commands::debug::config(&config),
            DebugCommands::Format => commands::debug::format(&config),
            DebugCommands::Map => commands::debug::map(&config),
        },
        
        Commands::System(system_cmd) => match system_cmd {
            SystemCommands::Clean { cache } => commands::system::clean(*cache, &config),
            SystemCommands::Verify => commands::system::verify(&config),
            SystemCommands::View => commands::system::view::view(&config),
            SystemCommands::Title { title } => commands::system::title(title, &config),
            SystemCommands::Override { context } => commands::system::override_title(context, &config),
        },
        
        Commands::Task(task_cmd) => match task_cmd {
            TaskCommands::Start { description } => commands::task::start(description, &config),
            TaskCommands::Complete => commands::task::complete(&config),
            TaskCommands::Clear => commands::task::clear(&config),
            TaskCommands::Status => commands::task::status(&config),
            TaskCommands::Check => commands::task::check(&config),
        },
        
        Commands::Topologist(topologist_cmd) => match topologist_cmd {
            TopologistCommands::Init { root_path, config: gen_config } => commands::topologist::init(root_path, *gen_config, &config),
            TopologistCommands::Commit { root_path, message, dry_run, skip_deps } => commands::topologist::commit(root_path, message, *dry_run, *skip_deps, &config),
            TopologistCommands::Analyze { root_path, format } => commands::topologist::analyze(root_path, format, &config),
        },
        
        Commands::Ci(ci_cmd) => match ci_cmd {
            CiCommands::Agents => commands::ci::agents(&config),
            CiCommands::Status => commands::ci::status(&config),
            CiCommands::Connect => commands::ci::connect(&config),
            CiCommands::Activate { name } => commands::ci::activate(name, &config),
            CiCommands::Health => commands::ci::health(&config),
            CiCommands::Validate => commands::ci::validate(&config),
            CiCommands::Repair { dry_run } => commands::ci::repair(*dry_run, &config),
            CiCommands::Monitor { interval } => commands::ci::monitor(*interval, &config),
        },
        
        Commands::Security(security_cmd) => match security_cmd {
            SecurityCommands::VerifyAgent { name } => {
                security::commands::verify_agent(name).map_err(|e| anyhow::Error::new(e))
            },
            SecurityCommands::ChallengeIdentity { name } => {
                security::commands::challenge_identity(name).map_err(|e| anyhow::Error::new(e))
            },
            SecurityCommands::SystemStatus => {
                security::commands::system_status().map_err(|e| anyhow::Error::new(e))
            },
            SecurityCommands::EmergencyMode => {
                security::commands::emergency_mode().map_err(|e| anyhow::Error::new(e))
            },
            SecurityCommands::Report => {
                let enforcer = security::SecurityEnforcer::new();
                println!("{}", enforcer.generate_security_report());
                Ok(())
            },
        },
        
    };
    
    // Handle any errors
    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}