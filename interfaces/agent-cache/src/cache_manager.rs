use std::collections::{HashMap, BTreeMap};
use std::fs::{self, create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde_json::{from_str, to_string_pretty, from_reader};

use crate::error::{AgentCacheError, Result};
use crate::models::{AgentCacheEntry, AgentStatus};
use crate::terminal;
use crate::security::SecurityEnforcer;

/// Database format for storing all cache entries in a single file
#[derive(serde::Serialize, serde::Deserialize)]
struct AgentCacheDatabase {
    /// Version of the database schema
    version: u8,
    /// Last time the database was modified
    last_modified: SystemTime,
    /// All cache entries
    entries: Vec<AgentCacheEntry>,
}

impl AgentCacheDatabase {
    /// Create a new empty database
    fn new() -> Self {
        AgentCacheDatabase {
            version: 1,
            last_modified: SystemTime::now(),
            entries: Vec::new(),
        }
    }
    
    /// Load database from file or create new if it doesn't exist
    fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            let file = File::open(path).map_err(AgentCacheError::IoError)?;
            let db: AgentCacheDatabase = from_reader(file)
                .map_err(|e| AgentCacheError::JsonError(e))?;
            Ok(db)
        } else {
            Ok(Self::new())
        }
    }
    
    /// Save database to file
    fn save(&self, path: &Path) -> Result<()> {
        let json = to_string_pretty(self).map_err(AgentCacheError::JsonError)?;
        let mut file = File::create(path).map_err(AgentCacheError::IoError)?;
        file.write_all(json.as_bytes()).map_err(AgentCacheError::IoError)
    }
}

/// Index for fast lookups in the agent cache database
struct AgentCacheIndex {
    /// Map from agent name to indices in the database
    agent_index: HashMap<String, Vec<usize>>,
    /// Map from (agent, directory) to indices in the database
    context_index: HashMap<(String, String), Vec<usize>>,
    /// Map from timestamp to index in the database (sorted)
    timestamp_index: BTreeMap<u64, usize>,
}

impl AgentCacheIndex {
    /// Build indices from database entries
    fn build(entries: &[AgentCacheEntry]) -> Self {
        let mut agent_index = HashMap::new();
        let mut context_index = HashMap::new();
        let mut timestamp_index = BTreeMap::new();
        
        for (i, entry) in entries.iter().enumerate() {
            // Agent index
            agent_index.entry(entry.agent_name.clone())
                .or_insert_with(Vec::new)
                .push(i);
            
            // Context index
            context_index.entry((entry.agent_name.clone(), entry.directory.clone()))
                .or_insert_with(Vec::new)
                .push(i);
            
            // Timestamp index
            if let Ok(timestamp) = entry.timestamp.duration_since(UNIX_EPOCH) {
                timestamp_index.insert(timestamp.as_secs(), i);
            }
        }
        
        AgentCacheIndex {
            agent_index,
            context_index,
            timestamp_index,
        }
    }
    
    /// Get latest entry for agent
    fn get_latest_for_agent<'a>(&self, agent_name: &str, entries: &'a [AgentCacheEntry]) -> Option<&'a AgentCacheEntry> {
        if let Some(indices) = self.agent_index.get(agent_name) {
            let mut latest_idx = None;
            let mut latest_timestamp = 0;
            
            for &idx in indices {
                if entries[idx].status == AgentStatus::Active {
                    if let Ok(timestamp) = entries[idx].timestamp.duration_since(UNIX_EPOCH) {
                        let timestamp = timestamp.as_secs();
                        if latest_idx.is_none() || timestamp > latest_timestamp {
                            latest_idx = Some(idx);
                            latest_timestamp = timestamp;
                        }
                    }
                }
            }
            
            if let Some(idx) = latest_idx {
                return Some(&entries[idx]);
            }
        }
        
        None
    }
    
    /// Get all active agents
    fn get_active_agents(&self, entries: &[AgentCacheEntry]) -> Vec<String> {
        let mut result = Vec::new();
        
        for (agent_name, indices) in &self.agent_index {
            for &idx in indices {
                if entries[idx].status == AgentStatus::Active {
                    result.push(agent_name.clone());
                    break;
                }
            }
        }
        
        result
    }
}

