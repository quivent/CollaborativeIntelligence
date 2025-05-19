use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serde_json;
use md5;
use log::{debug, info, warn, error};
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::{channel, Receiver};
use std::thread;

// Import from other crate modules
use crate::memory_segment_parser::{MemorySegment, MemorySegmentParser};
use crate::memory_loader::MemoryLoader;
use crate::memory_updater::{MemoryUpdater, MemoryUpdate, UpdateType};

// Checksum entry for a memory file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecksumEntry {
    pub file_path: PathBuf,
    pub checksum: String,
    pub last_checked: SystemTime,
    pub segments_checksums: HashMap<String, String>, // segment_id -> segment_checksum
    pub metadata: HashMap<String, String>,
}

// Cache statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChecksumCacheStats {
    pub total_validations: u64,
    pub valid_files: u64,
    pub modified_files: u64,
    pub segment_changes: HashMap<String, u64>, // agent -> change count
    pub avg_validation_time_ms: u64,
    pub total_validation_time_ms: u64,
    pub last_full_validation: Option<SystemTime>,
}

// Configuration for checksum cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecksumCacheConfig {
    pub cache_dir: PathBuf,
    pub validation_interval_seconds: u64,
    pub enable_file_watcher: bool,
    pub file_watcher_debounce_ms: u64,
    pub live_update: bool,
    pub verbose_logging: bool,
}

impl Default for ChecksumCacheConfig {
    fn default() -> Self {
        ChecksumCacheConfig {
            cache_dir: PathBuf::from("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CACHE/CHECKSUMS"),
            validation_interval_seconds: 60,
            enable_file_watcher: true,
            file_watcher_debounce_ms: 500,
            live_update: true,
            verbose_logging: false,
        }
    }
}

// Memory checksum cache implementation
pub struct ChecksumCache {
    config: ChecksumCacheConfig,
    parser: MemorySegmentParser,
    // Use DashMap for thread-safe concurrent access
    checksums: DashMap<String, ChecksumEntry>,
    last_validation: HashMap<String, SystemTime>, // agent -> last validation time
    stats: Arc<RwLock<ChecksumCacheStats>>,
    memory_loader: Option<Arc<Mutex<MemoryLoader>>>,
    memory_updater: Option<Arc<Mutex<MemoryUpdater>>>,
    watcher_thread: Option<thread::JoinHandle<()>>,
    watcher_running: Arc<RwLock<bool>>,
}

impl ChecksumCache {
    // Create a new checksum cache
    pub fn new(config: ChecksumCacheConfig) -> Result<Self, Box<dyn Error>> {
        // Create cache directory if needed
        fs::create_dir_all(&config.cache_dir)?;
        
        // Initialize parser
        let mut parser = MemorySegmentParser::new();
        
        // Register common domains
        parser.register_domain("identity", vec!["identity".to_string(), "purpose".to_string(), "core".to_string()]);
        parser.register_domain("learning", vec!["learn".to_string(), "knowledge".to_string(), "education".to_string()]);
        parser.register_domain("system", vec!["system".to_string(), "framework".to_string(), "architecture".to_string()]);
        parser.register_domain("memory", vec!["memory".to_string(), "recall".to_string(), "remember".to_string()]);
        parser.register_domain("technical", vec!["code".to_string(), "implementation".to_string(), "software".to_string()]);
        
        let mut cache = ChecksumCache {
            config,
            parser,
            checksums: DashMap::new(),
            last_validation: HashMap::new(),
            stats: Arc::new(RwLock::new(ChecksumCacheStats::default())),
            memory_loader: None,
            memory_updater: None,
            watcher_thread: None,
            watcher_running: Arc::new(RwLock::new(false)),
        };
        
        // Load existing checksums
        cache.load_checksums()?;
        
        Ok(cache)
    }
    
    // Connect memory loader and updater
    pub fn connect_memory_system(&mut self, loader: Arc<Mutex<MemoryLoader>>, updater: Arc<Mutex<MemoryUpdater>>) {
        self.memory_loader = Some(loader);
        self.memory_updater = Some(updater);
    }
    
