use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use dashmap::DashMap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use log::{debug, info, warn, error};

// Import from other crate modules
use crate::memory_segment_parser::{MemorySegment, MemorySegmentParser, MemoryType, MemoryTier};
use crate::memory_activation_model::MemoryActivationModel;
use crate::memory_loader::MemoryLoader;

// Configuration for global context cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalContextCacheConfig {
    pub agents_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub enable_shared_knowledge: bool,
    pub enable_cross_references: bool,
    pub max_shared_segments: usize,
    pub max_tier_level: u8,  // 1 = Tier1 only, 2 = Tier1+Tier2, 3 = All tiers
    pub update_interval_seconds: u64,
    pub parallel_processing: bool,
    pub verbose_logging: bool,
}

impl Default for GlobalContextCacheConfig {
    fn default() -> Self {
        GlobalContextCacheConfig {
            agents_dir: PathBuf::from("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS"),
            cache_dir: PathBuf::from("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CACHE/GLOBAL"),
            enable_shared_knowledge: true,
            enable_cross_references: true,
            max_shared_segments: 50,
            max_tier_level: 2,  // Default to Tier1 and Tier2 only
            update_interval_seconds: 3600, // Hourly updates
            parallel_processing: true,
            verbose_logging: false,
        }
    }
}

// Statistics for global context cache
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalCacheStats {
    pub total_segments: usize,
    pub shared_segments: usize,
    pub segments_by_agent: HashMap<String, usize>,
    pub segments_by_type: HashMap<String, usize>,
    pub cross_references: usize,
    pub last_update: Option<SystemTime>,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub avg_lookup_time_ns: u64,
}

// Shared memory segment with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedSegment {
    pub segment: MemorySegment,
    pub source_agent: String,
    pub shared_with: HashSet<String>,
    pub relevance_score: f64,
    pub last_accessed: SystemTime,
    pub access_count: u32,
    pub cross_references: Vec<String>, // IDs of related segments
}

// Cross-reference between segments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    pub source_segment_id: String,
    pub target_segment_id: String,
    pub source_agent: String,
    pub target_agent: String,
    pub relevance: f64,
    pub created_at: SystemTime,
}

// Global context cache implementation
pub struct GlobalContextCache {
    config: GlobalContextCacheConfig,
    parser: MemorySegmentParser,
    shared_segments: DashMap<String, SharedSegment>, // segment_id -> SharedSegment
    agent_segments: DashMap<String, HashSet<String>>, // agent_name -> Set of segment_ids
    cross_references: DashMap<String, Vec<CrossReference>>, // segment_id -> CrossReferences
    activation_models: DashMap<String, Arc<Mutex<MemoryActivationModel>>>, // agent_name -> ActivationModel
    stats: Arc<RwLock<GlobalCacheStats>>,
    update_thread: Option<thread::JoinHandle<()>>,
    running: Arc<RwLock<bool>>,
}

impl GlobalContextCache {
    // Create a new global context cache
    pub fn new(config: GlobalContextCacheConfig) -> Result<Self, Box<dyn Error>> {
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
        
        let cache = GlobalContextCache {
            config,
            parser,
            shared_segments: DashMap::new(),
            agent_segments: DashMap::new(),
            cross_references: DashMap::new(),
            activation_models: DashMap::new(),
            stats: Arc::new(RwLock::new(GlobalCacheStats::default())),
            update_thread: None,
            running: Arc::new(RwLock::new(false)),
        };
        
        // Load existing cache if available
        cache.load_cache()?;
        
        Ok(cache)
    }
    
    // Register an agent's activation model
    pub fn register_activation_model(&self, agent_name: &str, model: Arc<Mutex<MemoryActivationModel>>) {
        self.activation_models.insert(agent_name.to_string(), model);
    }
    
    // Load cache from disk
    fn load_cache(&self) -> Result<(), Box<dyn Error>> {
        // Load shared segments
        let segments_file = self.config.cache_dir.join("shared_segments.json");
        if segments_file.exists() {
            let content = fs::read_to_string(&segments_file)?;
            let segments: Vec<SharedSegment> = serde_json::from_str(&content)?;
            
            for segment in segments {
                self.shared_segments.insert(segment.segment.id.clone(), segment.clone());
                
                // Update agent segments mapping
                let source_agent = segment.source_agent.clone();
                if !self.agent_segments.contains_key(&source_agent) {
                    self.agent_segments.insert(source_agent.clone(), HashSet::new());
                }
                self.agent_segments.get_mut(&source_agent).unwrap().insert(segment.segment.id.clone());
            }
        }
        
        // Load cross references
        let references_file = self.config.cache_dir.join("cross_references.json");
        if references_file.exists() {
            let content = fs::read_to_string(&references_file)?;
            let references: HashMap<String, Vec<CrossReference>> = serde_json::from_str(&content)?;
            
            for (segment_id, refs) in references {
                self.cross_references.insert(segment_id, refs);
            }
        }
        
        // Load statistics
        let stats_file = self.config.cache_dir.join("stats.json");
        if stats_file.exists() {
            let content = fs::read_to_string(&stats_file)?;
            let stats: GlobalCacheStats = serde_json::from_str(&content)?;
            
            if let Ok(mut stats_write) = self.stats.write() {
                *stats_write = stats;
            }
        }
        
        Ok(())
    }
    
