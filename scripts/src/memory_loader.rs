use std::path::{Path, PathBuf};
use std::fs;
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

use super::memory_segment_parser::{MemorySegment, MemorySegmentParser, MemoryType, MemoryTier};
use super::memory_activation_model::{MemoryActivationModel, SegmentActivation};

// Configuration for the memory loader
#[derive(Debug, Clone)]
pub struct MemoryLoaderConfig {
    pub base_directory: PathBuf,
    pub agents_directory: PathBuf,
    pub cache_directory: PathBuf,
    pub segment_size: usize,
    pub segment_overlap: usize,
    pub smart_segmentation: bool,
    pub cache_duration: Duration,
    pub preload_core_memory: bool,
}

impl Default for MemoryLoaderConfig {
    fn default() -> Self {
        Self {
            base_directory: PathBuf::from("."),
            agents_directory: PathBuf::from("AGENTS"),
            cache_directory: PathBuf::from("CACHE"),
            segment_size: 4096,
            segment_overlap: 256,
            smart_segmentation: true,
            cache_duration: Duration::from_secs(3600), // 1 hour cache
            preload_core_memory: true,
        }
    }
}

// Memory loader statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadStatistics {
    pub total_loads: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub total_segments_loaded: u64,
    pub load_time_ms: u64,
    pub last_load_time: u64,
}

impl Default for LoadStatistics {
    fn default() -> Self {
        Self {
            total_loads: 0,
            cache_hits: 0,
            cache_misses: 0,
            total_segments_loaded: 0,
            load_time_ms: 0,
            last_load_time: 0,
        }
    }
}

// Memory loader state
#[derive(Debug)]
pub struct LoaderState {
    pub load_statistics: LoadStatistics,
    pub segment_cache: HashMap<String, Vec<MemorySegment>>,
    pub cache_timestamps: HashMap<String, u64>,
    pub preloaded_agents: HashSet<String>,
}

impl Default for LoaderState {
    fn default() -> Self {
        Self {
            load_statistics: LoadStatistics::default(),
            segment_cache: HashMap::new(),
            cache_timestamps: HashMap::new(),
            preloaded_agents: HashSet::new(),
        }
    }
}

// Memory loader
pub struct MemoryLoader {
    config: MemoryLoaderConfig,
    state: Arc<Mutex<LoaderState>>,
    segment_parser: MemorySegmentParser,
    activation_model: Arc<MemoryActivationModel>,
}

impl MemoryLoader {
    // Create a new memory loader
    pub fn new(config: MemoryLoaderConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // Create segment parser
        let segment_parser = MemorySegmentParser::new(
            config.segment_size,
            config.segment_overlap,
            config.smart_segmentation,
        );
        
        // Create activation model
        let activation_model = Arc::new(MemoryActivationModel::new());
        
        // Create loader
        let loader = Self {
            config,
            state: Arc::new(Mutex::new(LoaderState::default())),
            segment_parser,
            activation_model,
        };
        
        // Preload core memory if configured
        if loader.config.preload_core_memory {
            loader.preload_core_memory()?;
        }
        
        Ok(loader)
    }
    
    // Get a reference to the activation model
    pub fn get_activation_model(&self) -> Arc<MemoryActivationModel> {
        self.activation_model.clone()
    }
    
    // Preload core memory for all agents
    fn preload_core_memory(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Get list of agent directories
        let agents_dir = self.config.base_directory.join(&self.config.agents_directory);
        
        if !agents_dir.exists() {
            return Ok(());
        }
        
        for entry in fs::read_dir(agents_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            // Only process directories
            if path.is_dir() {
                if let Some(agent_name) = path.file_name().and_then(|n| n.to_str()) {
                    // Skip directories that start with . or _
                    if agent_name.starts_with('.') || agent_name.starts_with('_') {
                        continue;
                    }
                    
                    // Try to load core memory file
                    let memory_file = path.join("MEMORY.md");
                    if memory_file.exists() {
                        // Parse memory file
                        let segments = self.segment_parser.parse_file(&memory_file)?;
                        
                        // Store in cache
                        let mut state = self.state.lock().unwrap();
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs();
                        
                        let cache_key = format!("{}:core", agent_name);
                        state.segment_cache.insert(cache_key.clone(), segments);
                        state.cache_timestamps.insert(cache_key, now);
                        state.preloaded_agents.insert(agent_name.to_string());
                    }
                }
            }
        }
        
        Ok(())
    }
    
