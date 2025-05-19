use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use log::{debug, info, warn, error};
use chrono::{DateTime, Local};

// Import from other crate modules
use crate::memory_segment_parser::{MemorySegment, MemorySegmentParser, MemoryTier, MemoryType};
use crate::memory_activation_model::MemoryActivationModel;
use crate::memory_loader::MemoryLoader;
use crate::memory_updater::{MemoryUpdater, MemoryUpdate, UpdateType};

// Configuration for memory consolidation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConsolidatorConfig {
    pub agents_dir: PathBuf,
    pub backup_dir: PathBuf,
    pub max_tier1_segments: usize,
    pub max_tier2_segments: usize,
    pub min_consolidation_interval_hours: u64,
    pub auto_consolidation: bool,
    pub parallel_processing: bool,
    pub create_backups: bool,
    pub verbose_logging: bool,
}

impl Default for MemoryConsolidatorConfig {
    fn default() -> Self {
        MemoryConsolidatorConfig {
            agents_dir: PathBuf::from("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS"),
            backup_dir: PathBuf::from("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CACHE/BACKUPS"),
            max_tier1_segments: 12,   // Keep Tier 1 memory focused on critical info
            max_tier2_segments: 30,   // Limit Tier 2 to prevent overloading
            min_consolidation_interval_hours: 24, // Daily consolidation
            auto_consolidation: true,
            parallel_processing: true,
            create_backups: true,
            verbose_logging: false,
        }
    }
}

// Statistics for memory consolidation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConsolidationStats {
    pub total_consolidations: u64,
    pub segments_before: HashMap<String, usize>,  // agent -> segment count
    pub segments_after: HashMap<String, usize>,   // agent -> segment count
    pub last_consolidation: HashMap<String, SystemTime>, // agent -> time
    pub consolidated_segments: HashMap<String, Vec<String>>, // agent -> segment IDs
    pub avg_consolidation_time_ms: u64,
    pub total_consolidation_time_ms: u64,
}

// Consolidation job structure
#[derive(Debug, Clone)]
struct ConsolidationJob {
    agent_name: String,
    segments: Vec<MemorySegment>,
    timestamp: SystemTime,
    priority: u8,  // Higher number = higher priority
}

// Memory consolidator implementation
pub struct MemoryConsolidator {
    config: MemoryConsolidatorConfig,
    parser: MemorySegmentParser,
    activation_model: Arc<Mutex<MemoryActivationModel>>,
    updater: Option<Arc<Mutex<MemoryUpdater>>>,
    stats: ConsolidationStats,
    consolidation_queue: VecDeque<ConsolidationJob>,
    background_thread: Option<thread::JoinHandle<()>>,
    running: Arc<RwLock<bool>>,
}

impl MemoryConsolidator {
    // Create a new memory consolidator
    pub fn new(config: MemoryConsolidatorConfig, activation_model: Arc<Mutex<MemoryActivationModel>>) -> Result<Self, Box<dyn Error>> {
        // Create backup directory if needed
        if config.create_backups {
            fs::create_dir_all(&config.backup_dir)?;
        }
        
        // Initialize parser
        let mut parser = MemorySegmentParser::new();
        
        // Register common domains
        parser.register_domain("identity", vec!["identity".to_string(), "purpose".to_string(), "core".to_string()]);
        parser.register_domain("learning", vec!["learn".to_string(), "knowledge".to_string(), "education".to_string()]);
        parser.register_domain("system", vec!["system".to_string(), "framework".to_string(), "architecture".to_string()]);
        parser.register_domain("memory", vec!["memory".to_string(), "recall".to_string(), "remember".to_string()]);
        parser.register_domain("technical", vec!["code".to_string(), "implementation".to_string(), "software".to_string()]);
        
        let consolidator = MemoryConsolidator {
            config,
            parser,
            activation_model,
            updater: None,
            stats: ConsolidationStats::default(),
            consolidation_queue: VecDeque::new(),
            background_thread: None,
            running: Arc::new(RwLock::new(false)),
        };
        
        Ok(consolidator)
    }
    