    // Load checksums from cache directory
    fn load_checksums(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.config.cache_dir.exists() {
            fs::create_dir_all(&self.config.cache_dir)?;
            return Ok(());
        }
        
        for entry in fs::read_dir(&self.config.cache_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let content = fs::read_to_string(&path)?;
                match serde_json::from_str::<ChecksumEntry>(&content) {
                    Ok(entry) => {
                        let key = self.get_cache_key(&entry.file_path);
                        self.checksums.insert(key, entry);
                    },
                    Err(e) => {
                        warn!("Failed to parse checksum file {}: {}", path.display(), e);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    // Start file watcher for live updates
    pub fn start_file_watcher(&mut self, base_dir: &Path) -> Result<(), Box<dyn Error>> {
        if self.config.enable_file_watcher {
            // Signal that watcher is running
            *self.watcher_running.write().unwrap() = true;
            
            // Create a new channel to receive file change events
            let (tx, rx) = channel();
            
            // Create a watcher with default settings
            let mut watcher = watcher(tx, Duration::from_millis(self.config.file_watcher_debounce_ms))?;
            
            // Watch the base directory recursively
            watcher.watch(base_dir, RecursiveMode::Recursive)?;
            
            // Clone things we need in the thread
            let cache_dir = self.config.cache_dir.clone();
            let live_update = self.config.live_update;
            let watcher_running = self.watcher_running.clone();
            let stats = self.stats.clone();
            let memory_loader = self.memory_loader.clone();
            let memory_updater = self.memory_updater.clone();
            
            // Spawn a thread to handle file change events
            let handle = thread::spawn(move || {
                Self::watch_files(rx, cache_dir, live_update, watcher_running, stats, memory_loader, memory_updater);
            });
            
            self.watcher_thread = Some(handle);
            
            info!("File watcher started for {}", base_dir.display());
        }
        
        Ok(())
    }
    
    // Stop file watcher
    pub fn stop_file_watcher(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(_) = self.watcher_thread {
            // Signal the watcher to stop
            *self.watcher_running.write().unwrap() = false;
            
            // Wait for the thread to finish
            if let Some(handle) = self.watcher_thread.take() {
                handle.join().map_err(|_| "Failed to join watcher thread")?;
            }
            
            info!("File watcher stopped");
        }
        
        Ok(())
    }
    
    // File watcher thread function
    fn watch_files(
        rx: Receiver<DebouncedEvent>,
        cache_dir: PathBuf,
        live_update: bool,
        running: Arc<RwLock<bool>>,
        stats: Arc<RwLock<ChecksumCacheStats>>,
        memory_loader: Option<Arc<Mutex<MemoryLoader>>>,
        memory_updater: Option<Arc<Mutex<MemoryUpdater>>>
    ) {
        let mut parser = MemorySegmentParser::new();
        
        // Register common domains
        parser.register_domain("identity", vec!["identity".to_string(), "purpose".to_string(), "core".to_string()]);
        parser.register_domain("learning", vec!["learn".to_string(), "knowledge".to_string(), "education".to_string()]);
        parser.register_domain("system", vec!["system".to_string(), "framework".to_string(), "architecture".to_string()]);
        parser.register_domain("memory", vec!["memory".to_string(), "recall".to_string(), "remember".to_string()]);
        parser.register_domain("technical", vec!["code".to_string(), "implementation".to_string(), "software".to_string()]);
        
        // Create a local cache for checksums
        let mut checksums: HashMap<String, ChecksumEntry> = HashMap::new();
        
        // Load existing checksums into the local cache
        if cache_dir.exists() {
            for entry in fs::read_dir(&cache_dir).unwrap_or_else(|_| panic!("Failed to read cache directory")) {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Ok(checksum_entry) = serde_json::from_str::<ChecksumEntry>(&content) {
                                let key = Self::static_get_cache_key(&checksum_entry.file_path);
                                checksums.insert(key, checksum_entry);
                            }
                        }
                    }
                }
            }
        }
        
        // Process file events until signaled to stop
        while *running.read().unwrap() {
            match rx.recv_timeout(Duration::from_millis(100)) {
                Ok(event) => {
                    match event {
                        DebouncedEvent::Create(path) | 
                        DebouncedEvent::Write(path) | 
                        DebouncedEvent::Chmod(path) => {
                            // Only process Markdown files
                            if path.extension().map_or(false, |ext| ext == "md") {
                                debug!("File changed: {}", path.display());
                                
                                // Check if this is a memory file (in AGENTS directory)
                                if path.to_string_lossy().contains("AGENTS") && 
                                   (path.file_name().map_or(false, |name| name.to_string_lossy().contains("MEMORY.md")) || 
                                    path.file_name().map_or(false, |name| name.to_string_lossy().contains("ContinuousLearning.md")) ||
                                    path.file_name().map_or(false, |name| name.to_string_lossy().contains("README.md"))) {
                                    
                                    // Extract agent name from path
                                    let agent_name = Self::extract_agent_name(&path);
                                    if let Some(agent) = agent_name {
                                        info!("Memory file changed for agent {}: {}", agent, path.display());
                                        
                                        // Validate the file and get changes
                                        if let Ok(changes) = Self::static_validate_file(
                                            &path, 
                                            &mut checksums, 
                                            &mut parser, 
                                            &cache_dir,
                                            &stats
                                        ) {
                                            // If there are changes and live update is enabled
                                            if !changes.is_empty() && live_update {
                                                // Update stats
                                                if let Ok(mut stats_write) = stats.write() {
                                                    stats_write.modified_files += 1;
                                                    *stats_write.segment_changes.entry(agent.clone()).or_insert(0) += changes.len() as u64;
                                                }
                                                
                                                // Create updates
                                                if let Some(updater) = &memory_updater {
                                                    if let Ok(mut updater_lock) = updater.lock() {
                                                        for (segment, change_type) in changes {
                                                            let update_type = match change_type.as_str() {
                                                                "ADDED" => UpdateType::Add,
                                                                "MODIFIED" => UpdateType::Modify,
                                                                "REMOVED" => UpdateType::Remove,
                                                                _ => continue,
                                                            };
                                                            
                                                            let update = updater_lock.create_update(
                                                                update_type,
                                                                Some(segment.clone()),
                                                                0.6, // High importance but not immediate
                                                                "file_watcher",
                                                                Some(path.clone())
                                                            );
                                                            
                                                            // Queue the update
                                                            if let Err(e) = updater_lock.queue_update(&agent, update) {
                                                                error!("Failed to queue update for agent {}: {}", agent, e);
                                                            }
                                                        }
                                                        
                                                        // Process the updates
                                                        if let Err(e) = updater_lock.process_pending_updates(&agent) {
                                                            error!("Failed to process updates for agent {}: {}", agent, e);
                                                        } else {
                                                            info!("Successfully processed memory updates for agent {}", agent);
                                                        }
                                                    }
                                                }
                                                
                                                // Reload memory if needed
                                                if let Some(loader) = &memory_loader {
                                                    if let Ok(mut loader_lock) = loader.lock() {
                                                        info!("Reloading memory for agent {}", agent);
                                                        match loader_lock.load_agent_memory(&agent, None) {
                                                            Ok(_) => {
                                                                info!("Successfully reloaded memory for agent {}", agent);
                                                            },
                                                            Err(e) => {
                                                                error!("Failed to reload memory for agent {}: {}", agent, e);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        _ => {} // Ignore other events
                    }
                },
                Err(_) => {
                    // Timeout, check if we need to stop
                    if !*running.read().unwrap() {
                        break;
                    }
                }
            }
        }
        
        info!("File watcher thread exiting");
    }
    
    // Extract agent name from file path
    fn extract_agent_name(path: &Path) -> Option<String> {
        let path_str = path.to_string_lossy();
        let parts: Vec<&str> = path_str.split("AGENTS/").collect();
        if parts.len() < 2 {
            return None;
        }
        
        let agent_path = parts[1];
        let agent_parts: Vec<&str> = agent_path.split('/').collect();
        if agent_parts.is_empty() {
            return None;
        }
        
        Some(agent_parts[0].to_string())
    }
    
    // Validate a memory file
    pub fn validate_file(&mut self, file_path: &Path) -> Result<Vec<(MemorySegment, String)>, Box<dyn Error>> {
        // Check if file exists
        if !file_path.exists() {
            return Err(format!("File does not exist: {}", file_path.display()).into());
        }
        
        // Track validation time
        let start_time = Instant::now();
        
        // Get cache key
        let cache_key = self.get_cache_key(file_path);
        
        // Get current checksum
        let file_content = fs::read_to_string(file_path)?;
        let current_checksum = format!("{:x}", md5::compute(&file_content));
        
        // Check if file has been modified
        let mut changes = Vec::new();
        let entry_exists = self.checksums.contains_key(&cache_key);
        
        // Compare with cached checksum if available
        if entry_exists {
            let entry = self.checksums.get(&cache_key).unwrap();
            
            if entry.checksum == current_checksum {
                // File not modified, return empty changes
                
                // Update stats
                if let Ok(mut stats) = self.stats.write() {
                    stats.total_validations += 1;
                    stats.valid_files += 1;
                    stats.total_validation_time_ms += start_time.elapsed().as_millis() as u64;
                    stats.avg_validation_time_ms = stats.total_validation_time_ms / stats.total_validations;
                }
                
                return Ok(changes);
            }
            
            // File modified, process segments
            info!("File modified: {}", file_path.display());
            
            // Parse current segments
            let current_segments = self.parser.parse_file(file_path)?;
            
            // Create segment checksums map
            let mut current_segment_checksums = HashMap::new();
            for segment in &current_segments {
                let segment_checksum = format!("{:x}", md5::compute(&segment.content));
                current_segment_checksums.insert(segment.id.clone(), segment_checksum);
            }
            
            // Compare segments with cached checksums
            let old_segments_checksums = &entry.segments_checksums;
            
            // Find added and modified segments
            for segment in &current_segments {
                let segment_id = &segment.id;
                let segment_checksum = current_segment_checksums.get(segment_id).unwrap();
                
                if !old_segments_checksums.contains_key(segment_id) {
                    // New segment
                    changes.push((segment.clone(), "ADDED".to_string()));
                } else if old_segments_checksums.get(segment_id).unwrap() != segment_checksum {
                    // Modified segment
                    changes.push((segment.clone(), "MODIFIED".to_string()));
                }
            }
            
            // Find removed segments
            for old_segment_id in old_segments_checksums.keys() {
                if !current_segment_checksums.contains_key(old_segment_id) {
                    // Removed segment - need to create a placeholder segment
                    let removed_segment = MemorySegment {
                        id: old_segment_id.clone(),
                        memory_type: crate::memory_segment_parser::MemoryType::Semantic, // Default
                        tier: crate::memory_segment_parser::MemoryTier::Tier3, // Default
                        domains: vec!["unknown".to_string()],
                        content: "".to_string(),
                        last_accessed: SystemTime::now(),
                        access_count: 0,
                        source_file: file_path.file_name().unwrap().to_string_lossy().to_string(),
                        line_start: 0,
                        line_end: 0,
                        metadata: HashMap::new(),
                    };
                    
                    changes.push((removed_segment, "REMOVED".to_string()));
                }
            }
            
            // Update cache entry
            let new_entry = ChecksumEntry {
                file_path: file_path.to_path_buf(),
                checksum: current_checksum,
                last_checked: SystemTime::now(),
                segments_checksums: current_segment_checksums,
                metadata: HashMap::new(),
            };
            
            self.checksums.insert(cache_key.clone(), new_entry.clone());
            
            // Save to cache file
            let cache_file = self.config.cache_dir.join(format!("{}.json", cache_key.replace("/", "_")));
            let json = serde_json::to_string_pretty(&new_entry)?;
            fs::write(cache_file, json)?;
        } else {
            // File not in cache, add it
            info!("New file added to cache: {}", file_path.display());
            
            // Parse segments
            let segments = self.parser.parse_file(file_path)?;
            
            // All segments are considered new
            for segment in &segments {
                changes.push((segment.clone(), "ADDED".to_string()));
            }
            
            // Create segment checksums
            let mut segment_checksums = HashMap::new();
            for segment in &segments {
                let segment_checksum = format!("{:x}", md5::compute(&segment.content));
                segment_checksums.insert(segment.id.clone(), segment_checksum);
            }
            
            // Create cache entry
            let entry = ChecksumEntry {
                file_path: file_path.to_path_buf(),
                checksum: current_checksum,
                last_checked: SystemTime::now(),
                segments_checksums: segment_checksums,
                metadata: HashMap::new(),
            };
            
            self.checksums.insert(cache_key.clone(), entry.clone());
            
            // Save to cache file
            let cache_file = self.config.cache_dir.join(format!("{}.json", cache_key.replace("/", "_")));
            let json = serde_json::to_string_pretty(&entry)?;
            fs::write(cache_file, json)?;
        }
        
        // Update stats
        if let Ok(mut stats) = self.stats.write() {
            stats.total_validations += 1;
            stats.modified_files += 1;
            stats.total_validation_time_ms += start_time.elapsed().as_millis() as u64;
            stats.avg_validation_time_ms = stats.total_validation_time_ms / stats.total_validations;
        }
        
        Ok(changes)
    }
    
    // Static version of validate_file for use in threads
    fn static_validate_file(
        file_path: &Path,
        checksums: &mut HashMap<String, ChecksumEntry>,
        parser: &mut MemorySegmentParser,
        cache_dir: &Path,
        stats: &Arc<RwLock<ChecksumCacheStats>>
    ) -> Result<Vec<(MemorySegment, String)>, Box<dyn Error>> {
        // Check if file exists
        if !file_path.exists() {
            return Err(format!("File does not exist: {}", file_path.display()).into());
        }
        
        // Track validation time
        let start_time = Instant::now();
        
        // Get cache key
        let cache_key = Self::static_get_cache_key(file_path);
        
        // Get current checksum
        let file_content = fs::read_to_string(file_path)?;
        let current_checksum = format!("{:x}", md5::compute(&file_content));
        
        // Check if file has been modified
        let mut changes = Vec::new();
        let entry_exists = checksums.contains_key(&cache_key);
        
        // Compare with cached checksum if available
        if entry_exists {
            let entry = checksums.get(&cache_key).unwrap();
            
            if entry.checksum == current_checksum {
                // File not modified, return empty changes
                
                // Update stats
                if let Ok(mut stats_write) = stats.write() {
                    stats_write.total_validations += 1;
                    stats_write.valid_files += 1;
                    stats_write.total_validation_time_ms += start_time.elapsed().as_millis() as u64;
                    stats_write.avg_validation_time_ms = stats_write.total_validation_time_ms / stats_write.total_validations;
                }
                
                return Ok(changes);
            }
            
            // File modified, process segments
            info!("File modified: {}", file_path.display());
            
            // Parse current segments
            let current_segments = parser.parse_file(file_path)?;
            
            // Create segment checksums map
            let mut current_segment_checksums = HashMap::new();
            for segment in &current_segments {
                let segment_checksum = format!("{:x}", md5::compute(&segment.content));
                current_segment_checksums.insert(segment.id.clone(), segment_checksum);
            }
            
            // Compare segments with cached checksums
            let old_segments_checksums = &entry.segments_checksums;
            
            // Find added and modified segments
            for segment in &current_segments {
                let segment_id = &segment.id;
                let segment_checksum = current_segment_checksums.get(segment_id).unwrap();
                
                if !old_segments_checksums.contains_key(segment_id) {
                    // New segment
                    changes.push((segment.clone(), "ADDED".to_string()));
                } else if old_segments_checksums.get(segment_id).unwrap() != segment_checksum {
                    // Modified segment
                    changes.push((segment.clone(), "MODIFIED".to_string()));
                }
            }
            
            // Find removed segments
            for old_segment_id in old_segments_checksums.keys() {
                if !current_segment_checksums.contains_key(old_segment_id) {
                    // Removed segment - need to create a placeholder segment
                    let removed_segment = MemorySegment {
                        id: old_segment_id.clone(),
                        memory_type: crate::memory_segment_parser::MemoryType::Semantic, // Default
                        tier: crate::memory_segment_parser::MemoryTier::Tier3, // Default
                        domains: vec!["unknown".to_string()],
                        content: "".to_string(),
                        last_accessed: SystemTime::now(),
                        access_count: 0,
                        source_file: file_path.file_name().unwrap().to_string_lossy().to_string(),
                        line_start: 0,
                        line_end: 0,
                        metadata: HashMap::new(),
                    };
                    
                    changes.push((removed_segment, "REMOVED".to_string()));
                }
            }
            
            // Update cache entry
            let new_entry = ChecksumEntry {
                file_path: file_path.to_path_buf(),
                checksum: current_checksum,
                last_checked: SystemTime::now(),
                segments_checksums: current_segment_checksums,
                metadata: HashMap::new(),
            };
            
            checksums.insert(cache_key.clone(), new_entry.clone());
            
            // Save to cache file
            let cache_file = cache_dir.join(format!("{}.json", cache_key.replace("/", "_")));
            let json = serde_json::to_string_pretty(&new_entry)?;
            fs::write(cache_file, json)?;
        } else {
            // File not in cache, add it
            info!("New file added to cache: {}", file_path.display());
            
            // Parse segments
            let segments = parser.parse_file(file_path)?;
            
            // All segments are considered new
            for segment in &segments {
                changes.push((segment.clone(), "ADDED".to_string()));
            }
            
            // Create segment checksums
            let mut segment_checksums = HashMap::new();
            for segment in &segments {
                let segment_checksum = format!("{:x}", md5::compute(&segment.content));
                segment_checksums.insert(segment.id.clone(), segment_checksum);
            }
            
            // Create cache entry
            let entry = ChecksumEntry {
                file_path: file_path.to_path_buf(),
                checksum: current_checksum,
                last_checked: SystemTime::now(),
                segments_checksums: segment_checksums,
                metadata: HashMap::new(),
            };
            
            checksums.insert(cache_key.clone(), entry.clone());
            
            // Save to cache file
            let cache_file = cache_dir.join(format!("{}.json", cache_key.replace("/", "_")));
            let json = serde_json::to_string_pretty(&entry)?;
            fs::write(cache_file, json)?;
        }
        
        // Update stats
        if let Ok(mut stats_write) = stats.write() {
            stats_write.total_validations += 1;
            stats_write.modified_files += 1;
            stats_write.total_validation_time_ms += start_time.elapsed().as_millis() as u64;
            stats_write.avg_validation_time_ms = stats_write.total_validation_time_ms / stats_write.total_validations;
        }
        
        Ok(changes)
    }
    
    // Validate all memory files for an agent
    pub fn validate_agent_memory(&mut self, agent_name: &str, base_dir: &Path) -> Result<HashMap<PathBuf, Vec<(MemorySegment, String)>>, Box<dyn Error>> {
        let agent_dir = base_dir.join(agent_name);
        if !agent_dir.exists() || !agent_dir.is_dir() {
            return Err(format!("Agent directory not found: {}", agent_dir.display()).into());
        }
        
        // Find all memory files
        let mut memory_files = Vec::new();
        
        let memory_md = agent_dir.join("MEMORY.md");
        if memory_md.exists() {
            memory_files.push(memory_md);
        }
        
        let learning_md = agent_dir.join("ContinuousLearning.md");
        if learning_md.exists() {
            memory_files.push(learning_md);
        }
        
        let readme_md = agent_dir.join("README.md");
        if readme_md.exists() {
            memory_files.push(readme_md);
        }
        
        // Validate each file
        let mut all_changes = HashMap::new();
        for file_path in memory_files {
            match self.validate_file(&file_path) {
                Ok(changes) => {
                    if !changes.is_empty() {
                        all_changes.insert(file_path, changes);
                    }
                },
                Err(e) => {
                    warn!("Failed to validate file {}: {}", file_path.display(), e);
                }
            }
        }
        
        // Update last validation time
        self.last_validation.insert(agent_name.to_string(), SystemTime::now());
        
        // Update stats
        if let Ok(mut stats) = self.stats.write() {
            stats.last_full_validation = Some(SystemTime::now());
        }
        
        Ok(all_changes)
    }
    
    // Get cache key for a file path
    fn get_cache_key(&self, file_path: &Path) -> String {
        // Use normalized path as cache key
        file_path.to_string_lossy().to_string()
    }
    
    // Static version of get_cache_key
    fn static_get_cache_key(file_path: &Path) -> String {
        // Use normalized path as cache key
        file_path.to_string_lossy().to_string()
    }
    
    // Get statistics
    pub fn get_statistics(&self) -> Arc<RwLock<ChecksumCacheStats>> {
        self.stats.clone()
    }
    
    // Reset statistics
    pub fn reset_statistics(&self) {
        if let Ok(mut stats) = self.stats.write() {
            *stats = ChecksumCacheStats::default();
        }
    }
    
    // Clean up old cache entries
    pub fn cleanup_cache(&mut self, max_age_days: u64) -> Result<usize, Box<dyn Error>> {
        let now = SystemTime::now();
        let max_age = Duration::from_secs(60 * 60 * 24 * max_age_days);
        
        let mut removed_count = 0;
        let mut to_remove = Vec::new();
        
        // Find old entries
        for entry in self.checksums.iter() {
            if let Ok(duration) = now.duration_since(entry.last_checked) {
                if duration > max_age {
                    to_remove.push(entry.key().clone());
                }
            }
        }
        
        // Remove from cache
        for key in &to_remove {
            self.checksums.remove(key);
            
            // Remove cache file
            let cache_file = self.config.cache_dir.join(format!("{}.json", key.replace("/", "_")));
            if cache_file.exists() {
                fs::remove_file(cache_file)?;
                removed_count += 1;
            }
        }
        
        Ok(removed_count)
    }
}

// Example usage function
pub fn main() -> Result<(), Box<dyn Error>> {
    // Set up logging
    env_logger::init();
    
    // Create cache with default config
    let config = ChecksumCacheConfig::default();
    let mut cache = ChecksumCache::new(config)?;
    
    // Example: Validate a file
    let file_path = Path::new("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS/Athena/MEMORY.md");
    let changes = cache.validate_file(file_path)?;
    
    println!("Found {} changes in file {}", changes.len(), file_path.display());
    for (segment, change_type) in &changes {
        println!("  {} segment: {}", change_type, segment.id);
    }
    
    // Example: Start file watcher
    let base_dir = Path::new("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS");
    cache.start_file_watcher(base_dir)?;
    
    // Sleep to allow watcher to process changes
    thread::sleep(Duration::from_secs(10));
    
    // Stop file watcher
    cache.stop_file_watcher()?;
    
    // Get statistics
    let stats = cache.get_statistics();
    if let Ok(stats_read) = stats.read() {
        println!("Total validations: {}", stats_read.total_validations);
        println!("Modified files: {}", stats_read.modified_files);
        println!("Average validation time: {}ms", stats_read.avg_validation_time_ms);
    }
    
    Ok(())
}