    // Load memory for an agent with optional context
    pub fn load_agent_memory(
        &self,
        agent_name: &str,
        context: Option<Vec<String>>,
    ) -> Result<Vec<MemorySegment>, Box<dyn std::error::Error>> {
        // Start timing
        let start_time = SystemTime::now();
        
        // Try to load from cache first if no specific context is requested
        if context.is_none() {
            let cache_key = format!("{}:core", agent_name);
            
            // Check if we have a valid cache entry
            let mut state = self.state.lock().unwrap();
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            let cache_valid = if let Some(timestamp) = state.cache_timestamps.get(&cache_key) {
                now.saturating_sub(*timestamp) < self.config.cache_duration.as_secs()
            } else {
                false
            };
            
            if cache_valid {
                if let Some(cached_segments) = state.segment_cache.get(&cache_key) {
                    // Update statistics
                    state.load_statistics.total_loads += 1;
                    state.load_statistics.cache_hits += 1;
                    state.load_statistics.total_segments_loaded += cached_segments.len() as u64;
                    
                    let elapsed = start_time
                        .elapsed()
                        .unwrap_or_default()
                        .as_millis() as u64;
                    
                    state.load_statistics.load_time_ms += elapsed;
                    state.load_statistics.last_load_time = now;
                    
                    // Register access for each segment
                    let cloned_segments = cached_segments.clone();
                    for segment in &cloned_segments {
                        self.activation_model.register_access(agent_name, segment, None);
                    }
                    
                    return Ok(cloned_segments);
                }
            }
            
            // Update stats for cache miss
            state.load_statistics.cache_misses += 1;
        }
        
        // Load from files if not in cache or a specific context is requested
        let mut segments = self.load_core_memory(agent_name)?;
        
        // Add context-specific memory if requested
        if let Some(contexts) = context {
            for ctx in contexts {
                let context_segments = self.load_context_memory(agent_name, &ctx)?;
                segments.extend(context_segments);
            }
        }
        
        // Update cache if no specific context was requested
        if context.is_none() {
            let cache_key = format!("{}:core", agent_name);
            
            let mut state = self.state.lock().unwrap();
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            state.segment_cache.insert(cache_key.clone(), segments.clone());
            state.cache_timestamps.insert(cache_key, now);
        }
        
        // Update statistics
        {
            let mut state = self.state.lock().unwrap();
            state.load_statistics.total_loads += 1;
            state.load_statistics.total_segments_loaded += segments.len() as u64;
            
            let elapsed = start_time
                .elapsed()
                .unwrap_or_default()
                .as_millis() as u64;
            
            state.load_statistics.load_time_ms += elapsed;
            state.load_statistics.last_load_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
        }
        
        // Register access for each segment
        for segment in &segments {
            self.activation_model.register_access(agent_name, segment, None);
        }
        
        Ok(segments)
    }
    
    // Load core memory (MEMORY.md and ContinuousLearning.md)
    fn load_core_memory(&self, agent_name: &str) -> Result<Vec<MemorySegment>, Box<dyn std::error::Error>> {
        let agent_dir = self.config.base_directory
            .join(&self.config.agents_directory)
            .join(agent_name);
        
        if !agent_dir.exists() {
            return Err(format!("Agent directory not found: {}", agent_name).into());
        }
        
        let mut segments = Vec::new();
        
        // Load MEMORY.md file
        let memory_file = agent_dir.join("MEMORY.md");
        if memory_file.exists() {
            let memory_segments = self.segment_parser.parse_file(&memory_file)?;
            segments.extend(memory_segments);
        }
        
        // Load ContinuousLearning.md file
        let learning_file = agent_dir.join("ContinuousLearning.md");
        if learning_file.exists() {
            let learning_segments = self.segment_parser.parse_file(&learning_file)?;
            segments.extend(learning_segments);
        }
        
        Ok(segments)
    }
    
