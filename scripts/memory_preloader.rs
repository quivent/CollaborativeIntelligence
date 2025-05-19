use std::collections::{HashMap, HashSet, VecDeque};
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
use chrono::{DateTime, Local};
use rand::Rng;
use regex::Regex;

// Import from other crate modules
use crate::memory_segment_parser::{MemorySegment, MemorySegmentParser, MemoryTier, MemoryType};
use crate::memory_activation_model::MemoryActivationModel;
use crate::memory_loader::MemoryLoader;

// Memory preloading configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPreloaderConfig {
    pub agents_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub history_limit: usize,           // How many activations to remember
    pub prediction_threshold: f64,      // Threshold for preloading (0.0-1.0)
    pub max_preload_agents: usize,      // Maximum agents to preload at once
    pub preload_tier_level: u8,         // 1=Tier1 only, 2=Tier1+2, 3=All
    pub adaptive_learning_rate: f64,    // How quickly system adapts (0.0-1.0)
    pub context_weight: f64,            // Weight for context in predictions
    pub time_weight: f64,               // Weight for time patterns in predictions
    pub sequence_weight: f64,           // Weight for sequential patterns
    pub concurrent_preloading: bool,    // Whether to preload in parallel
    pub enable_background: bool,        // Run in background thread
    pub preloading_interval_seconds: u64, // How often to update preloading
    pub verbose_logging: bool,
}

impl Default for MemoryPreloaderConfig {
    fn default() -> Self {
        MemoryPreloaderConfig {
            agents_dir: PathBuf::from("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS"),
            cache_dir: PathBuf::from("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CACHE/PRELOADER"),
            history_limit: 100,
            prediction_threshold: 0.65,
            max_preload_agents: 3,
            preload_tier_level: 2,
            adaptive_learning_rate: 0.2,
            context_weight: 0.5,
            time_weight: 0.3,
            sequence_weight: 0.2,
            concurrent_preloading: true,
            enable_background: true,
            preloading_interval_seconds: 300, // 5 minutes
            verbose_logging: false,
        }
    }
}

// Agent activation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationRecord {
    pub agent_name: String,
    pub timestamp: SystemTime,
    pub context: Option<String>,
    pub previous_agent: Option<String>,
    pub duration_seconds: u64,
    pub requested_by: String, // user, system, or agent name
}

// Agent transition probability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionProbability {
    pub from_agent: String,
    pub to_agent: String,
    pub probability: f64,
    pub count: u32,
    pub contexts: HashMap<String, u32>, // context -> count
}

// Context affinity record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAffinity {
    pub context: String,
    pub agent_affinities: HashMap<String, f64>, // agent -> affinity score
}

// Time pattern for activations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePattern {
    pub agent_name: String,
    pub hour_of_day: [f64; 24],         // Probability by hour
    pub day_of_week: [f64; 7],          // Probability by day
}

// Prediction result
#[derive(Debug, Clone)]
pub struct AgentPrediction {
    pub agent_name: String,
    pub probability: f64,
    pub confidence: f64,
    pub factors: HashMap<String, f64>,   // Factor -> contribution
}

// Statistics for preloader
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreloaderStats {
    pub total_activations: u64,
    pub successful_predictions: u64,
    pub prediction_accuracy: f64,
    pub preloading_count: u64,
    pub cache_hits: u64,
    pub avg_preload_time_ms: u64,
    pub last_preload: Option<SystemTime>,
    pub agent_activation_counts: HashMap<String, u64>,
    pub context_counts: HashMap<String, u64>,
}

// Memory preloader implementation
pub struct MemoryPreloader {
    config: MemoryPreloaderConfig,
    activation_history: VecDeque<ActivationRecord>,
    transition_matrix: HashMap<String, HashMap<String, TransitionProbability>>,
    context_affinities: HashMap<String, ContextAffinity>,
    time_patterns: HashMap<String, TimePattern>,
    preloaded_agents: HashSet<String>,
    currently_active_agent: Option<String>,
    memory_loader: Option<Arc<Mutex<MemoryLoader>>>,
    stats: Arc<RwLock<PreloaderStats>>,
    preload_thread: Option<thread::JoinHandle<()>>,
    running: Arc<RwLock<bool>>,
}

impl MemoryPreloader {
    // Create a new memory preloader
    pub fn new(config: MemoryPreloaderConfig) -> Result<Self, Box<dyn Error>> {
        // Create cache directory if needed
        fs::create_dir_all(&config.cache_dir)?;
        
        let mut preloader = MemoryPreloader {
            config,
            activation_history: VecDeque::with_capacity(100),
            transition_matrix: HashMap::new(),
            context_affinities: HashMap::new(),
            time_patterns: HashMap::new(),
            preloaded_agents: HashSet::new(),
            currently_active_agent: None,
            memory_loader: None,
            stats: Arc::new(RwLock::new(PreloaderStats::default())),
            preload_thread: None,
            running: Arc::new(RwLock::new(false)),
        };
        
        // Load existing preloader state
        preloader.load_state()?;
        
        Ok(preloader)
    }
    
