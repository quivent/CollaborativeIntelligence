use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

// Import from other crate modules
use crate::memory_segment_parser::{MemorySegment, MemorySegmentParser, MemoryTier, MemoryType};
use crate::memory_activation_model::{MemoryActivationModel, SegmentActivation};

// Loader state for resuming interrupted operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoaderState {
    pub last_load_time: SystemTime,
    pub loaded_agents: Vec<String>,
    pub tiers_loaded: HashMap<String, Vec<MemoryTier>>,
    pub segments_loaded: HashMap<String, Vec<String>>,
    pub failed_loads: HashMap<String, String>, // Agent -> Error message
    pub load_statistics: LoadStatistics,
}

// Statistics for memory loading
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoadStatistics {
    pub total_load_time_ms: u64,
    pub tier1_load_time_ms: u64,
    pub tier2_load_time_ms: u64,
    pub tier3_load_time_ms: u64,
    pub total_segments_loaded: u64,
    pub total_bytes_loaded: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub parallel_loads: u64,
    pub sequential_loads: u64,
}

// Memory segment cache
#[derive(Debug)]
pub struct MemoryCache {
    cache_dir: PathBuf,
    segments: HashMap<String, CachedSegment>,
    max_age: Duration,
    last_cleanup: SystemTime,
}

// Cached segment entry
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedSegment {
    segment: MemorySegment,
    source_hash: String,
    timestamp: SystemTime,
    access_count: u32,
}

// Config for memory loader
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLoaderConfig {
    pub base_agents_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub cache_max_age_days: u64,
    pub parallel_loading: bool,
    pub cache_enabled: bool,
    pub preload_identity_segments: bool,
    pub auto_refresh_interval_mins: u64,
    pub context_based_loading: bool,
    pub verbose_logging: bool,
    pub timeout_seconds: u64,
}

impl Default for MemoryLoaderConfig {
    fn default() -> Self {
        MemoryLoaderConfig {
            base_agents_dir: PathBuf::from("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS"),
            cache_dir: PathBuf::from("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CACHE/MEMORY"),
            cache_max_age_days: 7,
            parallel_loading: true,
            cache_enabled: true,
            preload_identity_segments: true,
            auto_refresh_interval_mins: 60,
            context_based_loading: true,
            verbose_logging: false,
            timeout_seconds: 30,
        }
    }
}

// Memory loader implementation
pub struct MemoryLoader {
    config: MemoryLoaderConfig,
    parser: MemorySegmentParser,
    cache: Option<MemoryCache>,
    activation_model: Arc<Mutex<MemoryActivationModel>>,
    state: LoaderState,
}

impl MemoryLoader {
    // Create a new memory loader
    pub fn new(config: MemoryLoaderConfig) -> Result<Self, Box<dyn Error>> {
        // Create cache directory if it doesn't exist
        if config.cache_enabled {
            fs::create_dir_all(&config.cache_dir)?;
        }
        
        // Initialize parser with common domains
        let mut parser = MemorySegmentParser::new();
        parser.register_domain("identity", vec!["identity".to_string(), "purpose".to_string(), "core".to_string()]);
        parser.register_domain("learning", vec!["learn".to_string(), "knowledge".to_string(), "education".to_string()]);
        parser.register_domain("system", vec!["system".to_string(), "framework".to_string(), "architecture".to_string()]);
        parser.register_domain("memory", vec!["memory".to_string(), "recall".to_string(), "remember".to_string()]);
        parser.register_domain("technical", vec!["code".to_string(), "implementation".to_string(), "software".to_string()]);
        
        // Initialize or load cache
        let cache = if config.cache_enabled {
            Some(MemoryCache {
                cache_dir: config.cache_dir.clone(),
                segments: HashMap::new(),
                max_age: Duration::from_secs(60 * 60 * 24 * config.cache_max_age_days),
                last_cleanup: SystemTime::now(),
            })
        } else {
            None
        };
        
        // Initialize empty loader state
        let state = LoaderState {
            last_load_time: SystemTime::now(),
            loaded_agents: Vec::new(),
            tiers_loaded: HashMap::new(),
            segments_loaded: HashMap::new(),
            failed_loads: HashMap::new(),
            load_statistics: LoadStatistics::default(),
        };
        
        Ok(MemoryLoader {
            config,
            parser,
            cache,
            activation_model: Arc::new(Mutex::new(MemoryActivationModel::new())),
            state,
        })
    }
    
