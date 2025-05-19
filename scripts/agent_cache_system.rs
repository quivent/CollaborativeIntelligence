use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

// Agent cache entry structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCacheEntry {
    pub agent_name: String,
    pub directory: String, 
    pub initializer: String,
    pub timestamp: SystemTime,
    pub intent: String,
    pub status: AgentStatus,
    #[serde(skip)]
    pub is_modified: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentStatus {
    Active,
    Inactive,
}

// Memory-first cache manager
pub struct AgentCacheManager {
    memory_cache: HashMap<String, AgentCacheEntry>,
    last_persistence: SystemTime,
    persistence_interval: Duration,
    cache_dir: PathBuf,
    modified: bool,
}

impl AgentCacheManager {
    // Create a new cache manager
    pub fn new(cache_dir: &Path, persistence_interval_seconds: u64) -> Result<Self, Box<dyn Error>> {
        let cache_dir = PathBuf::from(cache_dir);
        if !cache_dir.exists() {
            create_dir_all(&cache_dir)?;
        }
        
        let mut manager = AgentCacheManager {
            memory_cache: HashMap::new(),
            last_persistence: SystemTime::now(),
            persistence_interval: Duration::from_secs(persistence_interval_seconds),
            cache_dir,
            modified: false,
        };
        
        // Load existing cache entries if any
        manager.load_existing_entries()?;
        
        Ok(manager)
    }
    