    // Connect memory loader
    pub fn connect_memory_loader(&mut self, loader: Arc<Mutex<MemoryLoader>>) {
        self.memory_loader = Some(loader);
    }
    
    // Start background preloading thread
    pub fn start_background_preloading(&mut self) -> Result<(), Box<dyn Error>> {
        if self.config.enable_background {
            // Signal that preloading is running
            *self.running.write().unwrap() = true;
            
            // Clone references needed for the thread
            let running = self.running.clone();
            let config = self.config.clone();
            let stats = self.stats.clone();
            let memory_loader = self.memory_loader.clone();
            
            // Create thread-safe clones of our data structures
            let history_arc = Arc::new(Mutex::new(self.activation_history.clone()));
            let transitions_arc = Arc::new(Mutex::new(self.transition_matrix.clone()));
            let affinities_arc = Arc::new(Mutex::new(self.context_affinities.clone()));
            let patterns_arc = Arc::new(Mutex::new(self.time_patterns.clone()));
            let preloaded_arc = Arc::new(Mutex::new(self.preloaded_agents.clone()));
            let active_arc = Arc::new(Mutex::new(self.currently_active_agent.clone()));
            
            // Spawn the background thread
            let handle = thread::spawn(move || {
                info!("Background memory preloader started");
                
                while *running.read().unwrap() {
                    // Perform preloading
                    match predict_and_preload(
                        &config,
                        &history_arc,
                        &transitions_arc,
                        &affinities_arc,
                        &patterns_arc,
                        &preloaded_arc,
                        &active_arc,
                        &memory_loader,
                        &stats
                    ) {
                        Ok(_) => {
                            if config.verbose_logging {
                                debug!("Background preloading completed successfully");
                            }
                        },
                        Err(e) => {
                            warn!("Background preloading error: {}", e);
                        }
                    }
                    
                    // Save state periodically
                    match save_preloader_state(
                        &config.cache_dir,
                        &history_arc,
                        &transitions_arc,
                        &affinities_arc,
                        &patterns_arc,
                        &stats
                    ) {
                        Ok(_) => {
                            if config.verbose_logging {
                                debug!("Preloader state saved successfully");
                            }
                        },
                        Err(e) => {
                            warn!("Failed to save preloader state: {}", e);
                        }
                    }
                    
                    // Sleep for the preloading interval
                    for _ in 0..60 {  // Check every second if we should stop
                        if !*running.read().unwrap() {
                            break;
                        }
                        thread::sleep(Duration::from_secs(config.preloading_interval_seconds / 60));
                    }
                }
                
                info!("Background memory preloader stopped");
            });
            
            self.preload_thread = Some(handle);
        }
        
        Ok(())
    }
    
    // Stop background preloading
    pub fn stop_background_preloading(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(_) = self.preload_thread {
            // Signal the preloader to stop
            *self.running.write().unwrap() = false;
            
            // Wait for the thread to finish
            if let Some(handle) = self.preload_thread.take() {
                handle.join().map_err(|_| "Failed to join preloader thread")?;
            }
            
            // Save state one last time
            self.save_state()?;
            
            info!("Background memory preloader stopped");
        }
        
        Ok(())
    }
    
    // Load preloader state from disk
    fn load_state(&mut self) -> Result<(), Box<dyn Error>> {
        // Load activation history
        let history_file = self.config.cache_dir.join("activation_history.json");
        if history_file.exists() {
            let content = fs::read_to_string(&history_file)?;
            let history: VecDeque<ActivationRecord> = serde_json::from_str(&content)?;
            self.activation_history = history;
        }
        
        // Load transition matrix
        let transitions_file = self.config.cache_dir.join("transitions.json");
        if transitions_file.exists() {
            let content = fs::read_to_string(&transitions_file)?;
            let transitions: HashMap<String, HashMap<String, TransitionProbability>> = serde_json::from_str(&content)?;
            self.transition_matrix = transitions;
        }
        
        // Load context affinities
        let affinities_file = self.config.cache_dir.join("context_affinities.json");
        if affinities_file.exists() {
            let content = fs::read_to_string(&affinities_file)?;
            let affinities: HashMap<String, ContextAffinity> = serde_json::from_str(&content)?;
            self.context_affinities = affinities;
        }
        
        // Load time patterns
        let patterns_file = self.config.cache_dir.join("time_patterns.json");
        if patterns_file.exists() {
            let content = fs::read_to_string(&patterns_file)?;
            let patterns: HashMap<String, TimePattern> = serde_json::from_str(&content)?;
            self.time_patterns = patterns;
        }
        
        // Load statistics
        let stats_file = self.config.cache_dir.join("stats.json");
        if stats_file.exists() {
            let content = fs::read_to_string(&stats_file)?;
            let stats: PreloaderStats = serde_json::from_str(&content)?;
            
            if let Ok(mut stats_write) = self.stats.write() {
                *stats_write = stats;
            }
        }
        
        Ok(())
    }
    