    // Connect memory updater
    pub fn connect_updater(&mut self, updater: Arc<Mutex<MemoryUpdater>>) {
        self.updater = Some(updater);
    }
    
    // Start background consolidation thread
    pub fn start_background_consolidation(&mut self) -> Result<(), Box<dyn Error>> {
        if self.config.auto_consolidation {
            // Signal that consolidation is running
            *self.running.write().unwrap() = true;
            
            // Clone references needed for the thread
            let running = self.running.clone();
            let consolidation_interval = Duration::from_secs(3600 * self.config.min_consolidation_interval_hours);
            let config = self.config.clone();
            let activation_model = self.activation_model.clone();
            let updater = self.updater.clone();
            
            // Create statistics storage
            let stats = Arc::new(Mutex::new(self.stats.clone()));
            
            // Spawn the background thread
            let handle = thread::spawn(move || {
                let mut last_check = SystemTime::now();
                
                // Create local parser
                let mut parser = MemorySegmentParser::new();
                parser.register_domain("identity", vec!["identity".to_string(), "purpose".to_string(), "core".to_string()]);
                parser.register_domain("learning", vec!["learn".to_string(), "knowledge".to_string(), "education".to_string()]);
                parser.register_domain("system", vec!["system".to_string(), "framework".to_string(), "architecture".to_string()]);
                parser.register_domain("memory", vec!["memory".to_string(), "recall".to_string(), "remember".to_string()]);
                parser.register_domain("technical", vec!["code".to_string(), "implementation".to_string(), "software".to_string()]);
                
                while *running.read().unwrap() {
                    // Sleep for a while to avoid cpu spinning
                    thread::sleep(Duration::from_secs(60));
                    
                    // Check if it's time to do consolidation
                    let now = SystemTime::now();
                    if let Ok(duration) = now.duration_since(last_check) {
                        if duration < consolidation_interval {
                            continue;
                        }
                    }
                    
                    // Update last check time
                    last_check = now;
                    
                    // Get list of agents
                    let agents_dir = &config.agents_dir;
                    let agents = match fs::read_dir(agents_dir) {
                        Ok(entries) => {
                            entries
                                .filter_map(|e| e.ok())
                                .filter(|e| e.path().is_dir())
                                .filter_map(|e| e.file_name().into_string().ok())
                                .collect::<Vec<String>>()
                        },
                        Err(e) => {
                            error!("Failed to read agents directory: {}", e);
                            continue;
                        }
                    };
                    
                    // Process each agent
                    for agent_name in agents {
                        // Skip consolidation if it's too soon
                        if let Ok(mut stats_lock) = stats.lock() {
                            if let Some(last_time) = stats_lock.last_consolidation.get(&agent_name) {
                                if let Ok(duration) = now.duration_since(*last_time) {
                                    if duration < consolidation_interval {
                                        continue;
                                    }
                                }
                            }
                        }
                        
                        // Get agent memory
                        let agent_dir = agents_dir.join(&agent_name);
                        let memory_file = agent_dir.join("MEMORY.md");
                        let learning_file = agent_dir.join("ContinuousLearning.md");
                        
                        // Skip agents without memory files
                        if !memory_file.exists() && !learning_file.exists() {
                            continue;
                        }
                        
                        info!("Background consolidation for agent: {}", agent_name);
                        
                        // Parse memory segments
                        let mut all_segments = Vec::new();
                        
                        if memory_file.exists() {
                            if let Ok(segments) = parser.parse_file(&memory_file) {
                                all_segments.extend(segments);
                            }
                        }
                        
                        if learning_file.exists() {
                            if let Ok(segments) = parser.parse_file(&learning_file) {
                                all_segments.extend(segments);
                            }
                        }
                        
                        // Track segment count before consolidation
                        if let Ok(mut stats_lock) = stats.lock() {
                            stats_lock.segments_before.insert(agent_name.clone(), all_segments.len());
                        }
                        
                        // Only proceed if we have enough segments to make consolidation worthwhile
                        if all_segments.len() <= (config.max_tier1_segments + config.max_tier2_segments) {
                            continue;
                        }
                        
                        // Perform consolidation
                        let start_time = Instant::now();
                        let mut consolidated_ids = Vec::new();
                        
                        // Consolidate memory using the activation model
                        if let Ok(model) = activation_model.lock() {
                            // Create a map of segment IDs to their segments
                            let mut segment_map = HashMap::new();
                            for segment in &all_segments {
                                segment_map.insert(segment.id.clone(), segment.clone());
                            }
                            
                            // Get activation scores for segments
                            let mut segment_scores = Vec::new();
                            for segment in &all_segments {
                                let relevance = model.get_segment_relevance(&segment.id);
                                segment_scores.push((segment.id.clone(), relevance));
                            }
                            
                            // Sort by relevance score (descending)
                            segment_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                            
                            // Keep only the top segments
                            let mut tier1_count = 0;
                            let mut tier2_count = 0;
                            let mut segments_to_keep = HashSet::new();
                            
                            for (segment_id, _) in segment_scores {
                                if let Some(segment) = segment_map.get(&segment_id) {
                                    match segment.tier {
                                        MemoryTier::Tier1 => {
                                            if tier1_count < config.max_tier1_segments {
                                                segments_to_keep.insert(segment_id.clone());
                                                tier1_count += 1;
                                            } else {
                                                consolidated_ids.push(segment_id.clone());
                                            }
                                        },
                                        MemoryTier::Tier2 => {
                                            if tier2_count < config.max_tier2_segments {
                                                segments_to_keep.insert(segment_id.clone());
                                                tier2_count += 1;
                                            } else {
                                                consolidated_ids.push(segment_id.clone());
                                            }
                                        },
                                        MemoryTier::Tier3 => {
                                            // Keep all Tier3 segments (they're rarely loaded anyway)
                                            segments_to_keep.insert(segment_id.clone());
                                        }
                                    }
                                }
                            }
                            
                            // If we have an updater, remove the consolidated segments
                            if let Some(updater_arc) = &updater {
                                if let Ok(mut updater) = updater_arc.lock() {
                                    for segment_id in &consolidated_ids {
                                        if let Some(segment) = segment_map.get(segment_id) {
                                            // Create update to remove the segment
                                            let update = updater.create_update(
                                                UpdateType::Remove,
                                                Some(segment.clone()),
                                                0.5, // Medium importance
                                                "memory_consolidator",
                                                None
                                            );
                                            
                                            // Queue the update
                                            if let Err(e) = updater.queue_update(&agent_name, update) {
                                                error!("Failed to queue update for agent {}: {}", agent_name, e);
                                            }
                                        }
                                    }
                                    
                                    // Process the updates
                                    if let Err(e) = updater.process_pending_updates(&agent_name) {
                                        error!("Failed to process updates for agent {}: {}", agent_name, e);
                                    } else {
                                        info!("Successfully processed memory consolidation for agent {}", agent_name);
                                        
                                        // Track statistics
                                        if let Ok(mut stats_lock) = stats.lock() {
                                            stats_lock.total_consolidations += 1;
                                            
                                            // Calculate segments after consolidation
                                            let segments_after = all_segments.len() - consolidated_ids.len();
                                            stats_lock.segments_after.insert(agent_name.clone(), segments_after);
                                            
                                            // Track consolidated segments
                                            stats_lock.consolidated_segments.insert(agent_name.clone(), consolidated_ids.clone());
                                            
                                            // Track last consolidation time
                                            stats_lock.last_consolidation.insert(agent_name.clone(), SystemTime::now());
                                            
                                            // Track time
                                            let elapsed_ms = start_time.elapsed().as_millis() as u64;
                                            stats_lock.total_consolidation_time_ms += elapsed_ms;
                                            stats_lock.avg_consolidation_time_ms = 
                                                stats_lock.total_consolidation_time_ms / stats_lock.total_consolidations;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                info!("Background consolidation thread exiting");
            });
            
            self.background_thread = Some(handle);
            
            info!("Background consolidation started");
        }
        
        Ok(())
    }
    
    // Stop background consolidation
    pub fn stop_background_consolidation(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(_) = self.background_thread {
            // Signal the consolidation to stop
            *self.running.write().unwrap() = false;
            
            // Wait for the thread to finish
            if let Some(handle) = self.background_thread.take() {
                handle.join().map_err(|_| "Failed to join consolidation thread")?;
            }
            
            info!("Background consolidation stopped");
        }
        
        Ok(())
    }
    
    // Queue a consolidation job
    pub fn queue_consolidation(&mut self, agent_name: &str, priority: u8) -> Result<(), Box<dyn Error>> {
        // Get agent memory
        let agent_dir = self.config.agents_dir.join(agent_name);
        let memory_file = agent_dir.join("MEMORY.md");
        let learning_file = agent_dir.join("ContinuousLearning.md");
        
        // Skip agents without memory files
        if !memory_file.exists() && !learning_file.exists() {
            return Err(format!("No memory files found for agent: {}", agent_name).into());
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
        
        // Create consolidation job
        let job = ConsolidationJob {
            agent_name: agent_name.to_string(),
            segments: all_segments,
            timestamp: SystemTime::now(),
            priority,
        };
        
        // Add to queue based on priority
        if priority > 127 {
            // High priority: add to front of queue
            self.consolidation_queue.push_front(job);
        } else {
            // Normal priority: add to back of queue
            self.consolidation_queue.push_back(job);
        }
        
        Ok(())
    }
    
    // Process the next consolidation job in the queue
    pub fn process_next_job(&mut self) -> Result<bool, Box<dyn Error>> {
        // Check if queue is empty
        if self.consolidation_queue.is_empty() {
            return Ok(false);
        }
        
        // Get the next job
        let job = match self.consolidation_queue.pop_front() {
            Some(j) => j,
            None => return Ok(false),
        };
        
        info!("Processing consolidation job for agent: {}", job.agent_name);
        
        // Track segment count before consolidation
        self.stats.segments_before.insert(job.agent_name.clone(), job.segments.len());
        
        // Only proceed if we have enough segments to make consolidation worthwhile
        if job.segments.len() <= (self.config.max_tier1_segments + self.config.max_tier2_segments) {
            info!("Skipping consolidation for agent {}: Not enough segments", job.agent_name);
            return Ok(false);
        }
        
        // Perform consolidation
        let start_time = Instant::now();
        let mut consolidated_ids = Vec::new();
        
        // Consolidate memory using the activation model
        if let Ok(model) = self.activation_model.lock() {
            // Create a map of segment IDs to their segments
            let mut segment_map = HashMap::new();
            for segment in &job.segments {
                segment_map.insert(segment.id.clone(), segment.clone());
            }
            
            // Get activation scores for segments
            let mut segment_scores = Vec::new();
            for segment in &job.segments {
                let relevance = model.get_segment_relevance(&segment.id);
                segment_scores.push((segment.id.clone(), relevance));
            }
            
            // Sort by relevance score (descending)
            segment_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            
            // Keep only the top segments
            let mut tier1_count = 0;
            let mut tier2_count = 0;
            let mut segments_to_keep = HashSet::new();
            
            for (segment_id, _) in segment_scores {
                if let Some(segment) = segment_map.get(&segment_id) {
                    match segment.tier {
                        MemoryTier::Tier1 => {
                            if tier1_count < self.config.max_tier1_segments {
                                segments_to_keep.insert(segment_id.clone());
                                tier1_count += 1;
                            } else {
                                consolidated_ids.push(segment_id.clone());
                            }
                        },
                        MemoryTier::Tier2 => {
                            if tier2_count < self.config.max_tier2_segments {
                                segments_to_keep.insert(segment_id.clone());
                                tier2_count += 1;
                            } else {
                                consolidated_ids.push(segment_id.clone());
                            }
                        },
                        MemoryTier::Tier3 => {
                            // Keep all Tier3 segments (they're rarely loaded anyway)
                            segments_to_keep.insert(segment_id.clone());
                        }
                    }
                }
            }
            
            // If we have an updater, remove the consolidated segments
            if let Some(updater_arc) = &self.updater {
                if let Ok(mut updater) = updater_arc.lock() {
                    for segment_id in &consolidated_ids {
                        if let Some(segment) = segment_map.get(segment_id) {
                            // Create update to remove the segment
                            let update = updater.create_update(
                                UpdateType::Remove,
                                Some(segment.clone()),
                                0.5, // Medium importance
                                "memory_consolidator",
                                None
                            );
                            
                            // Queue the update
                            if let Err(e) = updater.queue_update(&job.agent_name, update) {
                                error!("Failed to queue update for agent {}: {}", job.agent_name, e);
                            }
                        }
                    }
                    
                    // Process the updates
                    if let Err(e) = updater.process_pending_updates(&job.agent_name) {
                        error!("Failed to process updates for agent {}: {}", job.agent_name, e);
                        return Err(format!("Failed to process updates: {}", e).into());
                    } else {
                        info!("Successfully processed memory consolidation for agent {}", job.agent_name);
                        
                        // Update statistics
                        self.stats.total_consolidations += 1;
                        
                        // Calculate segments after consolidation
                        let segments_after = job.segments.len() - consolidated_ids.len();
                        self.stats.segments_after.insert(job.agent_name.clone(), segments_after);
                        
                        // Track consolidated segments
                        self.stats.consolidated_segments.insert(job.agent_name.clone(), consolidated_ids);
                        
                        // Track last consolidation time
                        self.stats.last_consolidation.insert(job.agent_name.clone(), SystemTime::now());
                        
                        // Track time
                        let elapsed_ms = start_time.elapsed().as_millis() as u64;
                        self.stats.total_consolidation_time_ms += elapsed_ms;
                        self.stats.avg_consolidation_time_ms = 
                            self.stats.total_consolidation_time_ms / self.stats.total_consolidations;
                        
                        return Ok(true);
                    }
                }
            }
        }
        
        // If we get here, we couldn't perform consolidation
        warn!("Consolidation not performed for agent {}: No updater available", job.agent_name);
        Ok(false)
    }
    
    // Consolidate all agent memory in parallel
    pub fn consolidate_all_memory(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        let start_time = Instant::now();
        let mut consolidated_agents = Vec::new();
        
        // Get list of agents
        let agents_dir = &self.config.agents_dir;
        let agents = fs::read_dir(agents_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect::<Vec<String>>();
        
        if self.config.parallel_processing {
            // Process agents in parallel using rayon
            let results: Vec<Result<String, String>> = agents.par_iter()
                .map(|agent_name| {
                    match self.queue_consolidation(agent_name, 100) {
                        Ok(_) => {
                            match self.process_next_job() {
                                Ok(true) => Ok(agent_name.clone()),
                                Ok(false) => Err(format!("Consolidation skipped for agent {}", agent_name)),
                                Err(e) => Err(format!("Consolidation failed for agent {}: {}", agent_name, e)),
                            }
                        },
                        Err(e) => Err(format!("Failed to queue consolidation for agent {}: {}", agent_name, e)),
                    }
                })
                .collect();
            
            // Collect successful consolidations
            for result in results {
                match result {
                    Ok(agent_name) => consolidated_agents.push(agent_name),
                    Err(e) => warn!("{}", e),
                }
            }
        } else {
            // Process agents sequentially
            for agent_name in &agents {
                if let Ok(_) = self.queue_consolidation(agent_name, 100) {
                    match self.process_next_job() {
                        Ok(true) => consolidated_agents.push(agent_name.clone()),
                        Ok(false) => warn!("Consolidation skipped for agent {}", agent_name),
                        Err(e) => warn!("Consolidation failed for agent {}: {}", agent_name, e),
                    }
                }
            }
        }
        
        // Update statistics
        let elapsed_ms = start_time.elapsed().as_millis() as u64;
        info!("Consolidated memory for {} agents in {}ms", consolidated_agents.len(), elapsed_ms);
        
        Ok(consolidated_agents)
    }
    
    // Get consolidation statistics
    pub fn get_statistics(&self) -> &ConsolidationStats {
        &self.stats
    }
    
    // Reset statistics
    pub fn reset_statistics(&mut self) {
        self.stats = ConsolidationStats::default();
    }
    
    // Format consolidation report
    pub fn format_report(&self) -> String {
        let now: DateTime<Local> = SystemTime::now().into();
        let mut report = format!("# Memory Consolidation Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", now.format("%Y-%m-%d %H:%M:%S")));
        
        // Overall statistics
        report.push_str("## Overall Statistics\n\n");
        report.push_str(&format!("Total consolidations: {}\n", self.stats.total_consolidations));
        report.push_str(&format!("Average consolidation time: {}ms\n\n", self.stats.avg_consolidation_time_ms));
        
        // Agent-specific statistics
        report.push_str("## Agent Statistics\n\n");
        report.push_str("| Agent | Before | After | Reduction | Last Consolidation |\n");
        report.push_str("|-------|--------|-------|-----------|-------------------|\n");
        
        for (agent, segments_before) in &self.stats.segments_before {
            let segments_after = self.stats.segments_after.get(agent).unwrap_or(&0);
            let reduction = if *segments_before > 0 {
                (((*segments_before - *segments_after) as f64) / (*segments_before as f64) * 100.0) as u32
            } else {
                0
            };
            
            let last_time = self.stats.last_consolidation.get(agent)
                .map(|t| {
                    let dt: DateTime<Local> = (*t).into();
                    dt.format("%Y-%m-%d %H:%M").to_string()
                })
                .unwrap_or_else(|| "Never".to_string());
            
            report.push_str(&format!("| {} | {} | {} | {}% | {} |\n", 
                agent, segments_before, segments_after, reduction, last_time));
        }
        
        // Consolidated segments details
        report.push_str("\n## Consolidated Segments\n\n");
        
        for (agent, segment_ids) in &self.stats.consolidated_segments {
            report.push_str(&format!("### {}\n\n", agent));
            report.push_str(&format!("Consolidated {} segments:\n\n", segment_ids.len()));
            
            for (i, segment_id) in segment_ids.iter().enumerate().take(10) {
                report.push_str(&format!("{}. {}\n", i + 1, segment_id));
            }
            
            if segment_ids.len() > 10 {
                report.push_str(&format!("\n... and {} more\n", segment_ids.len() - 10));
            }
            
            report.push_str("\n");
        }
        
        report
    }
}

// Example usage function
pub fn main() -> Result<(), Box<dyn Error>> {
    // Set up logging
    env_logger::init();
    
    // Create activation model
    let activation_model = Arc::new(Mutex::new(MemoryActivationModel::new()));
    
    // Create consolidator with default config
    let config = MemoryConsolidatorConfig::default();
    let mut consolidator = MemoryConsolidator::new(config, activation_model)?;
    
    // Example: Consolidate all memory
    println!("Consolidating all agent memory...");
    let consolidated_agents = consolidator.consolidate_all_memory()?;
    
    println!("Consolidated memory for {} agents:", consolidated_agents.len());
    for agent in &consolidated_agents {
        println!("  - {}", agent);
    }
    
    // Show statistics
    let stats = consolidator.get_statistics();
    println!("Total consolidations: {}", stats.total_consolidations);
    println!("Average consolidation time: {}ms", stats.avg_consolidation_time_ms);
    
    // Generate report
    let report = consolidator.format_report();
    println!("\nConsolidation Report:\n{}", report);
    
    Ok(())
}