    // Load context-specific memory
    fn load_context_memory(&self, agent_name: &str, context: &str) -> Result<Vec<MemorySegment>, Box<dyn std::error::Error>> {
        let agent_dir = self.config.base_directory
            .join(&self.config.agents_directory)
            .join(agent_name);
        
        let sessions_dir = agent_dir.join("Sessions");
        if !sessions_dir.exists() {
            return Ok(Vec::new());
        }
        
        let mut segments = Vec::new();
        let context_lower = context.to_lowercase();
        
        // First, check for directories matching the context
        for entry in fs::read_dir(&sessions_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if dir_name.to_lowercase().contains(&context_lower) {
                        // Found a matching directory, load all markdown files
                        for file_entry in fs::read_dir(&path)? {
                            let file_entry = file_entry?;
                            let file_path = file_entry.path();
                            
                            if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "md") {
                                let file_segments = self.segment_parser.parse_file(&file_path)?;
                                segments.extend(file_segments);
                            }
                        }
                    }
                }
            }
        }
        
        // Next, check for files directly matching the context
        for entry in fs::read_dir(&sessions_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                if let Some(file_name) = path.file_stem().and_then(|n| n.to_str()) {
                    if file_name.to_lowercase().contains(&context_lower) {
                        let file_segments = self.segment_parser.parse_file(&path)?;
                        segments.extend(file_segments);
                    }
                }
            }
        }
        
        Ok(segments)
    }
    
    // Get loader state
    pub fn get_loader_state(&self) -> LoaderState {
        self.state.lock().unwrap().clone()
    }
    
    // Reset statistics
    pub fn reset_statistics(&self) {
        let mut state = self.state.lock().unwrap();
        state.load_statistics = LoadStatistics::default();
    }
    
    // Clear cache
    pub fn clear_cache(&self) {
        let mut state = self.state.lock().unwrap();
        state.segment_cache.clear();
        state.cache_timestamps.clear();
    }
    
    // Remove agent from cache
    pub fn remove_from_cache(&self, agent_name: &str) {
        let mut state = self.state.lock().unwrap();
        let keys_to_remove: Vec<String> = state.segment_cache.keys()
            .filter(|key| key.starts_with(&format!("{}:", agent_name)))
            .cloned()
            .collect();
        
        for key in keys_to_remove {
            state.segment_cache.remove(&key);
            state.cache_timestamps.remove(&key);
        }
        
        state.preloaded_agents.remove(agent_name);
    }
    
    // Update configuration
    pub fn update_config(&mut self, config: MemoryLoaderConfig) {
        self.config = config;
        self.segment_parser = MemorySegmentParser::new(
            self.config.segment_size,
            self.config.segment_overlap,
            self.config.smart_segmentation,
        );
    }
    
    // Preload specific agent
    pub fn preload_agent(&self, agent_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Load core memory
        let segments = self.load_core_memory(agent_name)?;
        
        // Update cache
        let cache_key = format!("{}:core", agent_name);
        
        let mut state = self.state.lock().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        state.segment_cache.insert(cache_key.clone(), segments);
        state.cache_timestamps.insert(cache_key, now);
        state.preloaded_agents.insert(agent_name.to_string());
        
        Ok(())
    }
}

// Clone implementation for LoaderState
impl Clone for LoaderState {
    fn clone(&self) -> Self {
        Self {
            load_statistics: self.load_statistics.clone(),
            segment_cache: self.segment_cache.clone(),
            cache_timestamps: self.cache_timestamps.clone(),
            preloaded_agents: self.preloaded_agents.clone(),
        }
    }
}