    // Save preloader state to disk
    fn save_state(&self) -> Result<(), Box<dyn Error>> {
        // Save activation history
        let history_json = serde_json::to_string(&self.activation_history)?;
        let history_file = self.config.cache_dir.join("activation_history.json");
        fs::write(&history_file, history_json)?;
        
        // Save transition matrix
        let transitions_json = serde_json::to_string(&self.transition_matrix)?;
        let transitions_file = self.config.cache_dir.join("transitions.json");
        fs::write(&transitions_file, transitions_json)?;
        
        // Save context affinities
        let affinities_json = serde_json::to_string(&self.context_affinities)?;
        let affinities_file = self.config.cache_dir.join("context_affinities.json");
        fs::write(&affinities_file, affinities_json)?;
        
        // Save time patterns
        let patterns_json = serde_json::to_string(&self.time_patterns)?;
        let patterns_file = self.config.cache_dir.join("time_patterns.json");
        fs::write(&patterns_file, patterns_json)?;
        
        // Save statistics
        if let Ok(stats_read) = self.stats.read() {
            let stats_json = serde_json::to_string_pretty(&*stats_read)?;
            let stats_file = self.config.cache_dir.join("stats.json");
            fs::write(&stats_file, stats_json)?;
        }
        
        Ok(())
    }
    
    // Record an agent activation
    pub fn record_activation(&mut self, 
                            agent_name: &str, 
                            context: Option<&str>,
                            requested_by: &str) -> Result<(), Box<dyn Error>> {
        let now = SystemTime::now();
        
        // Create activation record
        let record = ActivationRecord {
            agent_name: agent_name.to_string(),
            timestamp: now,
            context: context.map(|s| s.to_string()),
            previous_agent: self.currently_active_agent.clone(),
            duration_seconds: 0, // Will be updated on next activation
            requested_by: requested_by.to_string(),
        };
        
        // Update duration for previous activation if any
        if let Some(prev_record) = self.activation_history.back_mut() {
            if let Ok(duration) = now.duration_since(prev_record.timestamp) {
                prev_record.duration_seconds = duration.as_secs();
            }
        }
        
        // Add to history
        self.activation_history.push_back(record);
        
        // Limit history size
        while self.activation_history.len() > self.config.history_limit {
            self.activation_history.pop_front();
        }
        
        // Set currently active agent
        self.currently_active_agent = Some(agent_name.to_string());
        
        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.total_activations += 1;
            
            // Update agent activation count
            *stats.agent_activation_counts
                .entry(agent_name.to_string())
                .or_insert(0) += 1;
            
            // Update context count if provided
            if let Some(ctx) = context {
                *stats.context_counts
                    .entry(ctx.to_string())
                    .or_insert(0) += 1;
            }
        }
        
        // Update transition probabilities
        self.update_transitions(agent_name, context);
        
        // Update context affinities
        if let Some(ctx) = context {
            self.update_context_affinities(agent_name, ctx);
        }
        
        // Update time patterns
        self.update_time_patterns(agent_name);
        
        // Check if we correctly predicted this activation
        self.check_prediction_accuracy(agent_name);
        
        // Predict and preload next agents
        self.predict_and_preload(context)?;
        
