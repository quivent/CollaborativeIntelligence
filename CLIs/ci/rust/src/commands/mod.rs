//! Command structure and routing for CI CLI
//!
//! This module defines the command structure and routes commands to their handlers.

use clap::{Parser, Subcommand};
use crate::config::Config;
use anyhow::Result;

pub mod init;
pub mod integrate;
pub mod status;
pub mod fix;
pub mod verify;
pub mod load;
pub mod agents;
pub mod map;

// Re-export command functions
pub use init::init;
pub use integrate::integrate;
pub use status::status;
pub use fix::fix;
pub use verify::verify;
pub use load::load;
pub use agents::agents;
pub use map::map;

/// CI CLI - Collaborative Intelligence Command Line Interface
#[derive(Parser)]
#[clap(name = "ci")]
#[clap(about = "Collaborative Intelligence CLI", long_about = None)]
#[clap(version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

/// Commands available in the CI CLI
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new project with CI
    Init {
        /// Name of the project to create
        #[clap(value_parser)]
        name: String,
        
        /// Comma-separated list of agents to include
        #[clap(long, value_parser, default_value = "Athena,ProjectArchitect")]
        agents: String,
        
        /// Integration type: embedded, sibling, symlink
        #[clap(long, value_parser, default_value = "embedded")]
        integration: String,
        
        /// Disable fast activation
        #[clap(long, action)]
        no_fast: bool,
        
        /// Path to CI repository (optional)
        #[clap(long, value_parser)]
        ci_path: Option<String>,
    },
    
    /// Integrate CI into an existing project
    Integrate {
        /// Path to the project (default: current directory)
        #[clap(value_parser)]
        path: Option<String>,
        
        /// Comma-separated list of agents to include
        #[clap(long, value_parser, default_value = "Athena,ProjectArchitect")]
        agents: String,
        
        /// Integration type: embedded, sibling, symlink
        #[clap(long, value_parser, default_value = "embedded")]
        integration: String,
        
        /// Disable fast activation
        #[clap(long, action)]
        no_fast: bool,
        
        /// Path to CI repository (optional)
        #[clap(long, value_parser)]
        ci_path: Option<String>,
    },
    
    /// Check CI integration status
    Status {
        /// Path to the project (default: current directory)
        #[clap(value_parser)]
        path: Option<String>,
    },
    
    /// Fix CI integration issues
    Fix {
        /// Path to the project (default: current directory)
        #[clap(value_parser)]
        path: Option<String>,
        
        /// Type of fix to apply
        #[clap(value_parser)]
        fix_type: Option<String>,
    },
    
    /// Verify CI integration is working
    Verify {
        /// Path to the project (default: current directory)
        #[clap(value_parser)]
        path: Option<String>,
    },
    
    /// Load agent memory
    Load {
        /// Name of the agent to load memory for
        #[clap(value_parser)]
        agent: String,
        
        /// Context to use for loading memory
        #[clap(long, value_parser)]
        context: Option<String>,
        
        /// Path to the memory file (optional)
        #[clap(long, value_parser)]
        path: Option<String>,
    },
    
    /// Display available Collaborative Intelligence agents
    Agents {
        /// Show detailed information for each agent
        #[clap(long, action)]
        detailed: bool,
        
        /// Filter agents by name or role
        #[clap(long, value_parser)]
        filter: Option<String>,
        
        /// Show specific agent information
        #[clap(value_parser)]
        agent_name: Option<String>,
    },
    
    /// Display visual map of CLI commands
    Map,
}