    // Load existing cache files into memory
    fn load_existing_entries(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.cache_dir.exists() {
            return Ok(());
        }
        
        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                let content = fs::read_to_string(&path)?;
                if let Some(cache_entry) = self.parse_cache_file(&content) {
                    let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                    self.memory_cache.insert(file_name, cache_entry);
                }
            }
        }
        
        Ok(())
    }
    
    // Parse a cache file into an AgentCacheEntry
    fn parse_cache_file(&self, content: &str) -> Option<AgentCacheEntry> {
        // Simple parsing for markdown format
        let mut agent_name = String::new();
        let mut directory = String::new();
        let mut initializer = String::new();
        let mut timestamp = SystemTime::now();
        let mut intent = String::new();
        let mut status = AgentStatus::Inactive;
        let mut in_intent_section = false;
        
        for line in content.lines() {
            if line.starts_with("- **Agent:**") {
                agent_name = line.trim_start_matches("- **Agent:**").trim().to_string();
            } else if line.starts_with("- **Working Directory:**") {
                directory = line.trim_start_matches("- **Working Directory:**").trim().to_string();
            } else if line.starts_with("- **Initializer:**") {
                initializer = line.trim_start_matches("- **Initializer:**").trim().to_string();
            } else if line.starts_with("- **Timestamp:**") {
                // Timestamp parsing is simplified - in a real impl, would use proper datetime parsing
                // Just keeping current timestamp for this example
            } else if line.starts_with("## Intent") {
                in_intent_section = true;
                continue;
            } else if line.starts_with("## Status") {
                in_intent_section = false;
            } else if in_intent_section && !line.is_empty() {
                if !intent.is_empty() {
                    intent.push('\n');
                }
                intent.push_str(line);
            } else if line.trim() == "Active" {
                status = AgentStatus::Active;
            } else if line.trim() == "Inactive" {
                status = AgentStatus::Inactive;
            }
        }
        
        if agent_name.is_empty() || directory.is_empty() || initializer.is_empty() {
            return None;
        }
        
        Some(AgentCacheEntry {
            agent_name,
            directory,
            initializer,
            timestamp,
            intent,
            status,
            is_modified: false,
        })
    }
    
    // Create a new agent cache entry
    pub fn create_agent_cache(
        &mut self,
        agent_name: &str,
        directory: &str,
        initializer: &str, 
        intent: &str
    ) -> Result<String, Box<dyn Error>> {
        // Update terminal window title immediately
        self.update_terminal_title(agent_name)?;
        
        // Create entry ID
        let entry_id = format!(
            "{}_{}_{}_{}",
            self.sanitize(agent_name),
            self.sanitize(directory),
            self.sanitize(initializer), 
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
        );
        
        // Create entry in memory
        let entry = AgentCacheEntry {
            agent_name: agent_name.to_string(),
            directory: directory.to_string(),
            initializer: initializer.to_string(),
            timestamp: SystemTime::now(),
            intent: intent.to_string(),
            status: AgentStatus::Active,
            is_modified: true,
        };
        
        // Store in memory cache
        self.memory_cache.insert(format!("{}.md", entry_id), entry);
        self.modified = true;
        
        // Maybe persist changes based on interval
        self.maybe_persist()?;
        
        Ok(entry_id)
    }
    
    // Sanitize string for filename
    fn sanitize(&self, s: &str) -> String {
        s.replace(' ', "_")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == '.')
            .collect()
    }
    
    // Conditionally persist if interval has elapsed
    fn maybe_persist(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.modified {
            return Ok(());
        }
        
        let now = SystemTime::now();
        if now.duration_since(self.last_persistence)? >= self.persistence_interval {
            self.persist_to_disk()?;
            self.last_persistence = now;
            self.modified = false;
        }
        
        Ok(())
    }
    
    // Force persistence to disk immediately
    pub fn force_persist(&mut self) -> Result<(), Box<dyn Error>> {
        if self.modified {
            self.persist_to_disk()?;
            self.last_persistence = SystemTime::now();
            self.modified = false;
        }
        Ok(())
    }
    
    // Helper to persist entries to disk
    fn persist_to_disk(&mut self) -> Result<(), Box<dyn Error>> {
        for (id, entry) in &mut self.memory_cache {
            if entry.is_modified {
                let path = self.cache_dir.join(id);
                let content = self.format_entry_as_markdown(entry);
                fs::write(path, content)?;
                entry.is_modified = false;
            }
        }
        Ok(())
    }
    
    // Format entry as markdown
    fn format_entry_as_markdown(&self, entry: &AgentCacheEntry) -> String {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        format!(
            r#"# Agent Activation Cache

## Metadata
- **Agent:** {}
- **Working Directory:** {}
- **Initializer:** {}
- **Timestamp:** {}

## Intent
{}

## Status
{}

This file was automatically generated by the Agent Cache System and should be immediately integrated into the agent's short-term memory upon activation.
"#,
            entry.agent_name,
            entry.directory,
            entry.initializer,
            timestamp,
            entry.intent,
            if entry.status == AgentStatus::Active { "Active" } else { "Inactive" }
        )
    }
    
    // Update the terminal window title
    fn update_terminal_title(&self, agent_name: &str) -> Result<(), Box<dyn Error>> {
        // For macOS Terminal and iTerm2
        print!("\x1B]0;Agent: {}\x07", agent_name);
        
        // For Linux terminals
        print!("\x1B]2;Agent: {}\x07", agent_name);
        
        // For Windows Command Prompt/PowerShell
        #[cfg(target_os = "windows")]
        {
            let title_cmd = format!("title Agent: {}", agent_name);
            Command::new("cmd")
                .args(&["/C", &title_cmd])
                .output()?;
        }
        
        Ok(())
    }
    
    // Get entry for agent
    pub fn get_entry_for_agent(&self, agent_name: &str) -> Option<AgentCacheEntry> {
        let mut latest_entry: Option<&AgentCacheEntry> = None;
        let mut latest_time = UNIX_EPOCH;
        
        for entry in self.memory_cache.values() {
            if entry.agent_name == agent_name && entry.status == AgentStatus::Active {
                if latest_entry.is_none() || entry.timestamp > latest_time {
                    latest_entry = Some(entry);
                    latest_time = entry.timestamp;
                }
            }
        }
        
        latest_entry.cloned()
    }
    
    // Deactivate agent
    pub fn deactivate_agent(&mut self, agent_name: &str) -> Result<(), Box<dyn Error>> {
        let mut modified = false;
        
        for entry in self.memory_cache.values_mut() {
            if entry.agent_name == agent_name && entry.status == AgentStatus::Active {
                entry.status = AgentStatus::Inactive;
                entry.is_modified = true;
                modified = true;
            }
        }
        
        if modified {
            self.modified = true;
            self.force_persist()?;
        }
        
        Ok(())
    }
    
    // Get all active agents
    pub fn get_active_agents(&self) -> Vec<String> {
        let mut active_agents = Vec::new();
        
        for entry in self.memory_cache.values() {
            if entry.status == AgentStatus::Active && !active_agents.contains(&entry.agent_name) {
                active_agents.push(entry.agent_name.clone());
            }
        }
        
        active_agents
    }
    
    // Clean up old cache files
    pub fn cleanup_old_cache(&mut self, days: u64) -> Result<usize, Box<dyn Error>> {
        let seconds = days * 24 * 60 * 60;
        let now = SystemTime::now();
        let mut removed_count = 0;
        let mut to_remove = Vec::new();
        
        for (id, entry) in &self.memory_cache {
            if entry.status == AgentStatus::Inactive {
                if let Ok(duration) = now.duration_since(entry.timestamp) {
                    if duration.as_secs() > seconds {
                        to_remove.push(id.clone());
                    }
                }
            }
        }
        
        for id in &to_remove {
            self.memory_cache.remove(id);
            let path = self.cache_dir.join(id);
            if path.exists() {
                fs::remove_file(path)?;
                removed_count += 1;
            }
        }
        
        Ok(removed_count)
    }
}

// Main entry point for testing
pub fn main() -> Result<(), Box<dyn Error>> {
    let cache_dir = Path::new("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CACHE/SESSIONS");
    let mut cache_manager = AgentCacheManager::new(cache_dir, 60)?; // 60 second persistence interval
    
    // Create a test entry
    let entry_id = cache_manager.create_agent_cache(
        "TestAgent",
        "/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence",
        "Manual Test",
        "Testing the Agent Cache System"
    )?;
    
    println!("Created agent cache entry: {}", entry_id);
    
    // Force persistence
    cache_manager.force_persist()?;
    
    // Get active agents
    let active_agents = cache_manager.get_active_agents();
    println!("Active agents: {:?}", active_agents);
    
    // Get entry for agent
    if let Some(entry) = cache_manager.get_entry_for_agent("TestAgent") {
        println!("Found entry for TestAgent: {:?}", entry);
    }
    
    // Deactivate agent
    cache_manager.deactivate_agent("TestAgent")?;
    
    println!("TestAgent deactivated");
    
    Ok(())
}