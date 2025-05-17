pub mod agent;
pub mod entities;
pub mod populate;
// pub mod learning;
// pub mod session;
// pub mod knowledge;
// pub mod project;
// pub mod metrics;
// pub mod admin;
// pub mod query;
pub mod stubs;

// Re-export stubs as the actual modules
pub use stubs::learning;
pub use stubs::session;
pub use stubs::knowledge;
pub use stubs::project;
pub use stubs::metrics;
pub use stubs::admin;
pub use stubs::query;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum AgentCommands {
    /// List all agents
    List {
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,
        
        /// Filter by type
        #[arg(short = 't', long)]
        agent_type: Option<String>,
    },
    
    /// Show agent details
    Show {
        /// Agent name
        name: String,
    },
    
    /// Create new agent
    Create {
        /// Agent name
        name: String,
        
        /// Agent type
        #[arg(short = 't', long)]
        agent_type: String,
        
        /// Memory path
        #[arg(short, long)]
        memory_path: Option<String>,
    },
    
    /// Update agent
    Update {
        /// Agent name
        name: String,
        
        /// New status
        #[arg(short, long)]
        status: Option<String>,
        
        /// New memory version
        #[arg(short, long)]
        memory_version: Option<i32>,
    },
    
    /// Delete agent
    Delete {
        /// Agent name
        name: String,
        
        /// Force deletion without confirmation
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum LearningCommands {
    /// List learning entries
    List {
        /// Filter by agent
        #[arg(short, long)]
        agent: Option<String>,
        
        /// Limit results
        #[arg(short, long, default_value = "50")]
        limit: i32,
        
        /// Filter by date (YYYY-MM-DD)
        #[arg(short, long)]
        since: Option<String>,
    },
    
    /// Add learning entry
    Add {
        /// Agent name
        agent: String,
        
        /// Trigger type
        trigger: String,
        
        /// Learning content
        content: String,
        
        /// Importance (0.0-1.0)
        #[arg(short, long)]
        importance: Option<f64>,
        
        /// Category
        #[arg(short, long)]
        category: Option<String>,
    },
    
    /// Consolidate learnings
    Consolidate {
        /// Agent name
        agent: String,
        
        /// Minimum importance threshold
        #[arg(short, long, default_value = "0.5")]
        threshold: f64,
    },
}

#[derive(Subcommand)]
pub enum SessionCommands {
    /// List sessions
    List {
        /// Filter by agent
        #[arg(short, long)]
        agent: Option<String>,
        
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,
        
        /// Show only active sessions
        #[arg(long)]
        active: bool,
    },
    
    /// Create session
    Create {
        /// Agent name
        agent: String,
        
        /// Session type
        session_type: String,
    },
    
    /// Close session
    Close {
        /// Session ID
        id: i64,
        
        /// Outcomes (JSON)
        #[arg(short, long)]
        outcomes: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum KnowledgeCommands {
    /// Search knowledge base
    Search {
        /// Search query
        query: String,
        
        /// Filter by agent
        #[arg(short, long)]
        agent: Option<String>,
        
        /// Filter by type
        #[arg(short = 't', long)]
        knowledge_type: Option<String>,
    },
    
    /// Add knowledge entry
    Add {
        /// Agent name
        agent: String,
        
        /// Knowledge type
        knowledge_type: String,
        
        /// Content
        content: String,
        
        /// Category
        #[arg(short, long)]
        category: Option<String>,
        
        /// Confidence (0.0-1.0)
        #[arg(short = 'C', long)]
        confidence: Option<f64>,
    },
}

#[derive(Subcommand)]
pub enum ProjectCommands {
    /// List projects
    List {
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,
        
        /// Sort by field
        #[arg(long, default_value = "last_accessed")]
        sort: String,
    },
    
    /// Show project details
    Show {
        /// Project name
        name: String,
    },
    
    /// Register project
    Register {
        /// Project name
        name: String,
        
        /// Project path
        path: String,
        
        /// Project type
        #[arg(short = 't', long)]
        project_type: Option<String>,
    },
    
    /// Update project
    Update {
        /// Project name
        name: String,
        
        /// New status
        #[arg(short, long)]
        status: Option<String>,
        
        /// New CI version
        #[arg(short, long)]
        ci_version: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum MetricsCommands {
    /// Show metrics dashboard
    Dashboard {
        /// Time period in days
        #[arg(short, long, default_value = "30")]
        days: i64,
    },
    
    /// List metrics
    List {
        /// Project name
        project: String,
        
        /// Metric type
        metric_type: String,
        
        /// Limit results
        #[arg(short, long, default_value = "100")]
        limit: i32,
    },
    
    /// Add metric
    Add {
        /// Project name
        project: String,
        
        /// Metric type
        metric_type: String,
        
        /// Value
        value: f64,
    },
    
    /// Generate report
    Report {
        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Report type
        #[arg(short = 't', long, default_value = "summary")]
        report_type: String,
    },
}

#[derive(Subcommand)]
pub enum AdminCommands {
    /// Initialize database
    Init {
        /// Force reinitialization
        #[arg(short, long)]
        force: bool,
    },
    
    /// Optimize database
    Optimize,
    
    /// Backup database
    Backup {
        /// Backup directory
        #[arg(short, long)]
        output: PathBuf,
    },
    
    /// Restore database
    Restore {
        /// Backup file
        #[arg(short, long)]
        input: PathBuf,
    },
    
    /// Health check
    Health,
    
    /// Database statistics
    Stats,
    
    /// Export data
    Export {
        /// Output file
        #[arg(short, long)]
        output: PathBuf,
        
        /// Include sensitive data
        #[arg(long)]
        include_sensitive: bool,
    },
    
    /// Import data
    Import {
        /// Input file
        #[arg(short, long)]
        input: PathBuf,
    },
}

use std::path::PathBuf;

#[derive(Subcommand)]
pub enum EntitiesCommands {
    /// List all entities (tables) in both databases
    List,
    
    /// Show attributes (columns) for a specific entity
    Attributes {
        /// Entity/table name
        entity_name: String,
    },
}

#[derive(Subcommand)]
pub enum PopulateCommands {
    /// Scan directory and populate database with discovered information
    Scan {
        /// Directory to scan (defaults to parent directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
        
        /// Preview changes without applying them
        #[arg(long)]
        dry_run: bool,
    },
}

#[cfg(test)]
mod tests;