    // Load an agent's memory
    pub fn load_agent_memory(&mut self, agent_name: &str, context: Option<Vec<String>>) -> Result<Vec<MemorySegment>, Box<dyn Error>> {
        let start_time = Instant::now();
        
        let agent_dir = self.config.base_agents_dir.join(agent_name);
        if !agent_dir.exists() || !agent_dir.is_dir() {
            return Err(format!("Agent directory not found: {}", agent_dir.display()).into());
        }
        
        // Track this agent as loaded
        if !self.state.loaded_agents.contains(&agent_name.to_string()) {
            self.state.loaded_agents.push(agent_name.to_string());
        }
        
        // Initialize tracking for this agent
        self.state.tiers_loaded.entry(agent_name.to_string())
            .or_insert_with(Vec::new);
        self.state.segments_loaded.entry(agent_name.to_string())
            .or_insert_with(Vec::new);
        
        let mut all_segments = Vec::new();
        
        // Set context for activation model if provided
        if let Some(ctx) = &context {
            if let Ok(mut model) = self.activation_model.lock() {
                model.set_context(ctx.clone());
            }
        }
        
        // Load memory files in order of importance
        let memory_files = self.get_agent_memory_files(agent_dir.clone())?;
        
        // First approach: Load each tier sequentially
        if !self.config.parallel_loading {
            self.state.load_statistics.sequential_loads += 1;
            
            // Load Tier 1 segments (critical, always loaded)
            let tier1_start = Instant::now();
            let tier1_segments = self.load_segments_by_tier(agent_name, &memory_files, MemoryTier::Tier1)?;
            all_segments.extend(tier1_segments);
            self.state.load_statistics.tier1_load_time_ms += tier1_start.elapsed().as_millis() as u64;
            
            // Add tier to list of loaded tiers for this agent
            if !self.state.tiers_loaded.get(agent_name).unwrap().contains(&MemoryTier::Tier1) {
                self.state.tiers_loaded.get_mut(agent_name).unwrap().push(MemoryTier::Tier1);
            }
            
            // Load Tier 2 segments (important, loaded in most contexts)
            let tier2_start = Instant::now();
            let tier2_segments = self.load_segments_by_tier(agent_name, &memory_files, MemoryTier::Tier2)?;
            all_segments.extend(tier2_segments);
            self.state.load_statistics.tier2_load_time_ms += tier2_start.elapsed().as_millis() as u64;
            
            // Add tier to list of loaded tiers for this agent
            if !self.state.tiers_loaded.get(agent_name).unwrap().contains(&MemoryTier::Tier2) {
                self.state.tiers_loaded.get_mut(agent_name).unwrap().push(MemoryTier::Tier2);
            }
            
            // Load Tier 3 segments (supplementary, loaded when relevant to context)
            if self.config.context_based_loading && context.is_some() {
                let tier3_start = Instant::now();
                let tier3_segments = self.load_segments_by_tier_and_context(
                    agent_name, 
                    &memory_files, 
                    MemoryTier::Tier3, 
                    context.as_ref().unwrap()
                )?;
                all_segments.extend(tier3_segments);
                self.state.load_statistics.tier3_load_time_ms += tier3_start.elapsed().as_millis() as u64;
                
                // Add tier to list of loaded tiers for this agent
                if !self.state.tiers_loaded.get(agent_name).unwrap().contains(&MemoryTier::Tier3) {
                    self.state.tiers_loaded.get_mut(agent_name).unwrap().push(MemoryTier::Tier3);
                }
            }
        } else {
            // Second approach: Load tiers in parallel
            self.state.load_statistics.parallel_loads += 1;
            
            let agent_name_clone = agent_name.to_string();
            let memory_files_arc = Arc::new(memory_files);
            let context_arc = Arc::new(context.clone());
            let loader_arc = Arc::new(Mutex::new(self));
            
            // Create threads for each tier
            let t1_loader = loader_arc.clone();
            let t1_files = memory_files_arc.clone();
            let t1_agent = agent_name_clone.clone();
            let tier1_handle = thread::spawn(move || {
                let tier1_start = Instant::now();
                
                if let Ok(mut loader) = t1_loader.lock() {
                    // Load Tier 1 segments
                    let segments = loader.load_segments_by_tier(&t1_agent, &t1_files, MemoryTier::Tier1).unwrap_or_default();
                    
                    // Update statistics
                    loader.state.load_statistics.tier1_load_time_ms += tier1_start.elapsed().as_millis() as u64;
                    
                    // Add tier to list of loaded tiers
                    if !loader.state.tiers_loaded.get(&t1_agent).unwrap().contains(&MemoryTier::Tier1) {
                        loader.state.tiers_loaded.get_mut(&t1_agent).unwrap().push(MemoryTier::Tier1);
                    }
                    
                    segments
                } else {
                    Vec::new()
                }
            });
            
            let t2_loader = loader_arc.clone();
            let t2_files = memory_files_arc.clone();
            let t2_agent = agent_name_clone.clone();
            let tier2_handle = thread::spawn(move || {
                let tier2_start = Instant::now();
                
                if let Ok(mut loader) = t2_loader.lock() {
                    // Load Tier 2 segments
                    let segments = loader.load_segments_by_tier(&t2_agent, &t2_files, MemoryTier::Tier2).unwrap_or_default();
                    
                    // Update statistics
                    loader.state.load_statistics.tier2_load_time_ms += tier2_start.elapsed().as_millis() as u64;
                    
                    // Add tier to list of loaded tiers
                    if !loader.state.tiers_loaded.get(&t2_agent).unwrap().contains(&MemoryTier::Tier2) {
                        loader.state.tiers_loaded.get_mut(&t2_agent).unwrap().push(MemoryTier::Tier2);
                    }
                    
                    segments
                } else {
                    Vec::new()
                }
            });
            
            let t3_loader = loader_arc.clone();
            let t3_files = memory_files_arc.clone();
            let t3_agent = agent_name_clone.clone();
            let t3_context = context_arc.clone();
            let context_based = self.config.context_based_loading;
            
            let tier3_handle = thread::spawn(move || {
                let tier3_start = Instant::now();
                
                if let Ok(mut loader) = t3_loader.lock() {
                    // Load Tier 3 segments if context is provided
                    let segments = if context_based && t3_context.is_some() {
                        loader.load_segments_by_tier_and_context(
                            &t3_agent,
                            &t3_files,
                            MemoryTier::Tier3,
                            t3_context.as_ref().unwrap()
                        ).unwrap_or_default()
                    } else {
                        Vec::new()
                    };
                    
                    // Update statistics
                    loader.state.load_statistics.tier3_load_time_ms += tier3_start.elapsed().as_millis() as u64;
                    
                    // Add tier to list of loaded tiers
                    if !loader.state.tiers_loaded.get(&t3_agent).unwrap().contains(&MemoryTier::Tier3) && !segments.is_empty() {
                        loader.state.tiers_loaded.get_mut(&t3_agent).unwrap().push(MemoryTier::Tier3);
                    }
                    
                    segments
                } else {
                    Vec::new()
                }
            });
            
            // Wait for all threads to complete
            let tier1_segments = tier1_handle.join().unwrap_or_default();
            let tier2_segments = tier2_handle.join().unwrap_or_default();
            let tier3_segments = tier3_handle.join().unwrap_or_default();
            
            // Combine all segments
            all_segments.extend(tier1_segments);
            all_segments.extend(tier2_segments);
            all_segments.extend(tier3_segments);
        }
        
        // Update statistics
        self.state.load_statistics.total_load_time_ms += start_time.elapsed().as_millis() as u64;
        self.state.load_statistics.total_segments_loaded += all_segments.len() as u64;
        self.state.load_statistics.total_bytes_loaded += all_segments.iter()
            .map(|s| s.content.len() as u64)
            .sum::<u64>();
        
        // Register loaded segments with activation model
        if let Ok(mut model) = self.activation_model.lock() {
            for segment in &all_segments {
                model.register_segment(segment.clone());
            }
        }
        
        // Update loader state
        self.state.last_load_time = SystemTime::now();
        
        Ok(all_segments)
    }
    
