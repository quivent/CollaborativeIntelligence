// memory_schema.rs - Schema definitions for the memory optimization database
//
// This file defines the database schema for the optimized memory storage system.
// It includes both the internal Rust struct representations and the serialization
// formats for JSON storage.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Type alias for timestamps (milliseconds since UNIX epoch)
pub type Timestamp = u64;

/// Represents a memory segment (part of an agent's memory)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySegment {
    /// Name of the segment (e.g., "MEMORY", "ContinuousLearning", "README")
    pub name: String,
    
    /// Content of the memory segment
    pub content: String,
    
    /// When the segment was last updated
    pub updated_at: Timestamp,
    
    /// Checksum of the content (for integrity validation)
    pub checksum: String,
    
    /// Optional metadata associated with this segment
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl MemorySegment {
    /// Create a new memory segment
    pub fn new(name: String, content: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as Timestamp;
        
        // Calculate checksum (SHA-256)
        let checksum = calculate_checksum(&content);
        
        Self {
            name,
            content,
            updated_at: now,
            checksum,
            metadata: HashMap::new(),
        }
    }
    
    /// Add metadata to segment
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

/// Represents an agent cache entry in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCacheEntry {
    /// Name of the agent
    pub name: String,
    
    /// Path to the agent's directory
    pub directory: PathBuf,
    
    /// When the agent cache was first created
    pub created_at: Timestamp,
    
    /// When the agent cache was last accessed
    pub accessed_at: Timestamp,
    
    /// Who/what created this cache entry
    pub initializer: String,
    
    /// Why this cache entry was created
    pub intent: String,
    
    /// Memory segments for this agent
    #[serde(default)]
    pub segments: HashMap<String, MemorySegment>,
    
    /// Additional metadata associated with this agent
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl AgentCacheEntry {
    /// Create a new agent cache entry
    pub fn new(name: String, directory: PathBuf, initializer: String, intent: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as Timestamp;
            
        Self {
            name,
            directory,
            created_at: now,
            accessed_at: now,
            initializer,
            intent,
            segments: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// Add a memory segment to this agent
    pub fn add_segment(&mut self, segment: MemorySegment) {
        self.segments.insert(segment.name.clone(), segment);
    }
    
    /// Update the accessed_at timestamp
    pub fn update_accessed(&mut self) {
        self.accessed_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as Timestamp;
    }
    
    /// Add metadata to agent
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

/// The main database structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDatabase {
    /// Schema version for compatibility checks
    pub schema_version: String,
    
    /// When the database was created
    pub created_at: Timestamp,
    
    /// When the database was last updated
    pub updated_at: Timestamp,
    
    /// Agent cache entries
    pub agents: HashMap<String, AgentCacheEntry>,
    
    /// Global metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl MemoryDatabase {
    /// Create a new, empty database
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as Timestamp;
            
        Self {
            schema_version: "1.0.0".to_string(),
            created_at: now,
            updated_at: now,
            agents: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// Add an agent to the database
    pub fn add_agent(&mut self, agent: AgentCacheEntry) {
        self.agents.insert(agent.name.clone(), agent);
        self.update_timestamp();
    }
    
    /// Get an agent by name
    pub fn get_agent(&mut self, name: &str) -> Option<&mut AgentCacheEntry> {
        if let Some(agent) = self.agents.get_mut(name) {
            agent.update_accessed();
            self.update_timestamp();
            Some(agent)
        } else {
            None
        }
    }
    
    /// Update the updated_at timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as Timestamp;
    }
    
    /// Add metadata to database
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

/// Calculate a checksum for content integrity
fn calculate_checksum(content: &str) -> String {
    use sha2::{Sha256, Digest};
    
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();
    
    format!("{:x}", result)
}

/// Load a database from disk
pub fn load_database(path: &PathBuf) -> Result<MemoryDatabase, std::io::Error> {
    if !path.exists() {
        return Ok(MemoryDatabase::new());
    }
    
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let db = serde_json::from_reader(reader)?;
    
    Ok(db)
}

/// Save a database to disk
pub fn save_database(db: &MemoryDatabase, path: &PathBuf) -> Result<(), std::io::Error> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Save to a temporary file first
    let temp_path = path.with_extension("tmp");
    let file = std::fs::File::create(&temp_path)?;
    let writer = std::io::BufWriter::new(file);
    
    serde_json::to_writer_pretty(writer, db)?;
    
    // Rename the temporary file to the actual file (atomic operation)
    std::fs::rename(temp_path, path)?;
    
    Ok(())
}

/// Calculate the size of the database
pub fn calculate_database_size(db: &MemoryDatabase) -> usize {
    // Serialize to JSON and count bytes
    match serde_json::to_string(db) {
        Ok(json) => json.len(),
        Err(_) => 0,
    }
}

/// Create an index of agents and their memory segments
pub fn create_index(db: &MemoryDatabase) -> HashMap<String, Vec<String>> {
    let mut index = HashMap::new();
    
    for (agent_name, agent) in &db.agents {
        let segment_names: Vec<String> = agent.segments.keys().cloned().collect();
        index.insert(agent_name.clone(), segment_names);
    }
    
    index
}