        Ok(())
    }
    
    // Update transition probabilities
    fn update_transitions(&mut self, agent_name: &str, context: Option<&str>) {
        // Only update if we have a previous agent
        if let Some(prev_agent) = &self.currently_active_agent {
            // Skip if activating the same agent
            if prev_agent == agent_name {
                return;
            }
            
            // Get or create from_agent transitions
            if !self.transition_matrix.contains_key(prev_agent) {
                self.transition_matrix.insert(prev_agent.clone(), HashMap::new());
            }
            
            let from_transitions = self.transition_matrix.get_mut(prev_agent).unwrap();
            
            // Get or create transition probability
            if !from_transitions.contains_key(agent_name) {
                from_transitions.insert(agent_name.to_string(), TransitionProbability {
                    from_agent: prev_agent.clone(),
                    to_agent: agent_name.to_string(),
                    probability: 0.0,
                    count: 0,
                    contexts: HashMap::new(),
                });
            }
            
            let transition = from_transitions.get_mut(agent_name).unwrap();
            
            // Update transition count
            transition.count += 1;
            
            // Update context count if provided
            if let Some(ctx) = context {
                *transition.contexts
                    .entry(ctx.to_string())
                    .or_insert(0) += 1;
            }
            
            // Recalculate probabilities for all transitions from prev_agent
            let total_count: u32 = from_transitions.values()
                .map(|t| t.count)
                .sum();
            
            for transition in from_transitions.values_mut() {
                transition.probability = transition.count as f64 / total_count as f64;
            }
        }
    }
    
    // Update context affinities
    fn update_context_affinities(&mut self, agent_name: &str, context: &str) {
        // Get or create context affinity
        if !self.context_affinities.contains_key(context) {
            self.context_affinities.insert(context.to_string(), ContextAffinity {
                context: context.to_string(),
                agent_affinities: HashMap::new(),
            });
        }
        
        let affinity = self.context_affinities.get_mut(context).unwrap();
        
        // Update affinity score
        let current_score = affinity.agent_affinities
            .entry(agent_name.to_string())
            .or_insert(0.0);
        
        // Apply adaptive learning rate
        *current_score = (*current_score * (1.0 - self.config.adaptive_learning_rate)) + 
                         (1.0 * self.config.adaptive_learning_rate);
        
        // Slightly decrease all other agent affinities for this context
        let decay_factor = 0.99;
        for (other_agent, score) in affinity.agent_affinities.iter_mut() {
            if other_agent != agent_name {
                *score *= decay_factor;
            }
        }
        
        // Normalize affinities
        let total: f64 = affinity.agent_affinities.values().sum();
        if total > 0.0 {
            for score in affinity.agent_affinities.values_mut() {
                *score /= total;
            }
        }
    }
    
    // Update time patterns
    fn update_time_patterns(&mut self, agent_name: &str) {
        // Get current time
        let now: DateTime<Local> = SystemTime::now().into();
        let hour = now.hour() as usize;
        let weekday = now.weekday().num_days_from_monday() as usize;
        
        // Get or create time pattern
        if !self.time_patterns.contains_key(agent_name) {
            self.time_patterns.insert(agent_name.to_string(), TimePattern {
                agent_name: agent_name.to_string(),
                hour_of_day: [0.0; 24],
                day_of_week: [0.0; 7],
            });
        }
        
        let pattern = self.time_patterns.get_mut(agent_name).unwrap();
        
        // Update hourly pattern
        pattern.hour_of_day[hour] += 1.0;
        
        // Update daily pattern
        pattern.day_of_week[weekday] += 1.0;
        
        // Normalize patterns
        let hour_sum: f64 = pattern.hour_of_day.iter().sum();
        if hour_sum > 0.0 {
            for val in pattern.hour_of_day.iter_mut() {
                *val /= hour_sum;
            }
        }
        
        let day_sum: f64 = pattern.day_of_week.iter().sum();
        if day_sum > 0.0 {
            for val in pattern.day_of_week.iter_mut() {
                *val /= day_sum;
            }
        }
    }
    
    // Check prediction accuracy
    fn check_prediction_accuracy(&mut self, agent_name: &str) {
        // If this agent is in preloaded set, we predicted correctly
        if self.preloaded_agents.contains(agent_name) {
            if let Ok(mut stats) = self.stats.write() {
                stats.successful_predictions += 1;
                stats.prediction_accuracy = stats.successful_predictions as f64 / stats.total_activations as f64;
            }
        }
    }
    
    // Predict next agents and preload their memory
    pub fn predict_and_preload(&mut self, context: Option<&str>) -> Result<Vec<AgentPrediction>, Box<dyn Error>> {
        let start_time = Instant::now();
        
        // Predict next agents
        let predictions = self.predict_next_agents(context)?;
        
        // Filter predictions by threshold
        let filtered_predictions: Vec<AgentPrediction> = predictions.into_iter()
            .filter(|p| p.probability >= self.config.prediction_threshold)
            .take(self.config.max_preload_agents)
            .collect();
        
        // Clear previous preloaded agents
        self.preloaded_agents.clear();
        
        // Preload agents
        if self.config.concurrent_preloading && filtered_predictions.len() > 1 {
            // Preload in parallel
            let loader_arc = self.memory_loader.clone();
            let results: Vec<Result<String, String>> = filtered_predictions.par_iter()
                .map(|prediction| {
                    let agent_name = &prediction.agent_name;
                    match &loader_arc {
                        Some(loader) => {
                            if let Ok(mut loader_lock) = loader.lock() {
                                match loader_lock.load_agent_memory(agent_name, context.map(|s| vec![s.to_string()])) {
                                    Ok(_) => Ok(agent_name.clone()),
                                    Err(e) => Err(format!("Failed to preload agent {}: {}", agent_name, e)),
                                }
                            } else {
                                Err(format!("Failed to lock memory loader for agent {}", agent_name))
                            }
                        },
                        None => Err(format!("No memory loader connected for agent {}", agent_name)),
                    }
                })
                .collect();
            
            // Process results
            for result in results {
                match result {
                    Ok(agent_name) => {
                        self.preloaded_agents.insert(agent_name);
                        debug!("Preloaded agent: {}", agent_name);
                    },
                    Err(e) => warn!("{}", e),
                }
            }
        } else {
            // Preload sequentially
            for prediction in &filtered_predictions {
                let agent_name = &prediction.agent_name;
                if let Some(loader) = &self.memory_loader {
                    if let Ok(mut loader_lock) = loader.lock() {
                        match loader_lock.load_agent_memory(agent_name, context.map(|s| vec![s.to_string()])) {
                            Ok(_) => {
                                self.preloaded_agents.insert(agent_name.clone());
                                debug!("Preloaded agent: {}", agent_name);
                            },
                            Err(e) => warn!("Failed to preload agent {}: {}", agent_name, e),
                        }
                    }
                }
            }
        }
        
        // Update statistics
        let elapsed_ms = start_time.elapsed().as_millis() as u64;
        if let Ok(mut stats) = self.stats.write() {
            stats.preloading_count += 1;
            stats.avg_preload_time_ms = ((stats.avg_preload_time_ms * (stats.preloading_count - 1)) + elapsed_ms) / stats.preloading_count;
            stats.last_preload = Some(SystemTime::now());
        }
        
        // Save state
        self.save_state()?;
        
        Ok(filtered_predictions)
    }
    
    // Predict next agents based on history and context
    pub fn predict_next_agents(&self, context: Option<&str>) -> Result<Vec<AgentPrediction>, Box<dyn Error>> {
        let mut predictions = HashMap::new();
        
        // Get list of all agents from various data structures
        let mut all_agents = HashSet::new();
        
        // Add agents from transition matrix
        for (from_agent, transitions) in &self.transition_matrix {
            all_agents.insert(from_agent.clone());
            for (to_agent, _) in transitions {
                all_agents.insert(to_agent.clone());
            }
        }
        
        // Add agents from time patterns
        for agent in self.time_patterns.keys() {
            all_agents.insert(agent.clone());
        }
        
        // Add agents from context affinities
        for affinity in self.context_affinities.values() {
            for agent in affinity.agent_affinities.keys() {
                all_agents.insert(agent.clone());
            }
        }
        
        // Calculate sequential probability (transition matrix)
        if let Some(current_agent) = &self.currently_active_agent {
            if let Some(transitions) = self.transition_matrix.get(current_agent) {
                for (next_agent, probability) in transitions {
                    let factor_value = probability.probability * self.config.sequence_weight;
                    
                    predictions
                        .entry(next_agent.clone())
                        .or_insert_with(|| {
                            HashMap::new()
                        })
                        .insert("sequence".to_string(), factor_value);
                }
            }
        }
        
        // Calculate context-based probability
        if let Some(ctx) = context {
            if let Some(affinity) = self.context_affinities.get(ctx) {
                for (agent, score) in &affinity.agent_affinities {
                    let factor_value = score * self.config.context_weight;
                    
                    predictions
                        .entry(agent.clone())
                        .or_insert_with(|| {
                            HashMap::new()
                        })
                        .insert("context".to_string(), factor_value);
                }
            } else {
                // Try fuzzy matching contexts
                for (context_key, affinity) in &self.context_affinities {
                    // Simple fuzzy matching - check for common words
                    if contexts_are_similar(ctx, context_key) {
                        for (agent, score) in &affinity.agent_affinities {
                            let factor_value = score * self.config.context_weight * 0.8; // Apply discount for fuzzy match
                            
                            let factors = predictions
                                .entry(agent.clone())
                                .or_insert_with(|| {
                                    HashMap::new()
                                });
                            
                            // Use max of existing or new score for fuzzy context
                            let existing = factors.get("context").unwrap_or(&0.0);
                            let new_value = factor_value.max(*existing);
                            factors.insert("context".to_string(), new_value);
                        }
                    }
                }
            }
        }
        
        // Calculate time-based probability
        let now: DateTime<Local> = SystemTime::now().into();
        let current_hour = now.hour() as usize;
        let current_weekday = now.weekday().num_days_from_monday() as usize;
        
        for (agent, pattern) in &self.time_patterns {
            // Combine hourly and daily patterns
            let hourly_prob = pattern.hour_of_day[current_hour];
            let daily_prob = pattern.day_of_week[current_weekday];
            let time_factor = (hourly_prob + daily_prob) / 2.0 * self.config.time_weight;
            
            predictions
                .entry(agent.clone())
                .or_insert_with(|| {
                    HashMap::new()
                })
                .insert("time".to_string(), time_factor);
        }
        
        // Combine all factors and calculate final probability
        let mut final_predictions = Vec::new();
        
        for (agent_name, factors) in &predictions {
            let sequence_factor = *factors.get("sequence").unwrap_or(&0.0);
            let context_factor = *factors.get("context").unwrap_or(&0.0);
            let time_factor = *factors.get("time").unwrap_or(&0.0);
            
            // Calculate combined probability
            let total_probability = sequence_factor + context_factor + time_factor;
            
            // Calculate confidence based on factor diversity
            let factor_count = factors.len() as f64;
            let confidence = factor_count / 3.0; // Maximum 3 factors
            
            final_predictions.push(AgentPrediction {
                agent_name: agent_name.clone(),
                probability: total_probability,
                confidence,
                factors: factors.clone(),
            });
        }
        
        // Sort by probability (descending)
        final_predictions.sort_by(|a, b| b.probability.partial_cmp(&a.probability).unwrap());
        
        Ok(final_predictions)
    }
    
    // Get preloader statistics
    pub fn get_statistics(&self) -> PreloaderStats {
        if let Ok(stats) = self.stats.read() {
            stats.clone()
        } else {
            PreloaderStats::default()
        }
    }
    
    // Format prediction report
    pub fn format_prediction_report(&self, predictions: &[AgentPrediction]) -> String {
        let mut report = String::new();
        report.push_str("# Agent Prediction Report\n\n");
        
        // Current time
        let now: DateTime<Local> = SystemTime::now().into();
        report.push_str(&format!("Generated: {}\n\n", now.format("%Y-%m-%d %H:%M:%S")));
        
        // Current agent
        if let Some(current) = &self.currently_active_agent {
            report.push_str(&format!("Currently active agent: {}\n\n", current));
        }
        
        // Predictions table
        report.push_str("## Predicted Next Agents\n\n");
        report.push_str("| Agent | Probability | Confidence | Factors |\n");
        report.push_str("|-------|-------------|------------|--------|\n");
        
        for prediction in predictions {
            // Format factors
            let factors = prediction.factors.iter()
                .map(|(k, v)| format!("{}={:.2}", k, v))
                .collect::<Vec<String>>()
                .join(", ");
            
            report.push_str(&format!("| {} | {:.1}% | {:.1}% | {} |\n", 
                prediction.agent_name,
                prediction.probability * 100.0,
                prediction.confidence * 100.0,
                factors
            ));
        }
        
        // Statistics
        let stats = self.get_statistics();
        report.push_str("\n## Preloader Statistics\n\n");
        report.push_str(&format!("Total activations: {}\n", stats.total_activations));
        report.push_str(&format!("Successful predictions: {}\n", stats.successful_predictions));
        report.push_str(&format!("Prediction accuracy: {:.1}%\n", stats.prediction_accuracy * 100.0));
        report.push_str(&format!("Average preload time: {}ms\n", stats.avg_preload_time_ms));
        
        report
    }
}

