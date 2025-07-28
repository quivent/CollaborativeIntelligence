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
        // Initialize internal database schema
        {
            let conn = self.internal_conn.lock().await;
            conn.execute(
                "CREATE TABLE IF NOT EXISTS agents (
                    name TEXT PRIMARY KEY,
                    agent_type TEXT NOT NULL,
                    status TEXT NOT NULL DEFAULT 'active',
                    activation_count INTEGER NOT NULL DEFAULT 0,
                    created_at INTEGER NOT NULL,
                    last_active INTEGER,
                    memory_version INTEGER NOT NULL DEFAULT 1,
                    memory_path TEXT
                )",
                [],
            )?;
            
            conn.execute(
                "CREATE TABLE IF NOT EXISTS learning_entries (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    agent_name TEXT NOT NULL,
                    timestamp INTEGER NOT NULL,
                    trigger_type TEXT NOT NULL,
                    importance REAL NOT NULL,
                    category TEXT,
                    content TEXT NOT NULL,
                    FOREIGN KEY (agent_name) REFERENCES agents (name)
                )",
                [],
            )?;
        }
        
        // Initialize projects database schema
        {
            let conn = self.projects_conn.lock().await;
            conn.execute(
                "CREATE TABLE IF NOT EXISTS projects (
                    name TEXT PRIMARY KEY,
                    project_type TEXT,
                    status TEXT NOT NULL DEFAULT 'discovered',
                    path TEXT NOT NULL,
                    last_accessed INTEGER
                )",
                [],
            )?;
        }
        
        Ok(())
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

pub struct CIDatabaseAPI {
    db: Database,
}

impl CIDatabaseAPI {
    pub fn new(db: Database) -> Self {
        CIDatabaseAPI { db }
    }
    
    pub async fn list_agents(&self) -> Result<Vec<Agent>> {
        let conn = self.db.internal_conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT name, agent_type, status, activation_count, created_at, last_active, memory_version, memory_path 
             FROM agents ORDER BY name"
        )?;
        
        let agent_iter = stmt.query_map([], |row| {
            Ok(Agent {
                name: row.get(0)?,
                agent_type: row.get(1)?,
                status: row.get(2)?,
                activation_count: row.get(3)?,
                created_at: DateTime::from_timestamp(row.get::<_, i64>(4)?, 0).unwrap_or_else(|| Utc::now()),
                last_active: row.get::<_, Option<i64>>(5)?.map(|ts| DateTime::from_timestamp(ts, 0).unwrap_or_else(|| Utc::now())),
                memory_version: row.get(6)?,
                memory_path: row.get(7)?,
            })
        })?;
        
        let mut agents = Vec::new();
        for agent in agent_iter {
            agents.push(agent?);
        }
        
        Ok(agents)
    }
    
    pub async fn get_agent(&self, name: &str) -> Result<Agent> {
        let conn = self.db.internal_conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT name, agent_type, status, activation_count, created_at, last_active, memory_version, memory_path 
             FROM agents WHERE name = ?1"
        )?;
        
        let agent = stmt.query_row([name], |row| {
            Ok(Agent {
                name: row.get(0)?,
                agent_type: row.get(1)?,
                status: row.get(2)?,
                activation_count: row.get(3)?,
                created_at: DateTime::from_timestamp(row.get::<_, i64>(4)?, 0).unwrap_or_else(|| Utc::now()),
                last_active: row.get::<_, Option<i64>>(5)?.map(|ts| DateTime::from_timestamp(ts, 0).unwrap_or_else(|| Utc::now())),
                memory_version: row.get(6)?,
                memory_path: row.get(7)?,
            })
        })?;
        
        Ok(agent)
    }
    
    pub async fn create_agent(&self, req: CreateAgentRequest) -> Result<Agent> {
        let now = Utc::now();
        let timestamp = now.timestamp();
        
        let conn = self.db.internal_conn.lock().await;
        conn.execute(
            "INSERT INTO agents (name, agent_type, status, activation_count, created_at, memory_path)
             VALUES (?1, ?2, 'active', 0, ?3, ?4)",
            [&req.name, &req.agent_type, &timestamp.to_string(), &req.memory_path.unwrap_or_default()]
        )?;
        
        Ok(Agent {
            name: req.name,
            agent_type: req.agent_type,
            status: "active".to_string(),
            activation_count: 0,
            created_at: now,
            last_active: None,
            memory_version: 1,
            memory_path: req.memory_path,
        })
    }
    
    pub async fn activate_agent(&self, name: &str) -> Result<()> {
        let conn = self.db.internal_conn.lock().await;
        let now = Utc::now().timestamp();
        
        conn.execute(
            "UPDATE agents SET status = 'active', last_active = ?1, activation_count = activation_count + 1 
             WHERE name = ?2",
            [&now.to_string(), name]
        )?;
        
        Ok(())
    }
    
    pub async fn deactivate_agent(&self, name: &str) -> Result<()> {
        let conn = self.db.internal_conn.lock().await;
        
        conn.execute(
            "UPDATE agents SET status = 'inactive' WHERE name = ?1",
            [name]
        )?;
        
        Ok(())
    }
    
    pub async fn register_project(&self, project: projects::Project) -> Result<()> {
        let conn = self.db.projects_conn.lock().await;
        let last_accessed = project.last_accessed.map(|dt| dt.timestamp()).unwrap_or(0);
        
        conn.execute(
            "INSERT OR REPLACE INTO projects (name, project_type, status, path, last_accessed)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            [
                &project.name,
                &project.project_type.unwrap_or_default(),
                &project.status,
                &project.path,
                &last_accessed.to_string(),
            ]
        )?;
        
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