/// Manager for agent cache entries
pub struct AgentCacheManager {
    /// Directory to store cache files
    cache_dir: PathBuf,
    /// Path to the database file
    db_path: PathBuf,
    /// Database of all cache entries
    database: AgentCacheDatabase,
    /// Index for fast lookups
    index: AgentCacheIndex,
    /// When cache was last persisted to disk
    last_persistence: SystemTime,
    /// How often to persist cache to disk
    persistence_interval: Duration,
    /// Whether cache has been modified since last persistence
    modified: bool,
    /// Legacy mode - maintain backward compatibility with individual files
    legacy_mode: bool,
}

impl AgentCacheManager {
    /// Create a new cache manager
    ///
    /// # Arguments
    ///
    /// * `cache_dir` - Directory to store cache files
    /// * `persistence_interval_seconds` - How often to persist cache to disk (in seconds)
    ///
    /// # Returns
    ///
    /// Result containing the new cache manager or an error
    pub fn new(cache_dir: &Path, persistence_interval_seconds: u64) -> Result<Self> {
        let cache_dir = PathBuf::from(cache_dir);
        if !cache_dir.exists() {
            create_dir_all(&cache_dir).map_err(|e| {
                AgentCacheError::IoError(e)
            })?;
        }
        
        // Database file path
        let db_path = cache_dir.join("agent_cache_db.json");
        
        // Load or create database
        let database = AgentCacheDatabase::load(&db_path)?;
        
        // Build index
        let index = AgentCacheIndex::build(&database.entries);
        
        let mut manager = AgentCacheManager {
            cache_dir,
            db_path,
            database,
            index,
            last_persistence: SystemTime::now(),
            persistence_interval: Duration::from_secs(persistence_interval_seconds),
            modified: false,
            legacy_mode: true, // Maintain compatibility for now
        };
        
        // Load existing cache entries from individual files if the database is empty
        if manager.database.entries.is_empty() {
            manager.load_legacy_entries()?;
        }
        
        Ok(manager)
    }
    