// Helper function to compare contexts for similarity
fn contexts_are_similar(ctx1: &str, ctx2: &str) -> bool {
    // Split contexts into words
    let words1: HashSet<String> = ctx1
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    
    let words2: HashSet<String> = ctx2
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    
    // Check if there are common words
    let intersection = words1.intersection(&words2).count();
    let min_size = words1.len().min(words2.len());
    
    // Consider similar if at least 30% of words match
    if min_size > 0 {
        (intersection as f64 / min_size as f64) >= 0.3
    } else {
        false
    }
}

// Background thread functions

// Main preloading function for thread
fn predict_and_preload(
    config: &MemoryPreloaderConfig,
    history: &Arc<Mutex<VecDeque<ActivationRecord>>>,
    transitions: &Arc<Mutex<HashMap<String, HashMap<String, TransitionProbability>>>>,
    affinities: &Arc<Mutex<HashMap<String, ContextAffinity>>>,
    patterns: &Arc<Mutex<HashMap<String, TimePattern>>>,
    preloaded: &Arc<Mutex<HashSet<String>>>,
    active_agent: &Arc<Mutex<Option<String>>>,
    memory_loader: &Option<Arc<Mutex<MemoryLoader>>>,
    stats: &Arc<RwLock<PreloaderStats>>
) -> Result<(), Box<dyn Error>> {
    // Derive context from recent history
    let context = if let Ok(history_lock) = history.lock() {
        if let Some(last_record) = history_lock.back() {
            last_record.context.clone()
        } else {
            None
        }
    } else {
        None
    };
    
    // Predict next agents
    let predictions = predict_next_agents(
        config,
        active_agent,
        transitions,
        affinities,
        patterns
    )?;
    
    // Filter predictions by threshold
    let filtered_predictions: Vec<(String, f64)> = predictions.into_iter()
        .filter(|(_, prob)| *prob >= config.prediction_threshold)
        .take(config.max_preload_agents)
        .collect();
    
    // Clear previous preloaded agents
    if let Ok(mut preloaded_lock) = preloaded.lock() {
        preloaded_lock.clear();
    }
    
    // Get currently active agent
    let current_agent = if let Ok(active_lock) = active_agent.lock() {
        active_lock.clone()
    } else {
        None
    };
    
    // Preload agents
    if config.concurrent_preloading && filtered_predictions.len() > 1 {
        // Preload in parallel
        let results: Vec<Result<String, String>> = filtered_predictions.par_iter()
            .map(|(agent_name, _)| {
                // Skip if this is the currently active agent
                if let Some(current) = &current_agent {
                    if current == agent_name {
                        return Ok(agent_name.clone());
                    }
                }
                
                // Preload agent memory
                match memory_loader {
                    Some(loader) => {
                        if let Ok(mut loader_lock) = loader.lock() {
                            match loader_lock.load_agent_memory(agent_name, context.as_ref().map(|s| vec![s.clone()])) {
                                Ok(_) => {
                                    if let Ok(mut preloaded_lock) = preloaded.lock() {
                                        preloaded_lock.insert(agent_name.clone());
                                    }
                                    Ok(agent_name.clone())
                                },
                                Err(e) => Err(format!("Failed to preload agent {}: {}", agent_name, e)),
                            }
                        } else {
                            Err(format!("Failed to lock memory loader for agent {}", agent_name))
                        }
                    },
                    None => Err(format!("No memory loader connected for agent {}", agent_name)),
                }
            })
            .collect();
        
        // Process results
        for result in results {
            match result {
                Ok(agent_name) => {
                    debug!("Preloaded agent: {}", agent_name);
                    
                    // Update cache hit counter
                    if let Ok(mut stats_write) = stats.write() {
                        stats_write.cache_hits += 1;
                    }
                },
                Err(e) => warn!("{}", e),
            }
        }
    } else {
        // Preload sequentially
        for (agent_name, _) in &filtered_predictions {
            // Skip if this is the currently active agent
            if let Some(current) = &current_agent {
                if current == agent_name {
                    continue;
                }
            }
            
            // Preload agent memory
            if let Some(loader) = memory_loader {
                if let Ok(mut loader_lock) = loader.lock() {
                    match loader_lock.load_agent_memory(agent_name, context.as_ref().map(|s| vec![s.clone()])) {
                        Ok(_) => {
                            if let Ok(mut preloaded_lock) = preloaded.lock() {
                                preloaded_lock.insert(agent_name.clone());
                            }
                            debug!("Preloaded agent: {}", agent_name);
                            
                            // Update cache hit counter
                            if let Ok(mut stats_write) = stats.write() {
                                stats_write.cache_hits += 1;
                            }
                        },
                        Err(e) => warn!("Failed to preload agent {}: {}", agent_name, e),
                    }
                }
            }
        }
    }
    
    // Update statistics
    if let Ok(mut stats_write) = stats.write() {
        stats_write.preloading_count += 1;
        stats_write.last_preload = Some(SystemTime::now());
    }
    
    Ok(())
}

