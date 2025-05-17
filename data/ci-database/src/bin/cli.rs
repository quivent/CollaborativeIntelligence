use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use ci_database::{Database, CIDatabaseAPI};
use ci_database::internal::{CreateAgentRequest, RecordLearningRequest, CreateSessionRequest, AddKnowledgeRequest};
use ci_database::projects::{RegisterProjectRequest, LogOperationRequest, RecordMetricRequest};
use std::path::PathBuf;
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "ci-db")]
#[command(about = "CI Database Management CLI", long_about = None)]
struct Cli {
    /// Database directory path
    #[arg(short, long, env = "CI_DB_PATH")]
    data_dir: Option<PathBuf>,
    
    /// Verbosity level
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Agent management commands
    #[command(subcommand)]
    Agent(AgentCommands),
    
    /// Learning system commands
    #[command(subcommand)]
    Learning(LearningCommands),
    
    /// Session management commands
    #[command(subcommand)]
    Session(SessionCommands),
    
    /// Knowledge base commands
    #[command(subcommand)]
    Knowledge(KnowledgeCommands),
    
    /// Project management commands
    #[command(subcommand)]
    Project(ProjectCommands),
    
    /// Operation logging commands
    #[command(subcommand)]
    Operation(OperationCommands),
    
    /// Metrics commands
    #[command(subcommand)]
    Metric(MetricCommands),
    
    /// Database utilities
    #[command(subcommand)]
    Db(DbCommands),
    
    /// Advanced features
    #[command(subcommand)]
    Advanced(AdvancedCommands),
}

#[derive(Subcommand)]
enum AgentCommands {
    /// Register a new agent
    Register {
        name: String,
        #[arg(short = 't', long)]
        agent_type: String,
        #[arg(long)]
        memory_path: Option<String>,
    },
    
    /// Get agent details
    Get { name: String },
    
    /// List all agents
    List,
    
    /// Activate an agent
    Activate { name: String },
    
    /// Deactivate an agent
    Deactivate { name: String },
}

#[derive(Subcommand)]
enum LearningCommands {
    /// Record a learning entry
    Record {
        agent: String,
        trigger: String,
        content: String,
        #[arg(long)]
        importance: Option<f64>,
        #[arg(long)]
        category: Option<String>,
    },
    
    /// Get learnings for an agent
    Get { agent: String },
}

#[derive(Subcommand)]
enum SessionCommands {
    /// Create a new session
    Create {
        agent: String,
        session_type: String,
    },
    
    /// Close a session
    Close {
        id: i64,
        #[arg(long)]
        outcomes: Option<String>,
    },
}

#[derive(Subcommand)]
enum KnowledgeCommands {
    /// Add knowledge entry
    Add {
        agent: String,
        knowledge_type: String,
        content: String,
        #[arg(long)]
        category: Option<String>,
        #[arg(long)]
        confidence: Option<f64>,
    },
    
    /// Search knowledge base
    Search { query: String },
}

#[derive(Subcommand)]
enum ProjectCommands {
    /// Register a new project
    Register {
        name: String,
        path: String,
        #[arg(long)]
        project_type: Option<String>,
        #[arg(long)]
        ci_version: Option<String>,
    },
    
    /// Get project details
    Get { name: String },
    
    /// List all projects
    List,
}

#[derive(Subcommand)]
enum OperationCommands {
    /// Log an operation
    Log {
        project: String,
        command: String,
        #[arg(long)]
        agent: Option<String>,
    },
    
    /// Complete an operation
    Complete {
        id: i64,
        #[arg(long)]
        status: String,
        #[arg(long)]
        duration: i64,
        #[arg(long)]
        error: Option<String>,
    },
}

#[derive(Subcommand)]
enum MetricCommands {
    /// Record a metric
    Record {
        project: String,
        metric_type: String,
        value: f64,
    },
    
    /// Get metrics
    Get {
        project: String,
        metric_type: String,
    },
}

#[derive(Subcommand)]
enum DbCommands {
    /// Initialize databases
    Init,
    
    /// Run migrations
    Migrate,
    
    /// Backup databases
    Backup {
        #[arg(short, long)]
        output: PathBuf,
    },
}

#[derive(Subcommand)]
enum AdvancedCommands {
    /// Export database to JSON
    Export {
        #[arg(short, long)]
        output: PathBuf,
        #[arg(long)]
        include_sensitive: bool,
    },
    
    /// Import data from JSON
    Import {
        #[arg(short, long)]
        input: PathBuf,
    },
    