    /// Load existing cache files into memory (legacy mode)
    fn load_legacy_entries(&mut self) -> Result<()> {
        if !self.cache_dir.exists() {
            return Ok(());
        }
        
        let mut entries = Vec::new();
        
        for entry in fs::read_dir(&self.cache_dir).map_err(AgentCacheError::IoError)? {
            let entry = entry.map_err(AgentCacheError::IoError)?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" {
                        // Load markdown files
                        let content = fs::read_to_string(&path).map_err(AgentCacheError::IoError)?;
                        if let Some(cache_entry) = self.parse_cache_file(&content) {
                            entries.push(cache_entry);
                        }
                    } else if ext == "json" && path != self.db_path {
                        // Load individual JSON files, but not the database file
                        let content = fs::read_to_string(&path).map_err(AgentCacheError::IoError)?;
                        match from_str::<AgentCacheEntry>(&content) {
                            Ok(cache_entry) => {
                                entries.push(cache_entry);
                            },
                            Err(_) => {
                                // Skip if not a valid cache entry (might be the database file or something else)
                            }
                        }
                    }
                }
            }
        }
        
        // Add entries to database
        self.database.entries.extend(entries);
        
        // Update index
        self.index = AgentCacheIndex::build(&self.database.entries);
        
        // Mark as modified to save consolidated database
        self.modified = true;
        self.force_persist()?;
        
        Ok(())
    }
    
    /// Parse a markdown cache file into an AgentCacheEntry
    fn parse_cache_file(&self, content: &str) -> Option<AgentCacheEntry> {
        // Simple parsing for markdown format
        let mut agent_name = String::new();
        let mut directory = String::new();
        let mut initializer = String::new();
        let timestamp = SystemTime::now();
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
    
    /// Create a new agent cache entry
    ///
    /// # Arguments
    ///
    /// * `agent_name` - Name of the agent
    /// * `directory` - Working directory
    /// * `initializer` - Who or what initialized the agent
    /// * `intent` - Intent or purpose of activating the agent
    ///
    /// # Returns
    ///
    /// Result containing the ID of the new entry or an error
    pub fn create_agent_cache(
        &mut self,
        agent_name: &str,
        directory: &str,
        initializer: &str, 
        intent: &str
    ) -> Result<String> {
        // CRITICAL: Enforce authentication before activation (AUTH_BYPASS_001 fix)
        let mut security_enforcer = SecurityEnforcer::new();
        
        println!("{}", "🔒 SECURITY ENFORCEMENT: Validating agent authentication...".to_string().as_str());
        match security_enforcer.pre_operation_check(agent_name) {
            Ok(true) => {
                println!("{}", "✅ Security check passed - User accepted risk for unverified agent");
            },
            Ok(false) => {
                return Err(AgentCacheError::SecurityError(crate::security::SecurityError::OperationBlocked(
                    format!("Agent activation blocked: {}", agent_name)
                )));
            },
            Err(e) => {
                return Err(AgentCacheError::SecurityError(e));
            }
        }
        // Update terminal window title immediately
        terminal::update_terminal_title(agent_name)?;
        
        // Create entry ID
        let entry_id = format!(
            "{}_{}_{}_{}",
            self.sanitize(agent_name),
            self.sanitize(directory),
            self.sanitize(initializer), 
            SystemTime::now().duration_since(UNIX_EPOCH)
                .map_err(AgentCacheError::TimeError)?.as_secs()
        );
        
        // Create entry
        let mut entry = AgentCacheEntry::new(agent_name, directory, initializer, intent);
        entry.is_modified = true;
        
        // Add entry to database
        self.database.entries.push(entry);
        self.database.last_modified = SystemTime::now();
        
        // Update index
        self.index = AgentCacheIndex::build(&self.database.entries);
        
        // Mark as modified
        self.modified = true;
        
        // Maybe persist changes based on interval
        self.maybe_persist()?;
        
        Ok(entry_id)
    }
    
    /// Sanitize string for filename
    fn sanitize(&self, s: &str) -> String {
        s.replace(' ', "_")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == '.')
            .collect()
    }
    
    /// Conditionally persist if interval has elapsed
    fn maybe_persist(&mut self) -> Result<()> {
        if !self.modified {
            return Ok(());
        }
        
        let now = SystemTime::now();
        if now.duration_since(self.last_persistence)
            .map_err(AgentCacheError::TimeError)? >= self.persistence_interval {
            self.persist_to_disk()?;
            self.last_persistence = now;
            self.modified = false;
        }
        
        Ok(())
    }
    
    /// Force persistence to disk immediately
    pub fn force_persist(&mut self) -> Result<()> {
        if self.modified {
            self.persist_to_disk()?;
            self.last_persistence = SystemTime::now();
            self.modified = false;
        }
        Ok(())
    }
    
    /// Helper to persist entries to disk
    fn persist_to_disk(&mut self) -> Result<()> {
        // Save database file
        self.database.save(&self.db_path)?;
        
        // In legacy mode, also save individual files for backward compatibility
        if self.legacy_mode {
            // Collect indices and create a copy of entries that need to be persisted
            let mut entries_to_update = Vec::new();
            
            for (i, entry) in self.database.entries.iter().enumerate() {
                if entry.is_modified {
                    entries_to_update.push((i, entry.clone()));
                }
            }
            
            // Process each entry that needs to be persisted
            for (i, entry) in entries_to_update {
                // Generate filename
                let filename = format!(
                    "{}_{}_{}_{}",
                    self.sanitize(&entry.agent_name),
                    self.sanitize(&entry.directory),
                    self.sanitize(&entry.initializer),
                    entry.timestamp.duration_since(UNIX_EPOCH)
                        .map_err(AgentCacheError::TimeError)?
                        .as_secs()
                );
                
                // Save markdown file
                let md_path = self.cache_dir.join(format!("{}.md", filename));
                let content = entry.to_markdown();
                let mut file = File::create(&md_path).map_err(AgentCacheError::IoError)?;
                file.write_all(content.as_bytes()).map_err(AgentCacheError::IoError)?;
                
                // Save JSON file
                let json_path = self.cache_dir.join(format!("{}.json", filename));
                let json_content = to_string_pretty(&entry)
                    .map_err(AgentCacheError::JsonError)?;
                let mut json_file = File::create(&json_path).map_err(AgentCacheError::IoError)?;
                json_file.write_all(json_content.as_bytes()).map_err(AgentCacheError::IoError)?;
                
                // Reset modified flag in the database
                self.database.entries[i].is_modified = false;
            }
        }
        
        Ok(())
    }
    
    /// Get entry for agent
    ///
    /// # Arguments
    ///
    /// * `agent_name` - Name of the agent
    ///
    /// # Returns
    ///
    /// Option containing the entry if found
    pub fn get_entry_for_agent(&self, agent_name: &str) -> Option<AgentCacheEntry> {
        self.index.get_latest_for_agent(agent_name, &self.database.entries).cloned()
    }
    
    /// Deactivate agent
    ///
    /// # Arguments
    ///
    /// * `agent_name` - Name of the agent to deactivate
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub fn deactivate_agent(&mut self, agent_name: &str) -> Result<()> {
        let mut modified = false;
        
        // Find all entries for this agent and deactivate them
        if let Some(indices) = self.index.agent_index.get(agent_name).cloned() {
            for idx in indices {
                if self.database.entries[idx].status == AgentStatus::Active {
                    self.database.entries[idx].status = AgentStatus::Inactive;
                    self.database.entries[idx].is_modified = true;
                    modified = true;
                }
            }
        }
        
        if modified {
            self.modified = true;
            self.database.last_modified = SystemTime::now();
            self.force_persist()?;
            
            // Reset terminal title
            terminal::reset_terminal_title()?;
        } else {
            return Err(AgentCacheError::AgentNotFound(agent_name.to_string()));
        }
        
        Ok(())
    }
    
    /// Get all active agents
    ///
    /// # Returns
    ///
    /// Vector of active agent names
    pub fn get_active_agents(&self) -> Vec<String> {
        self.index.get_active_agents(&self.database.entries)
    }
    
    /// Clean up old cache files
    ///
    /// # Arguments
    ///
    /// * `days` - Remove files older than this many days
    ///
    /// # Returns
    ///
    /// Result containing the number of files removed or an error
    pub fn cleanup_old_cache(&mut self, days: u64) -> Result<usize> {
        let seconds = days * 24 * 60 * 60;
        let now = SystemTime::now();
        let mut removed_count = 0;
        
        // Keep track of indices to remove
        let mut indices_to_remove = Vec::new();
        
        // Find old inactive entries
        for (i, entry) in self.database.entries.iter().enumerate() {
            if entry.status == AgentStatus::Inactive {
                if let Ok(duration) = now.duration_since(entry.timestamp) {
                    if duration.as_secs() > seconds {
                        indices_to_remove.push(i);
                    }
                }
            }
        }
        
        // Remove entries from database (starting from the back to avoid index issues)
        indices_to_remove.sort_by(|a, b| b.cmp(a));
        for idx in indices_to_remove {
            self.database.entries.remove(idx);
            removed_count += 1;
        }
        
        if removed_count > 0 {
            // Rebuild index
            self.index = AgentCacheIndex::build(&self.database.entries);
            
            // Mark as modified
            self.modified = true;
            self.database.last_modified = SystemTime::now();
            self.force_persist()?;
            
            // In legacy mode, also clean up old files
            if self.legacy_mode {
                self.cleanup_legacy_files(days)?;
            }
        }
        
        Ok(removed_count)
    }
    
    /// Clean up old individual files (legacy mode)
    fn cleanup_legacy_files(&self, days: u64) -> Result<usize> {
        let seconds = days * 24 * 60 * 60;
        let now = SystemTime::now();
        let mut removed_count = 0;
        
        for entry in fs::read_dir(&self.cache_dir).map_err(AgentCacheError::IoError)? {
            let entry = entry.map_err(AgentCacheError::IoError)?;
            let path = entry.path();
            
            if path.is_file() && path != self.db_path {
                if let Ok(metadata) = fs::metadata(&path) {
                    if let Ok(modified_time) = metadata.modified() {
                        if let Ok(duration) = now.duration_since(modified_time) {
                            if duration.as_secs() > seconds {
                                fs::remove_file(&path).map_err(AgentCacheError::IoError)?;
                                removed_count += 1;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(removed_count)
    }
    
    /// Disable legacy mode (stop maintaining individual files)
    pub fn disable_legacy_mode(&mut self) {
        self.legacy_mode = false;
    }
}