// Predict next agents for thread
fn predict_next_agents(
    config: &MemoryPreloaderConfig,
    active_agent: &Arc<Mutex<Option<String>>>,
    transitions: &Arc<Mutex<HashMap<String, HashMap<String, TransitionProbability>>>>,
    affinities: &Arc<Mutex<HashMap<String, ContextAffinity>>>,
    patterns: &Arc<Mutex<HashMap<String, TimePattern>>>
) -> Result<Vec<(String, f64)>, Box<dyn Error>> {
    let mut predictions = HashMap::new();
    
    // Get current agent
    let current_agent = if let Ok(active_lock) = active_agent.lock() {
        active_lock.clone()
    } else {
        None
    };
    
    // Calculate sequential probability (transition matrix)
    if let Some(current) = &current_agent {
        if let Ok(transitions_lock) = transitions.lock() {
            if let Some(agent_transitions) = transitions_lock.get(current) {
                for (next_agent, probability) in agent_transitions {
                    let factor_value = probability.probability * config.sequence_weight;
                    *predictions.entry(next_agent.clone()).or_insert(0.0) += factor_value;
                }
            }
        }
    }
    
    // Calculate time-based probability
    let now: DateTime<Local> = SystemTime::now().into();
    let current_hour = now.hour() as usize;
    let current_weekday = now.weekday().num_days_from_monday() as usize;
    
    if let Ok(patterns_lock) = patterns.lock() {
        for (agent, pattern) in &*patterns_lock {
            // Combine hourly and daily patterns
            let hourly_prob = pattern.hour_of_day[current_hour];
            let daily_prob = pattern.day_of_week[current_weekday];
            let time_factor = (hourly_prob + daily_prob) / 2.0 * config.time_weight;
            
            *predictions.entry(agent.clone()).or_insert(0.0) += time_factor;
        }
    }
    
    // Convert to sorted vector
    let mut result: Vec<(String, f64)> = predictions.into_iter().collect();
    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    Ok(result)
}