    // Save cache to disk
    fn save_cache(&self) -> Result<(), Box<dyn Error>> {
        // Save shared segments
        let segments: Vec<SharedSegment> = self.shared_segments.iter()
            .map(|entry| entry.value().clone())
            .collect();
        
        let segments_json = serde_json::to_string_pretty(&segments)?;
        let segments_file = self.config.cache_dir.join("shared_segments.json");
        fs::write(&segments_file, segments_json)?;
        
        // Save cross references
        let mut references = HashMap::new();
        for entry in self.cross_references.iter() {
            references.insert(entry.key().clone(), entry.value().clone());
        }
        
        let references_json = serde_json::to_string_pretty(&references)?;
        let references_file = self.config.cache_dir.join("cross_references.json");
        fs::write(&references_file, references_json)?;
        
        // Save statistics
        if let Ok(stats_read) = self.stats.read() {
            let stats_json = serde_json::to_string_pretty(&*stats_read)?;
            let stats_file = self.config.cache_dir.join("stats.json");
            fs::write(&stats_file, stats_json)?;
        }
        
        Ok(())
    }
    
    // Start background update thread
    pub fn start_background_updates(&mut self) -> Result<(), Box<dyn Error>> {
        // Signal that updates are running
        *self.running.write().unwrap() = true;
        
        // Clone references needed for the thread
        let running = self.running.clone();
        let config = self.config.clone();
        let stats = self.stats.clone();
        let cache_dir = self.config.cache_dir.clone();
        
        // Create thread-safe clones of our maps
        let shared_segments_arc = Arc::new(self.shared_segments.clone());
        let agent_segments_arc = Arc::new(self.agent_segments.clone());
        let cross_references_arc = Arc::new(self.cross_references.clone());
        let activation_models_arc = Arc::new(self.activation_models.clone());
        
        // Spawn the background thread
        let handle = thread::spawn(move || {
            info!("Background global context updater started");
            
            // Create local parser
            let mut parser = MemorySegmentParser::new();
            parser.register_domain("identity", vec!["identity".to_string(), "purpose".to_string(), "core".to_string()]);
            parser.register_domain("learning", vec!["learn".to_string(), "knowledge".to_string(), "education".to_string()]);
            parser.register_domain("system", vec!["system".to_string(), "framework".to_string(), "architecture".to_string()]);
            parser.register_domain("memory", vec!["memory".to_string(), "recall".to_string(), "remember".to_string()]);
            parser.register_domain("technical", vec!["code".to_string(), "implementation".to_string(), "software".to_string()]);
            
            while *running.read().unwrap() {
                // Update shared segments and cross references
                update_shared_knowledge(
                    &config,
                    &parser,
                    &shared_segments_arc,
                    &agent_segments_arc,
                    &cross_references_arc,
                    &activation_models_arc,
                    &stats
                );
                
                // Save cache state
                save_cache_state(
                    &cache_dir,
                    &shared_segments_arc,
                    &cross_references_arc,
                    &stats
                );
                
                // Sleep for update interval
                for _ in 0..60 {  // Check every minute if we should stop
                    if !*running.read().unwrap() {
                        break;
                    }
                    thread::sleep(Duration::from_secs(config.update_interval_seconds / 60));
                }
            }
            
            info!("Background global context updater stopped");
        });
        
        self.update_thread = Some(handle);
        
        Ok(())
    }
    
    // Stop background updates
    pub fn stop_background_updates(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(_) = self.update_thread {
            // Signal the updater to stop
            *self.running.write().unwrap() = false;
            
            // Wait for the thread to finish
            if let Some(handle) = self.update_thread.take() {
                handle.join().map_err(|_| "Failed to join update thread")?;
            }
            
            // Save cache state one last time
            self.save_cache()?;
            
            info!("Background global context updater stopped");
        }
        
        Ok(())
    }
    
