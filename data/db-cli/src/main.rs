use anyhow::Result;
use clap::{Parser, Subcommand};
use env_logger;
use log::{info, debug};
use std::path::PathBuf;

mod commands;
mod output;
mod database;

use commands::*;
use output::OutputFormat;

#[derive(Parser)]
#[command(
    name = "db",
    about = "CollaborativeIntelligence Database CLI",
    version,
    author,
    long_about = "A command-line interface for managing the CollaborativeIntelligence database system",
    arg_required_else_help = true
)]
struct Cli {
    /// Database directory path
    #[arg(short, long, global = true, env = "CI_DB_PATH", value_name = "DIR")]
    data_dir: Option<PathBuf>,
    
    /// Output format
    #[arg(short, long, value_enum, default_value = "table", global = true)]
    format: OutputFormat,
    
    /// Verbosity level
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Agent management
    #[command(subcommand)]
    Agent(AgentCommands),
    
    /// Learning system
    #[command(subcommand)]
    Learning(LearningCommands),
    
    /// Session management
    #[command(subcommand)]
    Session(SessionCommands),
    
    /// Knowledge base
    #[command(subcommand)]
    Knowledge(KnowledgeCommands),
    
    /// Project management
    #[command(subcommand)]
    Project(ProjectCommands),
    
    /// Metrics and analytics
    #[command(subcommand)]
    Metrics(MetricsCommands),
    
    /// Database administration
    #[command(subcommand)]
    Admin(AdminCommands),
    
    /// Database entities
    #[command(subcommand)]
    Entities(EntitiesCommands),
    
    /// Populate database from directory scan
    #[command(subcommand)]
    Populate(PopulateCommands),
    
    /// Query interface
    Query {
        /// SQL query to execute
        query: String,
        
        /// Target database (internal or projects)
        #[arg(short = 'b', long, default_value = "internal")]
        database: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(cli.verbose.log_level_filter())
        .init();
    
    // Determine database path
    let data_dir = cli.data_dir.unwrap_or_else(|| {
        let home = dirs::home_dir().expect("Could not find home directory");
        home.join(".ci-database")
    });
    
    debug!("Using database directory: {:?}", data_dir);
    
    // Create database instance
    let db = database::Database::new(&data_dir).await?;
    
    // Initialize if needed
    if !data_dir.join("internal/ci-internal.db").exists() {
        info!("Initializing database for first use...");
        db.initialize().await?;
        db.optimize().await?;
    }
    
    let api = database::CIDatabaseAPI::new(db.clone());
    
    // Execute command
    match cli.command {
        Commands::Agent(cmd) => agent::handle_command(cmd, &api, cli.format).await?,
        Commands::Learning(cmd) => learning::handle_command(cmd, &api, cli.format).await?,
        Commands::Session(cmd) => session::handle_command(cmd, &api, cli.format).await?,
        Commands::Knowledge(cmd) => knowledge::handle_command(cmd, &api, cli.format).await?,
        Commands::Project(cmd) => project::handle_command(cmd, &api, cli.format).await?,
        Commands::Metrics(cmd) => metrics::handle_command(cmd, &api, cli.format).await?,
        Commands::Admin(cmd) => admin::handle_command(cmd, &db, cli.format).await?,
        Commands::Entities(cmd) => entities::handle_command(cmd, &db, cli.format).await?,
        Commands::Populate(cmd) => populate::handle_command(cmd, &api, &db, cli.format).await?,
        Commands::Query { query, database } => {
            query::execute_query(&db, &query, &database, cli.format).await?
        }
    }
    
    Ok(())
}