// Save preloader state for thread
fn save_preloader_state(
    cache_dir: &Path,
    history: &Arc<Mutex<VecDeque<ActivationRecord>>>,
    transitions: &Arc<Mutex<HashMap<String, HashMap<String, TransitionProbability>>>>,
    affinities: &Arc<Mutex<HashMap<String, ContextAffinity>>>,
    patterns: &Arc<Mutex<HashMap<String, TimePattern>>>,
    stats: &Arc<RwLock<PreloaderStats>>
) -> Result<(), Box<dyn Error>> {
    // Save activation history
    if let Ok(history_lock) = history.lock() {
        let history_json = serde_json::to_string(&*history_lock)?;
        let history_file = cache_dir.join("activation_history.json");
        fs::write(&history_file, history_json)?;
    }
    
    // Save transition matrix
    if let Ok(transitions_lock) = transitions.lock() {
        let transitions_json = serde_json::to_string(&*transitions_lock)?;
        let transitions_file = cache_dir.join("transitions.json");
        fs::write(&transitions_file, transitions_json)?;
    }
    
    // Save context affinities
    if let Ok(affinities_lock) = affinities.lock() {
        let affinities_json = serde_json::to_string(&*affinities_lock)?;
        let affinities_file = cache_dir.join("context_affinities.json");
        fs::write(&affinities_file, affinities_json)?;
    }
    
    // Save time patterns
    if let Ok(patterns_lock) = patterns.lock() {
        let patterns_json = serde_json::to_string(&*patterns_lock)?;
        let patterns_file = cache_dir.join("time_patterns.json");
        fs::write(&patterns_file, patterns_json)?;
    }
    
    // Save statistics
    if let Ok(stats_read) = stats.read() {
        let stats_json = serde_json::to_string_pretty(&*stats_read)?;
        let stats_file = cache_dir.join("stats.json");
        fs::write(&stats_file, stats_json)?;
    }
    
    Ok(())
}