    /// Generate analytics report
    Analytics {
        #[arg(long, default_value = "30")]
        days: i64,
    },
    
    /// Perform health check
    Health,
    
    /// Generate knowledge graph data
    Graph,
    
    /// Automated backup with retention
    AutoBackup {
        #[arg(short, long)]
        backup_dir: PathBuf,
        #[arg(long, default_value = "30")]
        retention_days: i64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let log_level = match cli.verbose.log_level() {
        Some(level) => level.as_str(),
        None => "warn",
    };
    
    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .init();
    
    // Determine database path
    let data_dir = cli.data_dir.unwrap_or_else(|| {
        let home = dirs::home_dir().expect("Could not find home directory");
        home.join(".ci-database")
    });
    
    // Create database instance
    let db = Database::new(&data_dir).await?;
    let api = CIDatabaseAPI::new(db.clone());
    
    // Execute command
    match cli.command {
        Commands::Agent(cmd) => handle_agent_command(cmd, &api).await?,
        Commands::Learning(cmd) => handle_learning_command(cmd, &api).await?,
        Commands::Session(cmd) => handle_session_command(cmd, &api).await?,
        Commands::Knowledge(cmd) => handle_knowledge_command(cmd, &api).await?,
        Commands::Project(cmd) => handle_project_command(cmd, &api).await?,
        Commands::Operation(cmd) => handle_operation_command(cmd, &api).await?,
        Commands::Metric(cmd) => handle_metric_command(cmd, &api).await?,
        Commands::Db(cmd) => handle_db_command(cmd, &db).await?,
        Commands::Advanced(cmd) => handle_advanced_command(cmd, &db).await?,
    }
    
    Ok(())
}

async fn handle_agent_command(cmd: AgentCommands, api: &CIDatabaseAPI) -> Result<()> {
    match cmd {
        AgentCommands::Register { name, agent_type, memory_path } => {
            let req = CreateAgentRequest {
                name: name.clone(),
                agent_type,
                capabilities: None,
                memory_path,
                config: None,
            };
            
            let agent = api.create_agent(req).await?;
            println!("{} Agent '{}' registered successfully", "✓".green(), name);
            println!("ID: {}", agent.id.unwrap());
        }
        
        AgentCommands::Get { name } => {
            let agent = api.get_agent(&name).await?;
            println!("Agent: {}", agent.name);
            println!("Type: {}", agent.agent_type);
            println!("Status: {}", agent.status);
            println!("Created: {}", agent.created_at);
            if let Some(last_active) = agent.last_active {
                println!("Last Active: {}", last_active);
            }
            println!("Activation Count: {}", agent.activation_count);
        }
        
        AgentCommands::List => {
            let agents = api.list_agents().await?;
            println!("Total agents: {}", agents.len());
            println!();
            for agent in agents {
                println!("{:20} {:15} {:10} {}", 
                    agent.name, 
                    agent.agent_type, 
                    agent.status, 
                    agent.activation_count
                );
            }
        }
        
        AgentCommands::Activate { name } => {
            api.activate_agent(&name).await?;
            println!("{} Agent '{}' activated", "✓".green(), name);
        }
        
        AgentCommands::Deactivate { name } => {
            api.deactivate_agent(&name).await?;
            println!("{} Agent '{}' deactivated", "✓".green(), name);
        }
    }
    
    Ok(())
}

async fn handle_learning_command(cmd: LearningCommands, api: &CIDatabaseAPI) -> Result<()> {
    match cmd {
        LearningCommands::Record { agent, trigger, content, importance, category } => {
            let req = RecordLearningRequest {
                agent_name: agent.clone(),
                trigger_type: trigger,
                content,
                importance,
                category,
                metadata: None,
            };
            
            let learning = api.record_learning(req).await?;
            println!("{} Learning recorded for agent '{}'", "✓".green(), agent);
            println!("ID: {}", learning.id.unwrap());
        }
        
        LearningCommands::Get { agent } => {
            let learnings = api.get_learnings(&agent).await?;
            println!("Learnings for agent '{}': {}", agent, learnings.len());
            println!();
            
            for learning in learnings {
                println!("{} - {} ({})", 
                    learning.timestamp.format("%Y-%m-%d %H:%M:%S"),
                    learning.trigger_type,
                    learning.importance
                );
                println!("  {}", learning.content);
                if let Some(category) = &learning.category {
                    println!("  Category: {}", category);
                }
                println!();
            }
        }
    }
    
    Ok(())
}

async fn handle_session_command(cmd: SessionCommands, api: &CIDatabaseAPI) -> Result<()> {
    match cmd {
        SessionCommands::Create { agent, session_type } => {
            let req = CreateSessionRequest {
                agent_name: agent.clone(),
                session_type: session_type.clone(),
                metadata: None,
            };
            
            let session = api.create_session(req).await?;
            println!("{} Session created for agent '{}'", "✓".green(), agent);
            println!("ID: {}", session.id.unwrap());
            println!("Type: {}", session_type);
        }
        
        SessionCommands::Close { id, outcomes } => {
            let outcomes_json = if let Some(outcomes_str) = outcomes {
                Some(serde_json::from_str(&outcomes_str)?)
            } else {
                None
            };
            
            api.close_session(id, outcomes_json).await?;
            println!("{} Session {} closed", "✓".green(), id);
        }
    }
    
    Ok(())
}

async fn handle_knowledge_command(cmd: KnowledgeCommands, api: &CIDatabaseAPI) -> Result<()> {
    match cmd {
        KnowledgeCommands::Add { agent, knowledge_type, content, category, confidence } => {
            let req = AddKnowledgeRequest {
                agent_name: agent.clone(),
                knowledge_type,
                content,
                category,
                confidence,
                metadata: None,
            };
            
            let knowledge = api.add_knowledge(req).await?;
            println!("{} Knowledge added for agent '{}'", "✓".green(), agent);
            println!("ID: {}", knowledge.id.unwrap());
        }
        
        KnowledgeCommands::Search { query } => {
            let results = api.search_knowledge(&query).await?;
            println!("Found {} results for '{}'", results.len(), query);
            println!();
            
            for knowledge in results {
                println!("ID: {} - Type: {} (Confidence: {})", 
                    knowledge.id.unwrap(),
                    knowledge.knowledge_type,
                    knowledge.confidence
                );
                println!("  {}", knowledge.content);
                if let Some(category) = &knowledge.category {
                    println!("  Category: {}", category);
                }
                println!();
            }
        }
    }
    
    Ok(())
}

async fn handle_project_command(cmd: ProjectCommands, api: &CIDatabaseAPI) -> Result<()> {
    match cmd {
        ProjectCommands::Register { name, path, project_type, ci_version } => {
            let req = RegisterProjectRequest {
                name: name.clone(),
                path,
                project_type,
                ci_version,
                metadata: None,
                config: None,
            };
            
            let project = api.register_project(req).await?;
            println!("{} Project '{}' registered", "✓".green(), name);
            println!("ID: {}", project.id.unwrap());
        }
        
        ProjectCommands::Get { name } => {
            let project = api.get_project(&name).await?;
            println!("Project: {}", project.name);
            println!("Path: {}", project.path);
            println!("Type: {}", project.project_type.unwrap_or_default());
            println!("Status: {}", project.status);
            println!("Created: {}", project.created_at);
            if let Some(last_accessed) = project.last_accessed {
                println!("Last Accessed: {}", last_accessed);
            }
        }
        
        ProjectCommands::List => {
            let projects = api.list_projects().await?;
            println!("Total projects: {}", projects.len());
            println!();
            
            for project in projects {
                println!("{:20} {:10} {}", 
                    project.name,
                    project.status,
                    project.path
                );
            }
        }
    }
    
    Ok(())
}

async fn handle_operation_command(cmd: OperationCommands, api: &CIDatabaseAPI) -> Result<()> {
    match cmd {
        OperationCommands::Log { project, command, agent } => {
            let req = LogOperationRequest {
                project_name: project.clone(),
                command: command.clone(),
                arguments: None,
                agent_used: agent,
                metadata: None,
            };
            
            let operation = api.log_operation(req).await?;
            println!("{} Operation logged for project '{}'", "✓".green(), project);
            println!("ID: {}", operation.id.unwrap());
        }
        
        OperationCommands::Complete { id, status, duration, error } => {
            api.complete_operation(id, &status, duration, error).await?;
            println!("{} Operation {} completed", "✓".green(), id);
        }
    }
    
    Ok(())
}

async fn handle_metric_command(cmd: MetricCommands, api: &CIDatabaseAPI) -> Result<()> {
    match cmd {
        MetricCommands::Record { project, metric_type, value } => {
            let req = RecordMetricRequest {
                project_name: project.clone(),
                metric_type: metric_type.clone(),
                value,
                context: None,
            };
            
            let metric = api.record_metric(req).await?;
            println!("{} Metric recorded for project '{}'", "✓".green(), project);
            println!("ID: {}", metric.id.unwrap());
        }
        
        MetricCommands::Get { project, metric_type } => {
            let metrics = api.get_metrics(&project, &metric_type).await?;
            println!("Metrics for '{}' - '{}': {}", project, metric_type, metrics.len());
            println!();
            
            for metric in metrics {
                println!("{} - {}", 
                    metric.timestamp.format("%Y-%m-%d %H:%M:%S"),
                    metric.value
                );
            }
        }
    }
    
    Ok(())
}

async fn handle_db_command(cmd: DbCommands, db: &Database) -> Result<()> {
    match cmd {
        DbCommands::Init => {
            db.initialize().await?;
            println!("{} Databases initialized", "✓".green());
        }
        
        DbCommands::Migrate => {
            // Migrations are run automatically on connection
            println!("{} Migrations complete", "✓".green());
        }
        
        DbCommands::Backup { output } => {
            db.backup(&output).await?;
            println!("{} Backup created at {}", "✓".green(), output.display());
        }
    }
    
    Ok(())
}

async fn handle_advanced_command(cmd: AdvancedCommands, db: &Database) -> Result<()> {
    use ci_database::advanced_features::AdvancedFeatures;
    let advanced = AdvancedFeatures::new(db.clone());
    
    match cmd {
        AdvancedCommands::Export { output, include_sensitive } => {
            let export = advanced.export_to_json(include_sensitive).await?;
            let json = serde_json::to_string_pretty(&export)?;
            std::fs::write(&output, json)?;
            println!("{} Database exported to {}", "✓".green(), output.display());
        }
        
        AdvancedCommands::Import { input } => {
            let data = std::fs::read_to_string(&input)?;
            let summary = advanced.import_from_json(&data).await?;
            println!("{} Import completed", "✓".green());
            println!("Agents imported: {}", summary.agents_imported);
            println!("Agents skipped: {}", summary.agents_skipped);
            println!("Projects imported: {}", summary.projects_imported);
            println!("Projects skipped: {}", summary.projects_skipped);
        }
        
        AdvancedCommands::Analytics { days } => {
            let report = advanced.generate_analytics(days).await?;
            
            println!("Analytics Report ({} days)", days);
            println!("================");
            
            println!("\nTop Agents by Activity:");
            for agent in report.agent_activity.iter().take(5) {
                println!("  {} - {} sessions (avg {:.1} min)",
                    agent.agent_name,
                    agent.session_count,
                    agent.avg_duration_minutes
                );
            }
            
            println!("\nTop Projects by Operations:");
            for project in report.project_activity.iter().take(5) {
                println!("  {} - {} operations (avg {:.0} ms)",
                    project.project_name,
                    project.operation_count,
                    project.avg_duration_ms
                );
            }
            
            println!("\nLearning Trends:");
            for trend in report.learning_trends.iter().take(7) {
                println!("  {} - {} learnings (avg importance: {:.2})",
                    trend.date,
                    trend.count,
                    trend.avg_importance
                );
            }
        }
        
        AdvancedCommands::Health => {
            let health = advanced.health_check().await?;
            
            println!("Health Check Report");
            println!("==================");
            println!("Status: {}", 
                if health.status == "healthy" { 
                    health.status.green() 
                } else { 
                    health.status.red() 
                }
            );
            println!("Internal DB: {}", 
                if health.internal_db_ok { "OK".green() } else { "ERROR".red() }
            );
            println!("Projects DB: {}", 
                if health.projects_db_ok { "OK".green() } else { "ERROR".red() }
            );
            println!("Internal Size: {:.2} MB", health.internal_size_bytes as f64 / 1_048_576.0);
            println!("Projects Size: {:.2} MB", health.projects_size_bytes as f64 / 1_048_576.0);
            println!("Total Agents: {}", health.total_agents);
            println!("Total Projects: {}", health.total_projects);
            println!("Last Check: {}", health.last_check);
        }
        
        AdvancedCommands::Graph => {
            let graph = advanced.knowledge_graph_data().await?;
            let json = serde_json::to_string_pretty(&graph)?;
            println!("{}", json);
        }
        
        AdvancedCommands::AutoBackup { backup_dir, retention_days } => {
            let summary = advanced.automated_backup(
                backup_dir.to_str().unwrap(),
                retention_days
            ).await?;
            
            println!("{} Backup completed at {}", "✓".green(), summary.backup_time);
            println!("Old backups deleted: {}", summary.old_backups_deleted);
        }
    }
    
    Ok(())
}