    // Get all memory files for an agent
    fn get_agent_memory_files(&self, agent_dir: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let mut memory_files = Vec::new();
        
        // Check for MEMORY.md (primary memory file)
        let memory_md = agent_dir.join("MEMORY.md");
        if memory_md.exists() {
            memory_files.push(memory_md);
        }
        
        // Check for ContinuousLearning.md (secondary memory file)
        let learning_md = agent_dir.join("ContinuousLearning.md");
        if learning_md.exists() {
            memory_files.push(learning_md);
        }
        
        // Check for README.md (tertiary memory file, usually contains identity information)
        let readme_md = agent_dir.join("README.md");
        if readme_md.exists() {
            memory_files.push(readme_md);
        }
        
        // Check for any other .md files in the agent's directory
        for entry in fs::read_dir(agent_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && 
               path.extension().map_or(false, |ext| ext == "md") && 
               !memory_files.contains(&path) {
                memory_files.push(path);
            }
        }
        
        Ok(memory_files)
    }
    
    // Load segments by tier
    fn load_segments_by_tier(&mut self, agent_name: &str, files: &[PathBuf], tier: MemoryTier) -> Result<Vec<MemorySegment>, Box<dyn Error>> {
        let mut segments = Vec::new();
        
        for file_path in files {
            // Try to load from cache first
            if self.config.cache_enabled {
                if let Some(cached_segments) = self.load_from_cache(file_path, Some(tier), None)? {
                    segments.extend(cached_segments);
                    continue;
                }
            }
            
            // If not in cache or cache is disabled, parse the file
            let file_segments = self.parser.parse_file(file_path)?;
            
            // Filter by tier
            let tier_segments: Vec<MemorySegment> = file_segments.into_iter()
                .filter(|s| s.tier == tier)
                .collect();
            
            // Cache the segments if caching is enabled
            if self.config.cache_enabled && !tier_segments.is_empty() {
                if let Some(cache) = &mut self.cache {
                    for segment in &tier_segments {
                        cache.add_segment(segment.clone(), self.calculate_file_hash(file_path)?)?;
                    }
                }
            }
            
            segments.extend(tier_segments);
        }
        
        // Track loaded segments
        for segment in &segments {
            let segment_id = segment.id.clone();
            if !self.state.segments_loaded.get(agent_name).unwrap().contains(&segment_id) {
                self.state.segments_loaded.get_mut(agent_name).unwrap().push(segment_id);
            }
        }
        
        Ok(segments)
    }
    