    // Update shared knowledge
    pub fn update_shared_knowledge(&self) -> Result<(), Box<dyn Error>> {
        // Get list of agents
        let agents_dir = &self.config.agents_dir;
        let agents = fs::read_dir(agents_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect::<Vec<String>>();
        
        if self.config.parallel_processing {
            // Process agents in parallel
            agents.par_iter().for_each(|agent_name| {
                if let Err(e) = self.update_agent_shared_knowledge(agent_name) {
                    warn!("Failed to update shared knowledge for agent {}: {}", agent_name, e);
                }
            });
        } else {
            // Process agents sequentially
            for agent_name in &agents {
                if let Err(e) = self.update_agent_shared_knowledge(agent_name) {
                    warn!("Failed to update shared knowledge for agent {}: {}", agent_name, e);
                }
            }
        }
        
        // Update cross references if enabled
        if self.config.enable_cross_references {
            self.update_cross_references()?;
        }
        
        // Update statistics
        if let Ok(mut stats_write) = self.stats.write() {
            stats_write.total_segments = self.shared_segments.len();
            stats_write.cross_references = self.count_cross_references();
            stats_write.last_update = Some(SystemTime::now());
            
            // Update segments by agent
            stats_write.segments_by_agent.clear();
            for entry in self.agent_segments.iter() {
                stats_write.segments_by_agent.insert(entry.key().clone(), entry.value().len());
            }
            
            // Update segments by type
            stats_write.segments_by_type.clear();
            for entry in self.shared_segments.iter() {
                let type_name = format!("{:?}", entry.value().segment.memory_type);
                *stats_write.segments_by_type.entry(type_name).or_insert(0) += 1;
            }
        }
        
        // Save cache state
        self.save_cache()?;
        
        Ok(())
    }
    
    // Update shared knowledge for a specific agent
    fn update_agent_shared_knowledge(&self, agent_name: &str) -> Result<(), Box<dyn Error>> {
        let agent_dir = self.config.agents_dir.join(agent_name);
        let memory_file = agent_dir.join("MEMORY.md");
        let learning_file = agent_dir.join("ContinuousLearning.md");
        
        // Skip agents without memory files
        if !memory_file.exists() && !learning_file.exists() {
            return Ok(());
        }
        
        // Parse memory segments
        let mut all_segments = Vec::new();
        
        if memory_file.exists() {
            let segments = self.parser.parse_file(&memory_file)?;
            all_segments.extend(segments);
        }
        
        if learning_file.exists() {
            let segments = self.parser.parse_file(&learning_file)?;
            all_segments.extend(segments);
        }
        
        // Filter segments by tier level
        let filtered_segments: Vec<MemorySegment> = match self.config.max_tier_level {
            1 => all_segments.into_iter()
                .filter(|s| s.tier == MemoryTier::Tier1)
                .collect(),
            2 => all_segments.into_iter()
                .filter(|s| s.tier == MemoryTier::Tier1 || s.tier == MemoryTier::Tier2)
                .collect(),
            _ => all_segments,
        };
        
        // Calculate relevance scores
        let mut scored_segments: Vec<(MemorySegment, f64)> = Vec::new();
        
        if let Some(model_arc) = self.activation_models.get(agent_name) {
            if let Ok(model) = model_arc.lock() {
                // Use activation model to score segments
                for segment in filtered_segments {
                    let relevance = model.get_segment_relevance(&segment.id);
                    scored_segments.push((segment, relevance));
                }
            } else {
                // Fall back to basic scoring if lock fails
                for segment in filtered_segments {
                    let relevance = self.calculate_segment_relevance(&segment);
                    scored_segments.push((segment, relevance));
                }
            }
        } else {
            // Fall back to basic scoring if no model exists
            for segment in filtered_segments {
                let relevance = self.calculate_segment_relevance(&segment);
                scored_segments.push((segment, relevance));
            }
        }
        
        // Sort by relevance (descending)
        scored_segments.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Keep only the top segments
        let top_segments = scored_segments.into_iter()
            .take(self.config.max_shared_segments)
            .collect::<Vec<(MemorySegment, f64)>>();
        
        // Create HashSet for agent segments
        let mut agent_segment_ids = HashSet::new();
        
        // Update shared segments
        for (segment, relevance) in top_segments {
            let segment_id = segment.id.clone();
            
            // Add to agent segments set
            agent_segment_ids.insert(segment_id.clone());
            
            // Check if segment already exists in shared segments
            if self.shared_segments.contains_key(&segment_id) {
                // Update existing shared segment
                if let Some(mut shared_segment) = self.shared_segments.get_mut(&segment_id) {
                    shared_segment.segment = segment;
                    shared_segment.relevance_score = relevance;
                    shared_segment.last_accessed = SystemTime::now();
                }
            } else {
                // Create new shared segment
                let shared_segment = SharedSegment {
                    segment,
                    source_agent: agent_name.to_string(),
                    shared_with: HashSet::new(),
                    relevance_score: relevance,
                    last_accessed: SystemTime::now(),
                    access_count: 0,
                    cross_references: Vec::new(),
                };
                
                self.shared_segments.insert(segment_id.clone(), shared_segment);
            }
        }
        
        // Update agent segments mapping
        self.agent_segments.insert(agent_name.to_string(), agent_segment_ids);
        
        Ok(())
    }
    
    // Update cross references between segments
    fn update_cross_references(&self) -> Result<(), Box<dyn Error>> {
        // Clear existing cross references
        self.cross_references.clear();
        
        // Build pairs of segments to compare
        let mut segment_pairs = Vec::new();
        
        // 1. Get unique pairs of agents
        let agents: Vec<String> = self.agent_segments.iter()
            .map(|entry| entry.key().clone())
            .collect();
        
        let agent_pairs: Vec<(String, String)> = agents.iter()
            .enumerate()
            .flat_map(|(i, a)| {
                agents[i+1..].iter().map(move |b| (a.clone(), b.clone()))
            })
            .collect();
        
        // 2. For each agent pair, create segment pairs
        for (agent_a, agent_b) in agent_pairs {
            if let (Some(segments_a), Some(segments_b)) = (
                self.agent_segments.get(&agent_a),
                self.agent_segments.get(&agent_b)
            ) {
                // Create segment pairs between these agents
                for segment_id_a in segments_a.value() {
                    if let Some(segment_a) = self.shared_segments.get(segment_id_a) {
                        for segment_id_b in segments_b.value() {
                            if let Some(segment_b) = self.shared_segments.get(segment_id_b) {
                                // Only compare certain memory types for efficiency
                                if is_comparable_memory_type(segment_a.segment.memory_type, segment_b.segment.memory_type) {
                                    segment_pairs.push((
                                        segment_id_a.clone(),
                                        segment_id_b.clone(),
                                        agent_a.clone(),
                                        agent_b.clone()
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Process segment pairs in parallel if enabled
        if self.config.parallel_processing {
            let refs: Vec<Option<CrossReference>> = segment_pairs.par_iter()
                .map(|(segment_id_a, segment_id_b, agent_a, agent_b)| {
                    self.compute_cross_reference(segment_id_a, segment_id_b, agent_a, agent_b)
                })
                .collect();
            
            // Add valid cross references
            for reference in refs.into_iter().flatten() {
                self.add_cross_reference(reference);
            }
        } else {
            // Process segment pairs sequentially
            for (segment_id_a, segment_id_b, agent_a, agent_b) in segment_pairs {
                if let Some(reference) = self.compute_cross_reference(&segment_id_a, &segment_id_b, &agent_a, &agent_b) {
                    self.add_cross_reference(reference);
                }
            }
        }
        
        // Update shared segments with cross references
        for entry in self.cross_references.iter() {
            let segment_id = entry.key();
            let references = entry.value();
            
            if let Some(mut shared_segment) = self.shared_segments.get_mut(segment_id) {
                shared_segment.cross_references = references.iter()
                    .map(|r| r.target_segment_id.clone())
                    .collect();
            }
        }
        
        Ok(())
    }
    
    // Compute a cross reference between two segments
    fn compute_cross_reference(&self, segment_id_a: &str, segment_id_b: &str, agent_a: &str, agent_b: &str) -> Option<CrossReference> {
        if let (Some(segment_a), Some(segment_b)) = (
            self.shared_segments.get(segment_id_a),
            self.shared_segments.get(segment_id_b)
        ) {
            // Calculate similarity between segments
            let similarity = calculate_text_similarity(&segment_a.segment.content, &segment_b.segment.content);
            
            // Only create cross reference if similarity is significant
            if similarity > 0.3 {
                let reference = CrossReference {
                    source_segment_id: segment_id_a.to_string(),
                    target_segment_id: segment_id_b.to_string(),
                    source_agent: agent_a.to_string(),
                    target_agent: agent_b.to_string(),
                    relevance: similarity,
                    created_at: SystemTime::now(),
                };
                
                return Some(reference);
            }
        }
        
        None
    }
    
    // Add a cross reference to the cache
    fn add_cross_reference(&self, reference: CrossReference) {
        let source_id = reference.source_segment_id.clone();
        
        if !self.cross_references.contains_key(&source_id) {
            self.cross_references.insert(source_id.clone(), Vec::new());
        }
        
        if let Some(mut refs) = self.cross_references.get_mut(&source_id) {
            refs.push(reference);
            
            // Sort by relevance (descending)
            refs.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());
            
            // Limit to top 10 references
            if refs.len() > 10 {
                *refs = refs.iter().take(10).cloned().collect();
            }
        }
    }
    
    // Count total cross references
    fn count_cross_references(&self) -> usize {
        self.cross_references.iter()
            .map(|entry| entry.value().len())
            .sum()
    }
    
    // Calculate relevance of a segment for sharing
    fn calculate_segment_relevance(&self, segment: &MemorySegment) -> f64 {
        // Base score depends on tier
        let tier_score = match segment.tier {
            MemoryTier::Tier1 => 1.0,
            MemoryTier::Tier2 => 0.7,
            MemoryTier::Tier3 => 0.3,
        };
        
        // Adjust by memory type
        let type_multiplier = match segment.memory_type {
            MemoryType::Identity => 0.9,  // Identity is mostly agent-specific
            MemoryType::Semantic => 1.2,  // Semantic knowledge is highly shareable
            MemoryType::Procedural => 1.1, // Procedural knowledge is shareable
            MemoryType::Relational => 1.3, // Relationships are highly valuable to share
            MemoryType::Episodic => 0.6,  // Episodic memories are mostly agent-specific
        };
        
        // Content length factor (favor shorter, focused segments)
        let length = segment.content.len() as f64;
        let length_factor = 1.0 / (1.0 + length / 5000.0);
        
        // Domain factor (certain domains are more shareable)
        let domain_factor = if segment.domains.iter().any(|d| 
            d == "system" || d == "knowledge" || d == "technical" || d == "framework"
        ) {
            1.2
        } else {
            1.0
        };
        
        // Combine factors
        tier_score * type_multiplier * length_factor * domain_factor
    }
    
    // Get shared segments for an agent
    pub fn get_shared_segments_for_agent(&self, agent_name: &str, max_segments: usize) -> Vec<SharedSegment> {
        let start_time = Instant::now();
        let mut result = Vec::new();
        
        // First, get segments from this agent
        if let Some(segment_ids) = self.agent_segments.get(agent_name) {
            for segment_id in segment_ids.value() {
                if let Some(segment) = self.shared_segments.get(segment_id) {
                    result.push(segment.clone());
                    
                    // Update access stats
                    if let Some(mut segment) = self.shared_segments.get_mut(segment_id) {
                        segment.last_accessed = SystemTime::now();
                        segment.access_count += 1;
                    }
                }
            }
        }
        
        // Then, add cross-referenced segments from other agents
        for shared_segment in &result {
            for cross_ref_id in &shared_segment.cross_references {
                if let Some(ref_segment) = self.shared_segments.get(cross_ref_id) {
                    // Skip if already in result
                    if result.iter().any(|s| s.segment.id == ref_segment.segment.id) {
                        continue;
                    }
                    
                    result.push(ref_segment.clone());
                    
                    // Update access stats
                    if let Some(mut segment) = self.shared_segments.get_mut(cross_ref_id) {
                        segment.last_accessed = SystemTime::now();
                        segment.access_count += 1;
                        
                        // Add current agent to shared_with set
                        segment.shared_with.insert(agent_name.to_string());
                    }
                }
            }
        }
        
        // Sort by relevance
        result.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        
        // Limit to max_segments
        let limited_result = result.into_iter().take(max_segments).collect();
        
        // Update stats
        let elapsed = start_time.elapsed().as_nanos();
        if let Ok(mut stats) = self.stats.write() {
            stats.cache_hits += 1;
            stats.avg_lookup_time_ns = 
                ((stats.avg_lookup_time_ns * (stats.cache_hits - 1)) + elapsed as u64) / stats.cache_hits;
        }
        
        limited_result
    }
    
    // Get a specific shared segment
    pub fn get_shared_segment(&self, segment_id: &str) -> Option<SharedSegment> {
        let start_time = Instant::now();
        
        let result = self.shared_segments.get(segment_id).map(|segment| {
            // Update access stats
            if let Some(mut segment_mut) = self.shared_segments.get_mut(segment_id) {
                segment_mut.last_accessed = SystemTime::now();
                segment_mut.access_count += 1;
            }
            
            segment.clone()
        });
        
        // Update stats
        let elapsed = start_time.elapsed().as_nanos();
        if let Ok(mut stats) = self.stats.write() {
            if result.is_some() {
                stats.cache_hits += 1;
            } else {
                stats.cache_misses += 1;
            }
            stats.avg_lookup_time_ns = 
                ((stats.avg_lookup_time_ns * (stats.cache_hits + stats.cache_misses - 1)) + elapsed as u64) 
                / (stats.cache_hits + stats.cache_misses);
        }
        
        result
    }
    
    // Get cross references for a segment
    pub fn get_cross_references(&self, segment_id: &str) -> Vec<CrossReference> {
        self.cross_references.get(segment_id)
            .map(|refs| refs.clone())
            .unwrap_or_default()
    }
    
    // Get all agents in the system
    pub fn get_all_agents(&self) -> Vec<String> {
        self.agent_segments.iter()
            .map(|entry| entry.key().clone())
            .collect()
    }
    
    // Get statistics
    pub fn get_statistics(&self) -> GlobalCacheStats {
        if let Ok(stats) = self.stats.read() {
            stats.clone()
        } else {
            GlobalCacheStats::default()
        }
    }
    
    // Format report
    pub fn format_report(&self) -> String {
        let stats = self.get_statistics();
        
        let mut report = String::new();
        report.push_str("# Global Context Cache Report\n\n");
        
        // Format timestamp
        let timestamp = stats.last_update.map(|t| {
            let dt: chrono::DateTime<chrono::Local> = t.into();
            dt.format("%Y-%m-%d %H:%M:%S").to_string()
        }).unwrap_or_else(|| "Never".to_string());
        
        report.push_str(&format!("Last updated: {}\n\n", timestamp));
        
        // Overall statistics
        report.push_str("## Overall Statistics\n\n");
        report.push_str(&format!("Total segments: {}\n", stats.total_segments));
        report.push_str(&format!("Cross references: {}\n", stats.cross_references));
        report.push_str(&format!("Cache hits: {}\n", stats.cache_hits));
        report.push_str(&format!("Cache misses: {}\n", stats.cache_misses));
        report.push_str(&format!("Average lookup time: {}ns\n\n", stats.avg_lookup_time_ns));
        
        // Segments by agent
        report.push_str("## Segments by Agent\n\n");
        report.push_str("| Agent | Segments |\n");
        report.push_str("|-------|----------|\n");
        
        let mut agents: Vec<(&String, &usize)> = stats.segments_by_agent.iter().collect();
        agents.sort_by(|a, b| b.1.cmp(a.1));
        
        for (agent, count) in agents {
            report.push_str(&format!("| {} | {} |\n", agent, count));
        }
        
        report.push_str("\n");
        
        // Segments by type
        report.push_str("## Segments by Type\n\n");
        report.push_str("| Type | Segments |\n");
        report.push_str("|------|----------|\n");
        
        let mut types: Vec<(&String, &usize)> = stats.segments_by_type.iter().collect();
        types.sort_by(|a, b| b.1.cmp(a.1));
        
        for (type_name, count) in types {
            report.push_str(&format!("| {} | {} |\n", type_name, count));
        }
        
        report
    }
}

// Helper function to determine if two memory types should be compared
fn is_comparable_memory_type(type_a: MemoryType, type_b: MemoryType) -> bool {
    match (type_a, type_b) {
        // Identity segments typically don't cross-reference
        (MemoryType::Identity, MemoryType::Identity) => false,
        (MemoryType::Identity, _) => false,
        (_, MemoryType::Identity) => false,
        
        // Episodic memories are usually agent-specific
        (MemoryType::Episodic, MemoryType::Episodic) => false,
        
        // These combinations are most valuable for cross-references
        (MemoryType::Semantic, MemoryType::Semantic) => true,
        (MemoryType::Procedural, MemoryType::Procedural) => true,
        (MemoryType::Relational, MemoryType::Relational) => true,
        (MemoryType::Semantic, MemoryType::Relational) => true,
        (MemoryType::Relational, MemoryType::Semantic) => true,
        (MemoryType::Procedural, MemoryType::Relational) => true,
        (MemoryType::Relational, MemoryType::Procedural) => true,
        
        // Other combinations are less valuable but still worth comparing
        _ => true,
    }
}

// Helper function to calculate text similarity
fn calculate_text_similarity(text1: &str, text2: &str) -> f64 {
    // Extract words from both texts
    let words1: HashSet<String> = text1
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    
    let words2: HashSet<String> = text2
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    
    // Calculate Jaccard similarity
    let intersection = words1.intersection(&words2).count();
    let union = words1.union(&words2).count();
    
    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

// Background helper function for thread
fn update_shared_knowledge(
    config: &GlobalContextCacheConfig,
    parser: &MemorySegmentParser,
    shared_segments: &Arc<DashMap<String, SharedSegment>>,
    agent_segments: &Arc<DashMap<String, HashSet<String>>>,
    cross_references: &Arc<DashMap<String, Vec<CrossReference>>>,
    activation_models: &Arc<DashMap<String, Arc<Mutex<MemoryActivationModel>>>>,
    stats: &Arc<RwLock<GlobalCacheStats>>
) {
    // Get list of agents
    let agents_dir = &config.agents_dir;
    if let Ok(entries) = fs::read_dir(agents_dir) {
        let agents = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect::<Vec<String>>();
        
        if config.parallel_processing {
            // Process agents in parallel
            agents.par_iter().for_each(|agent_name| {
                if let Err(e) = update_agent_shared_knowledge(
                    agent_name,
                    config,
                    parser,
                    shared_segments,
                    agent_segments,
                    activation_models
                ) {
                    warn!("Failed to update shared knowledge for agent {}: {}", agent_name, e);
                }
            });
        } else {
            // Process agents sequentially
            for agent_name in &agents {
                if let Err(e) = update_agent_shared_knowledge(
                    agent_name,
                    config,
                    parser,
                    shared_segments,
                    agent_segments,
                    activation_models
                ) {
                    warn!("Failed to update shared knowledge for agent {}: {}", agent_name, e);
                }
            }
        }
        
        // Update cross references if enabled
        if config.enable_cross_references {
            if let Err(e) = update_cross_references(
                config,
                shared_segments,
                agent_segments,
                cross_references
            ) {
                warn!("Failed to update cross references: {}", e);
            }
        }
        
        // Update statistics
        if let Ok(mut stats_write) = stats.write() {
            stats_write.total_segments = shared_segments.len();
            stats_write.cross_references = count_cross_references(cross_references);
            stats_write.last_update = Some(SystemTime::now());
            
            // Update segments by agent
            stats_write.segments_by_agent.clear();
            for entry in agent_segments.iter() {
                stats_write.segments_by_agent.insert(entry.key().clone(), entry.value().len());
            }
            
            // Update segments by type
            stats_write.segments_by_type.clear();
            for entry in shared_segments.iter() {
                let type_name = format!("{:?}", entry.value().segment.memory_type);
                *stats_write.segments_by_type.entry(type_name).or_insert(0) += 1;
            }
        }
    }
}

// Helper function for updating agent shared knowledge in thread
fn update_agent_shared_knowledge(
    agent_name: &str,
    config: &GlobalContextCacheConfig,
    parser: &MemorySegmentParser,
    shared_segments: &Arc<DashMap<String, SharedSegment>>,
    agent_segments: &Arc<DashMap<String, HashSet<String>>>,
    activation_models: &Arc<DashMap<String, Arc<Mutex<MemoryActivationModel>>>>
) -> Result<(), Box<dyn Error>> {
    let agent_dir = config.agents_dir.join(agent_name);
    let memory_file = agent_dir.join("MEMORY.md");
    let learning_file = agent_dir.join("ContinuousLearning.md");
    
    // Skip agents without memory files
    if !memory_file.exists() && !learning_file.exists() {
        return Ok(());
    }
    
    // Parse memory segments
    let mut all_segments = Vec::new();
    
    if memory_file.exists() {
        let segments = parser.parse_file(&memory_file)?;
        all_segments.extend(segments);
    }
    
    if learning_file.exists() {
        let segments = parser.parse_file(&learning_file)?;
        all_segments.extend(segments);
    }
    
    // Filter segments by tier level
    let filtered_segments: Vec<MemorySegment> = match config.max_tier_level {
        1 => all_segments.into_iter()
            .filter(|s| s.tier == MemoryTier::Tier1)
            .collect(),
        2 => all_segments.into_iter()
            .filter(|s| s.tier == MemoryTier::Tier1 || s.tier == MemoryTier::Tier2)
            .collect(),
        _ => all_segments,
    };
    
    // Calculate relevance scores
    let mut scored_segments: Vec<(MemorySegment, f64)> = Vec::new();
    
    if let Some(model_arc) = activation_models.get(agent_name) {
        if let Ok(model) = model_arc.lock() {
            // Use activation model to score segments
            for segment in filtered_segments {
                let relevance = model.get_segment_relevance(&segment.id);
                scored_segments.push((segment, relevance));
            }
        } else {
            // Fall back to basic scoring if lock fails
            for segment in filtered_segments {
                let relevance = calculate_basic_relevance(&segment);
                scored_segments.push((segment, relevance));
            }
        }
    } else {
        // Fall back to basic scoring if no model exists
        for segment in filtered_segments {
            let relevance = calculate_basic_relevance(&segment);
            scored_segments.push((segment, relevance));
        }
    }
    
    // Sort by relevance (descending)
    scored_segments.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    // Keep only the top segments
    let top_segments = scored_segments.into_iter()
        .take(config.max_shared_segments)
        .collect::<Vec<(MemorySegment, f64)>>();
    
    // Create HashSet for agent segments
    let mut agent_segment_ids = HashSet::new();
    
    // Update shared segments
    for (segment, relevance) in top_segments {
        let segment_id = segment.id.clone();
        
        // Add to agent segments set
        agent_segment_ids.insert(segment_id.clone());
        
        // Check if segment already exists in shared segments
        if shared_segments.contains_key(&segment_id) {
            // Update existing shared segment
            if let Some(mut shared_segment) = shared_segments.get_mut(&segment_id) {
                shared_segment.segment = segment;
                shared_segment.relevance_score = relevance;
                shared_segment.last_accessed = SystemTime::now();
            }
        } else {
            // Create new shared segment
            let shared_segment = SharedSegment {
                segment,
                source_agent: agent_name.to_string(),
                shared_with: HashSet::new(),
                relevance_score: relevance,
                last_accessed: SystemTime::now(),
                access_count: 0,
                cross_references: Vec::new(),
            };
            
            shared_segments.insert(segment_id.clone(), shared_segment);
        }
    }
    
    // Update agent segments mapping
    agent_segments.insert(agent_name.to_string(), agent_segment_ids);
    
    Ok(())
}

// Helper function for updating cross references in thread
fn update_cross_references(
    config: &GlobalContextCacheConfig,
    shared_segments: &Arc<DashMap<String, SharedSegment>>,
    agent_segments: &Arc<DashMap<String, HashSet<String>>>,
    cross_references: &Arc<DashMap<String, Vec<CrossReference>>>
) -> Result<(), Box<dyn Error>> {
    // Clear existing cross references
    cross_references.clear();
    
    // Build pairs of segments to compare
    let mut segment_pairs = Vec::new();
    
    // 1. Get unique pairs of agents
    let agents: Vec<String> = agent_segments.iter()
        .map(|entry| entry.key().clone())
        .collect();
    
    let agent_pairs: Vec<(String, String)> = agents.iter()
        .enumerate()
        .flat_map(|(i, a)| {
            agents[i+1..].iter().map(move |b| (a.clone(), b.clone()))
        })
        .collect();
    
    // 2. For each agent pair, create segment pairs
    for (agent_a, agent_b) in agent_pairs {
        if let (Some(segments_a), Some(segments_b)) = (
            agent_segments.get(&agent_a),
            agent_segments.get(&agent_b)
        ) {
            // Create segment pairs between these agents
            for segment_id_a in segments_a.value() {
                if let Some(segment_a) = shared_segments.get(segment_id_a) {
                    for segment_id_b in segments_b.value() {
                        if let Some(segment_b) = shared_segments.get(segment_id_b) {
                            // Only compare certain memory types for efficiency
                            if is_comparable_memory_type(segment_a.segment.memory_type, segment_b.segment.memory_type) {
                                segment_pairs.push((
                                    segment_id_a.clone(),
                                    segment_id_b.clone(),
                                    agent_a.clone(),
                                    agent_b.clone()
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Process segment pairs in parallel if enabled
    if config.parallel_processing {
        let refs: Vec<Option<CrossReference>> = segment_pairs.par_iter()
            .map(|(segment_id_a, segment_id_b, agent_a, agent_b)| {
                compute_cross_reference(
                    segment_id_a, 
                    segment_id_b, 
                    agent_a, 
                    agent_b,
                    shared_segments
                )
            })
            .collect();
        
        // Add valid cross references
        for reference in refs.into_iter().flatten() {
            add_cross_reference(reference, cross_references);
        }
    } else {
        // Process segment pairs sequentially
        for (segment_id_a, segment_id_b, agent_a, agent_b) in segment_pairs {
            if let Some(reference) = compute_cross_reference(
                &segment_id_a, 
                &segment_id_b, 
                &agent_a, 
                &agent_b,
                shared_segments
            ) {
                add_cross_reference(reference, cross_references);
            }
        }
    }
    
    // Update shared segments with cross references
    for entry in cross_references.iter() {
        let segment_id = entry.key();
        let references = entry.value();
        
        if let Some(mut shared_segment) = shared_segments.get_mut(segment_id) {
            shared_segment.cross_references = references.iter()
                .map(|r| r.target_segment_id.clone())
                .collect();
        }
    }
    
    Ok(())
}

// Helper function to compute cross reference in thread
fn compute_cross_reference(
    segment_id_a: &str, 
    segment_id_b: &str, 
    agent_a: &str, 
    agent_b: &str,
    shared_segments: &Arc<DashMap<String, SharedSegment>>
) -> Option<CrossReference> {
    if let (Some(segment_a), Some(segment_b)) = (
        shared_segments.get(segment_id_a),
        shared_segments.get(segment_id_b)
    ) {
        // Calculate similarity between segments
        let similarity = calculate_text_similarity(&segment_a.segment.content, &segment_b.segment.content);
        
        // Only create cross reference if similarity is significant
        if similarity > 0.3 {
            let reference = CrossReference {
                source_segment_id: segment_id_a.to_string(),
                target_segment_id: segment_id_b.to_string(),
                source_agent: agent_a.to_string(),
                target_agent: agent_b.to_string(),
                relevance: similarity,
                created_at: SystemTime::now(),
            };
            
            return Some(reference);
        }
    }
    
    None
}

// Helper function to add cross reference in thread
fn add_cross_reference(
    reference: CrossReference,
    cross_references: &Arc<DashMap<String, Vec<CrossReference>>>
) {
    let source_id = reference.source_segment_id.clone();
    
    if !cross_references.contains_key(&source_id) {
        cross_references.insert(source_id.clone(), Vec::new());
    }
    
    if let Some(mut refs) = cross_references.get_mut(&source_id) {
        refs.push(reference);
        
        // Sort by relevance (descending)
        refs.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());
        
        // Limit to top 10 references
        if refs.len() > 10 {
            *refs = refs.iter().take(10).cloned().collect();
        }
    }
}

// Helper function to count cross references in thread
fn count_cross_references(
    cross_references: &Arc<DashMap<String, Vec<CrossReference>>>
) -> usize {
    cross_references.iter()
        .map(|entry| entry.value().len())
        .sum()
}

// Helper function to calculate basic relevance in thread
fn calculate_basic_relevance(segment: &MemorySegment) -> f64 {
    // Base score depends on tier
    let tier_score = match segment.tier {
        MemoryTier::Tier1 => 1.0,
        MemoryTier::Tier2 => 0.7,
        MemoryTier::Tier3 => 0.3,
    };
    
    // Adjust by memory type
    let type_multiplier = match segment.memory_type {
        MemoryType::Identity => 0.9,  // Identity is mostly agent-specific
        MemoryType::Semantic => 1.2,  // Semantic knowledge is highly shareable
        MemoryType::Procedural => 1.1, // Procedural knowledge is shareable
        MemoryType::Relational => 1.3, // Relationships are highly valuable to share
        MemoryType::Episodic => 0.6,  // Episodic memories are mostly agent-specific
    };
    
    // Content length factor (favor shorter, focused segments)
    let length = segment.content.len() as f64;
    let length_factor = 1.0 / (1.0 + length / 5000.0);
    
    // Domain factor (certain domains are more shareable)
    let domain_factor = if segment.domains.iter().any(|d| 
        d == "system" || d == "knowledge" || d == "technical" || d == "framework"
    ) {
        1.2
    } else {
        1.0
    };
    
    // Combine factors
    tier_score * type_multiplier * length_factor * domain_factor
}

// Helper function to save cache state in thread
fn save_cache_state(
    cache_dir: &Path,
    shared_segments: &Arc<DashMap<String, SharedSegment>>,
    cross_references: &Arc<DashMap<String, Vec<CrossReference>>>,
    stats: &Arc<RwLock<GlobalCacheStats>>
) {
    // Save shared segments
    let segments: Vec<SharedSegment> = shared_segments.iter()
        .map(|entry| entry.value().clone())
        .collect();
    
    if let Ok(segments_json) = serde_json::to_string_pretty(&segments) {
        let segments_file = cache_dir.join("shared_segments.json");
        let _ = fs::write(&segments_file, segments_json);
    }
    
    // Save cross references
    let mut references = HashMap::new();
    for entry in cross_references.iter() {
        references.insert(entry.key().clone(), entry.value().clone());
    }
    
    if let Ok(references_json) = serde_json::to_string_pretty(&references) {
        let references_file = cache_dir.join("cross_references.json");
        let _ = fs::write(&references_file, references_json);
    }
    
    // Save statistics
    if let Ok(stats_read) = stats.read() {
        if let Ok(stats_json) = serde_json::to_string_pretty(&*stats_read) {
            let stats_file = cache_dir.join("stats.json");
            let _ = fs::write(&stats_file, stats_json);
        }
    }
}

// Example usage function
pub fn main() -> Result<(), Box<dyn Error>> {
    // Set up logging
    env_logger::init();
    
    // Create global context cache with default config
    let config = GlobalContextCacheConfig::default();
    let mut cache = GlobalContextCache::new(config)?;
    
    // Register an activation model
    let model = Arc::new(Mutex::new(MemoryActivationModel::new()));
    cache.register_activation_model("Athena", model);
    
    // Update shared knowledge
    println!("Updating shared knowledge...");
    cache.update_shared_knowledge()?;
    
    // Start background updates
    cache.start_background_updates()?;
    
    // Get some shared segments
    let segments = cache.get_shared_segments_for_agent("Athena", 10);
    println!("Found {} shared segments for Athena", segments.len());
    
    for segment in &segments {
        println!("Segment: {} (from {})", segment.segment.id, segment.source_agent);
        println!("  Relevance: {}", segment.relevance_score);
        println!("  Cross references: {}", segment.cross_references.len());
    }
    
    // Show statistics
    let stats = cache.get_statistics();
    println!("Total segments: {}", stats.total_segments);
    println!("Cross references: {}", stats.cross_references);
    
    // Generate report
    let report = cache.format_report();
    println!("\nGlobal Context Report:\n{}", report);
    
    // Stop background updates before exiting
    cache.stop_background_updates()?;
    
    Ok(())
}