// Example usage function
pub fn main() -> Result<(), Box<dyn Error>> {
    // Set up logging
    env_logger::init();
    
    // Create memory preloader with default config
    let config = MemoryPreloaderConfig::default();
    let mut preloader = MemoryPreloader::new(config)?;
    
    // Connect memory loader (in a real application)
    // let memory_loader = Arc::new(Mutex::new(MemoryLoader::new(loader_config)?));
    // preloader.connect_memory_loader(memory_loader);
    
    // Record an activation
    println!("Recording agent activation...");
    preloader.record_activation("Athena", Some("memory optimization"), "user")?;
    
    // Predict next agents
    println!("Predicting next agents...");
    let predictions = preloader.predict_next_agents(Some("memory optimization"))?;
    
    println!("Predicted next agents:");
    for prediction in &predictions {
        println!("  {} - {:.1}% probability, {:.1}% confidence", 
            prediction.agent_name,
            prediction.probability * 100.0,
            prediction.confidence * 100.0
        );
        
        println!("    Factors:");
        for (factor, value) in &prediction.factors {
            println!("      {} = {:.2}", factor, value);
        }
    }
    
    // Generate report
    let report = preloader.format_prediction_report(&predictions);
    println!("\nPrediction Report:\n{}", report);
    
    // Start background preloading
    preloader.start_background_preloading()?;
    
    // In a real application, this would run for a while
    println!("Background preloading started. Press Ctrl+C to stop.");
    thread::sleep(Duration::from_secs(5));
    
    // Stop background preloading
    preloader.stop_background_preloading()?;
    
    Ok(())
}