    // Load segments by tier and context relevance
    fn load_segments_by_tier_and_context(&mut self, agent_name: &str, files: &[PathBuf], tier: MemoryTier, context: &[String]) -> Result<Vec<MemorySegment>, Box<dyn Error>> {
        let mut segments = Vec::new();
        
        for file_path in files {
            // Try to load from cache first
            if self.config.cache_enabled {
                if let Some(cached_segments) = self.load_from_cache(file_path, Some(tier), None)? {
                    // Filter by context relevance
                    let relevant_segments = self.filter_by_context_relevance(cached_segments, context);
                    segments.extend(relevant_segments);
                    continue;
                }
            }
            
            // If not in cache or cache is disabled, parse the file
            let file_segments = self.parser.parse_file(file_path)?;
            
            // Filter by tier
            let tier_segments: Vec<MemorySegment> = file_segments.into_iter()
                .filter(|s| s.tier == tier)
                .collect();
            
            // Cache the segments if caching is enabled
            if self.config.cache_enabled && !tier_segments.is_empty() {
                if let Some(cache) = &mut self.cache {
                    for segment in &tier_segments {
                        cache.add_segment(segment.clone(), self.calculate_file_hash(file_path)?)?;
                    }
                }
            }
            
            // Filter by context relevance
            let relevant_segments = self.filter_by_context_relevance(tier_segments, context);
            segments.extend(relevant_segments);
        }
        
        // Track loaded segments
        for segment in &segments {
            let segment_id = segment.id.clone();
            if !self.state.segments_loaded.get(agent_name).unwrap().contains(&segment_id) {
                self.state.segments_loaded.get_mut(agent_name).unwrap().push(segment_id);
            }
        }
        
        Ok(segments)
    }
    
    // Filter segments by context relevance
    fn filter_by_context_relevance(&self, segments: Vec<MemorySegment>, context: &[String]) -> Vec<MemorySegment> {
        if context.is_empty() {
            return segments;
        }
        
        // Simple relevance scoring based on keyword matching
        segments.into_iter()
            .filter(|segment| {
                // Check content for context keywords
                let content_lower = segment.content.to_lowercase();
                for keyword in context {
                    if content_lower.contains(&keyword.to_lowercase()) {
                        return true;
                    }
                }
                
                // Check domains for context relevance
                for domain in &segment.domains {
                    for keyword in context {
                        if domain.to_lowercase().contains(&keyword.to_lowercase()) {
                            return true;
                        }
                    }
                }
                
                // Check metadata for context relevance
                for (_, value) in &segment.metadata {
                    for keyword in context {
                        if value.to_lowercase().contains(&keyword.to_lowercase()) {
                            return true;
                        }
                    }
                }
                
                false
            })
            .collect()
    }
    
