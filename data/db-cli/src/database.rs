// Temporary mock database API until ci_database is fixed
use anyhow::Result;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type SqliteConnection = rusqlite::Connection;

#[derive(Clone)]
pub struct Database {
    pub internal_conn: Arc<Mutex<SqliteConnection>>,
    pub projects_conn: Arc<Mutex<SqliteConnection>>,
}

impl Database {
    pub async fn new<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let internal_path = path.as_ref().join("internal/ci_internal.db");
        let projects_path = path.as_ref().join("projects/ci_projects.db");
        
        // Create directories if they don't exist
        std::fs::create_dir_all(internal_path.parent().unwrap())?;
        std::fs::create_dir_all(projects_path.parent().unwrap())?;
        
        let internal_conn = SqliteConnection::open(&internal_path)?;
        let projects_conn = SqliteConnection::open(&projects_path)?;
        
        Ok(Self {
            internal_conn: Arc::new(Mutex::new(internal_conn)),
            projects_conn: Arc::new(Mutex::new(projects_conn)),
        })
    }
    
    pub fn internal(&self) -> &Self {
        self
    }
    
    pub fn projects(&self) -> &Self {
        self
    }
    
    pub async fn initialize(&self) -> Result<()> {
        Ok(())  // Schema initialization would go here
    }
    
    pub async fn optimize(&self) -> Result<()> {
        self.internal_conn.lock().await.execute("VACUUM", [])?;
        self.projects_conn.lock().await.execute("VACUUM", [])?;
        Ok(())
    }
    
    pub async fn backup(&self, _path: impl AsRef<std::path::Path>) -> Result<()> {
        Ok(())  // Backup implementation would go here
    }
}

pub struct CIDatabaseAPI;

impl CIDatabaseAPI {
    pub fn new(_db: Database) -> Self {
        CIDatabaseAPI
    }
    
    pub async fn list_agents(&self) -> Result<Vec<Agent>> {
        Ok(vec![])
    }
    
    pub async fn get_agent(&self, _name: &str) -> Result<Agent> {
        Ok(Agent {
            name: "Mock".to_string(),
            agent_type: "mock".to_string(),
            status: "active".to_string(),
            activation_count: 0,
            created_at: Utc::now(),
            last_active: None,
            memory_version: 1,
            memory_path: None,
        })
    }
    
    pub async fn create_agent(&self, _req: CreateAgentRequest) -> Result<Agent> {
        Ok(Agent {
            name: "Mock".to_string(),
            agent_type: "mock".to_string(),
            status: "active".to_string(),
            activation_count: 0,
            created_at: Utc::now(),
            last_active: None,
            memory_version: 1,
            memory_path: None,
        })
    }
    
    pub async fn activate_agent(&self, _name: &str) -> Result<()> {
        Ok(())
    }
    
    pub async fn deactivate_agent(&self, _name: &str) -> Result<()> {
        Ok(())
    }
}

// Mock types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub name: String,
    pub agent_type: String,
    pub status: String,
    pub activation_count: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub last_active: Option<DateTime<Utc>>,
    pub memory_version: i32,
    pub memory_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentRequest {
    pub name: String,
    pub agent_type: String,
    pub memory_path: Option<String>,
}

// Add other mock types as needed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEntry {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    pub trigger_type: String,
    pub importance: f64,
    pub category: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub project_type: Option<String>,
    pub status: String,
    pub path: String,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub last_accessed: Option<DateTime<Utc>>,
}

// Export mock implementations
pub mod internal {
    pub use super::CreateAgentRequest;
}

pub mod projects {
    pub use super::Project;
}