    // Load segments from cache
    fn load_from_cache(&mut self, file_path: &Path, tier: Option<MemoryTier>, memory_type: Option<MemoryType>) -> Result<Option<Vec<MemorySegment>>, Box<dyn Error>> {
        if let Some(cache) = &mut self.cache {
            // Calculate file hash
            let file_hash = self.calculate_file_hash(file_path)?;
            
            // Generate cache key (normalized file path)
            let cache_key = file_path.to_string_lossy().to_string();
            
            // Check if file exists in cache and hash matches
            let mut segments = Vec::new();
            let mut found = false;
            
            for (id, cached) in &cache.segments {
                if id.starts_with(&cache_key) && cached.source_hash == file_hash {
                    // Check tier if specified
                    if tier.is_some() && cached.segment.tier != tier.unwrap() {
                        continue;
                    }
                    
                    // Check memory type if specified
                    if memory_type.is_some() && cached.segment.memory_type != memory_type.unwrap() {
                        continue;
                    }
                    
                    segments.push(cached.segment.clone());
                    found = true;
                    
                    // Track cache hit
                    self.state.load_statistics.cache_hits += 1;
                }
            }
            
            if found {
                return Ok(Some(segments));
            }
        }
        
        // Track cache miss
        self.state.load_statistics.cache_misses += 1;
        
        Ok(None)
    }
    
    // Calculate file hash for cache validation
    fn calculate_file_hash(&self, file_path: &Path) -> Result<String, Box<dyn Error>> {
        let content = fs::read_to_string(file_path)?;
        let hash = format!("{:x}", md5::compute(content));
        Ok(hash)
    }
    
    // Get activation model for external use
    pub fn get_activation_model(&self) -> Arc<Mutex<MemoryActivationModel>> {
        self.activation_model.clone()
    }
    
    // Get loader state
    pub fn get_loader_state(&self) -> &LoaderState {
        &self.state
    }
    
    // Reset statistics
    pub fn reset_statistics(&mut self) {
        self.state.load_statistics = LoadStatistics::default();
    }
    
    // Cleanup cache
    pub fn cleanup_cache(&mut self) -> Result<usize, Box<dyn Error>> {
        if let Some(cache) = &mut self.cache {
            let now = SystemTime::now();
            
            // Skip if cleanup was performed recently
            if now.duration_since(cache.last_cleanup)? < Duration::from_secs(60 * 60) {
                return Ok(0);
            }
            
            let mut to_remove = Vec::new();
            
            for (id, segment) in &cache.segments {
                if now.duration_since(segment.timestamp)? > cache.max_age {
                    to_remove.push(id.clone());
                }
            }
            
            let removed_count = to_remove.len();
            
            for id in &to_remove {
                cache.segments.remove(id);
                
                // Remove cache file if it exists
                let cache_file = cache.cache_dir.join(format!("{}.json", id));
                if cache_file.exists() {
                    fs::remove_file(cache_file)?;
                }
            }
            
            cache.last_cleanup = now;
            
            Ok(removed_count)
        } else {
            Ok(0)
        }
    }
}

// Implementation for MemoryCache
impl MemoryCache {
    // Add a segment to the cache
    fn add_segment(&mut self, segment: MemorySegment, source_hash: String) -> Result<(), Box<dyn Error>> {
        let segment_id = segment.id.clone();
        let cache_key = format!("{}_{}", segment.source_file, segment_id);
        
        let cached = CachedSegment {
            segment,
            source_hash,
            timestamp: SystemTime::now(),
            access_count: 0,
        };
        
        self.segments.insert(cache_key.clone(), cached.clone());
        
        // Save to disk
        let cache_file = self.cache_dir.join(format!("{}.json", cache_key));
        let json = serde_json::to_string(&cached)?;
        fs::write(cache_file, json)?;
        
        Ok(())
    }
    
    // Load all cache entries from disk
    pub fn load_from_disk(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir)?;
            return Ok(());
        }
        
        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let content = fs::read_to_string(&path)?;
                match serde_json::from_str::<CachedSegment>(&content) {
                    Ok(cached) => {
                        let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
                        self.segments.insert(file_name, cached);
                    },
                    Err(_) => continue, // Skip invalid cache entries
                }
            }
        }
        
        Ok(())
    }
}

// Example usage function
pub fn main() -> Result<(), Box<dyn Error>> {
    // Initialize loader with default config
    let config = MemoryLoaderConfig::default();
    let mut loader = MemoryLoader::new(config)?;
    
    // Load an agent's memory
    let agent_name = "Athena";
    let context = Some(vec!["memory".to_string(), "system".to_string()]);
    
    let segments = loader.load_agent_memory(agent_name, context)?;
    
    println!("Loaded {} memory segments for {}", segments.len(), agent_name);
    
    // Print loading statistics
    let stats = &loader.get_loader_state().load_statistics;
    println!("Total load time: {}ms", stats.total_load_time_ms);
    println!("Segments loaded: {}", stats.total_segments_loaded);
    println!("Bytes loaded: {}", stats.total_bytes_loaded);
    println!("Cache hits: {}", stats.cache_hits);
    println!("Cache misses: {}", stats.cache_misses);
    
    Ok(())
}