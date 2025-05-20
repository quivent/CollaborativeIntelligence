use std::path::{Path, PathBuf};
use std::fs;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use serde::{Serialize, Deserialize};

use super::memory_segment_parser::{MemorySegment, MemoryType, MemoryTier};
use super::memory_activation_model::MemoryActivationModel;

// Types of memory updates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdateType {
    Create,     // Create a new segment
    Append,     // Append to an existing segment
    Replace,    // Replace an existing segment
    Delete,     // Delete a segment
    Modify,     // Modify part of a segment
}

// Memory update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUpdate {
    pub update_type: UpdateType,
    pub segment: Option<MemorySegment>,
    pub target_segment_id: Option<String>,
    pub importance: f64,
    pub source: String,
    pub metadata: HashMap<String, String>,
    pub timestamp: u64,
}

// Batch of memory updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBatch {
    pub agent_name: String,
    pub updates: Vec<MemoryUpdate>,
    pub batch_id: String,
    pub timestamp: u64,
}

// Memory updater configuration
#[derive(Debug, Clone)]
pub struct MemoryUpdaterConfig {
    pub base_directory: PathBuf,
    pub agents_directory: PathBuf,
    pub batch_size: usize,
    pub auto_process: bool,
    pub auto_process_interval: Duration,
    pub importance_threshold: f64,
}

impl Default for MemoryUpdaterConfig {
    fn default() -> Self {
        Self {
            base_directory: PathBuf::from("."),
            agents_directory: PathBuf::from("AGENTS"),
            batch_size: 10,
            auto_process: true,
            auto_process_interval: Duration::from_secs(300), // 5 minutes
            importance_threshold: 0.5,
        }
    }
}

// Memory updater statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStatistics {
    pub total_updates: u64,
    pub creates: u64,
    pub appends: u64,
    pub replaces: u64,
    pub deletes: u64,
    pub modifies: u64,
    pub batches_processed: u64,
    pub last_update_time: u64,
}

impl Default for UpdateStatistics {
    fn default() -> Self {
        Self {
            total_updates: 0,
            creates: 0,
            appends: 0,
            replaces: 0,
            deletes: 0,
            modifies: 0,
            batches_processed: 0,
            last_update_time: 0,
        }
    }
}

// Memory updater
pub struct MemoryUpdater {
    config: MemoryUpdaterConfig,
    activation_model: Arc<MemoryActivationModel>,
    pending_updates: Arc<Mutex<HashMap<String, VecDeque<MemoryUpdate>>>>,
    statistics: Arc<Mutex<UpdateStatistics>>,
    last_auto_process: Arc<Mutex<u64>>,
}

impl MemoryUpdater {
    // Create a new updater
    pub fn new(
        config: MemoryUpdaterConfig,
        activation_model: Arc<MemoryActivationModel>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let updater = Self {
            config,
            activation_model,
            pending_updates: Arc::new(Mutex::new(HashMap::new())),
            statistics: Arc::new(Mutex::new(UpdateStatistics::default())),
            last_auto_process: Arc::new(Mutex::new(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            )),
        };
        
        // Start auto-processing if enabled
        if updater.config.auto_process {
            // In a real implementation, this would spawn a thread
            // For the sake of simplicity, we'll leave this as a comment
            /*
            let updater_clone = updater.clone();
            std::thread::spawn(move || {
                updater_clone.auto_process_loop();
            });
            */
        }
        
        Ok(updater)
    }
    
    // Create a memory update
    pub fn create_update(
        &self,
        update_type: UpdateType,
        segment: Option<MemorySegment>,
        importance: f64,
        source: &str,
        target_segment_id: Option<&str>,
    ) -> MemoryUpdate {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        MemoryUpdate {
            update_type,
            segment,
            target_segment_id: target_segment_id.map(|s| s.to_string()),
            importance,
            source: source.to_string(),
            metadata: HashMap::new(),
            timestamp,
        }
    }
    
    // Queue an update for processing
    pub fn queue_update(
        &self,
        agent_name: &str,
        update: MemoryUpdate,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Get pending updates
        let mut pending = self.pending_updates.lock().unwrap();
        
        // Get or create agent's update queue
        let agent_updates = pending
            .entry(agent_name.to_string())
            .or_insert_with(VecDeque::new);
        
        // Add update to queue
        agent_updates.push_back(update);
        
        // Check if we should auto-process
        if self.config.auto_process {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            let mut last_auto_process = self.last_auto_process.lock().unwrap();
            
            if now.saturating_sub(*last_auto_process) >= self.config.auto_process_interval.as_secs() {
                // Drop locks before processing
                drop(pending);
                drop(last_auto_process);
                
                // Process updates
                self.process_pending_updates(agent_name)?;
                
                // Update last auto-process time
                *self.last_auto_process.lock().unwrap() = now;
            }
        }
        
        Ok(())
    }
    
    // Process pending updates for an agent
    pub fn process_pending_updates(
        &self,
        agent_name: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // Get pending updates
        let mut pending = self.pending_updates.lock().unwrap();
        
        // Check if agent has any updates
        if let Some(agent_updates) = pending.get_mut(agent_name) {
            if agent_updates.is_empty() {
                return Ok(false);
            }
            
            // Determine batch size
            let batch_size = std::cmp::min(self.config.batch_size, agent_updates.len());
            
            // Take updates for this batch
            let mut batch_updates = Vec::with_capacity(batch_size);
            for _ in 0..batch_size {
                if let Some(update) = agent_updates.pop_front() {
                    batch_updates.push(update);
                }
            }
            
            // Create batch
            let batch_id = format!("{}_batch_{}", agent_name, SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs());
            
            let batch = UpdateBatch {
                agent_name: agent_name.to_string(),
                updates: batch_updates,
                batch_id,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            };
            
            // Release lock before processing
            drop(pending);
            
            // Process batch
            self.process_batch(&batch)?;
            
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    // Process a batch of updates
    fn process_batch(&self, batch: &UpdateBatch) -> Result<(), Box<dyn std::error::Error>> {
        // Group updates by file
        let mut file_updates: HashMap<String, Vec<&MemoryUpdate>> = HashMap::new();
        
        // First pass: group updates by file
        for update in &batch.updates {
            // Determine target file based on update type and segment
            let target_file = match update.update_type {
                UpdateType::Create => {
                    if let Some(segment) = &update.segment {
                        if segment.source_file.is_empty() {
                            // Determine appropriate file based on segment type
                            match segment.memory_type {
                                MemoryType::Learning => format!("AGENTS/{}/ContinuousLearning.md", batch.agent_name),
                                MemoryType::Core => format!("AGENTS/{}/MEMORY.md", batch.agent_name),
                                _ => {
                                    // For other types, create a new file in Sessions
                                    format!("AGENTS/{}/Sessions/{}.md", 
                                        batch.agent_name,
                                        segment.id.replace(':', "_").replace(' ', "_")
                                    )
                                }
                            }
                        } else {
                            segment.source_file.clone()
                        }
                    } else {
                        // Cannot create without a segment
                        continue;
                    }
                },
                _ => {
                    // For other update types, need a target segment
                    if let Some(segment) = &update.segment {
                        segment.source_file.clone()
                    } else if let Some(target_id) = &update.target_segment_id {
                        // Would need to look up the segment file from target ID
                        // For simplicity, we'll skip this case
                        continue;
                    } else {
                        // Cannot update without a target
                        continue;
                    }
                }
            };
            
            // Add to file updates
            file_updates.entry(target_file)
                .or_insert_with(Vec::new)
                .push(update);
        }
        
        // Second pass: process updates for each file
        for (file_path, updates) in file_updates {
            if updates.is_empty() {
                continue;
            }
            
            // Process updates for this file
            self.process_file_updates(&file_path, &updates, &batch.agent_name)?;
        }
        
        // Update statistics
        let mut stats = self.statistics.lock().unwrap();
        stats.batches_processed += 1;
        stats.last_update_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        for update in &batch.updates {
            stats.total_updates += 1;
            
            match update.update_type {
                UpdateType::Create => stats.creates += 1,
                UpdateType::Append => stats.appends += 1,
                UpdateType::Replace => stats.replaces += 1,
                UpdateType::Delete => stats.deletes += 1,
                UpdateType::Modify => stats.modifies += 1,
            }
        }
        
        Ok(())
    }
    
    // Process updates for a specific file
    fn process_file_updates(
        &self,
        file_path: &str,
        updates: &[&MemoryUpdate],
        agent_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let full_path = self.config.base_directory.join(file_path);
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Read existing content or create empty string
        let mut content = if full_path.exists() {
            fs::read_to_string(&full_path)?
        } else {
            String::new()
        };
        
        // Process updates
        let mut content_modified = false;
        
        for update in updates {
            match update.update_type {
                UpdateType::Create => {
                    if let Some(segment) = &update.segment {
                        // For create, append to the end of the file or create new file
                        if content.is_empty() {
                            content = segment.content.clone();
                        } else {
                            content.push_str("\n\n");
                            content.push_str(&segment.content);
                        }
                        content_modified = true;
                        
                        // Register with activation model
                        self.activation_model.register_access(agent_name, segment, Some(update.importance));
                    }
                },
                UpdateType::Append => {
                    if let Some(segment) = &update.segment {
                        // For append, find target segment and append to it
                        // This would be more complex in a real implementation
                        // For simplicity, we'll just append to the end
                        content.push_str("\n\n");
                        content.push_str(&segment.content);
                        content_modified = true;
                        
                        // Register with activation model
                        self.activation_model.register_access(agent_name, segment, Some(update.importance));
                    }
                },
                UpdateType::Replace => {
                    if let Some(segment) = &update.segment {
                        // For replace, this would find and replace a specific segment
                        // In a real implementation, this would be more sophisticated
                        // For simplicity, we'll just replace the whole file
                        content = segment.content.clone();
                        content_modified = true;
                        
                        // Register with activation model
                        self.activation_model.register_access(agent_name, segment, Some(update.importance));
                    }
                },
                // Other update types would be implemented similarly
                _ => {}
            }
        }
        
        // Write modified content back to file
        if content_modified {
            fs::write(&full_path, content)?;
        }
        
        Ok(())
    }
    
    // Auto-process loop (would run in a separate thread)
    fn auto_process_loop(&self) {
        loop {
            // Sleep for the auto-process interval
            std::thread::sleep(self.config.auto_process_interval);
            
            // Get agent names with pending updates
            let agent_names: Vec<String> = {
                let pending = self.pending_updates.lock().unwrap();
                pending.keys().cloned().collect()
            };
            
            // Process updates for each agent
            for agent_name in agent_names {
                if let Err(e) = self.process_pending_updates(&agent_name) {
                    eprintln!("Error processing updates for {}: {}", agent_name, e);
                }
            }
        }
    }
    
    // Get statistics
    pub fn get_statistics(&self) -> UpdateStatistics {
        self.statistics.lock().unwrap().clone()
    }
    
    // Reset statistics
    pub fn reset_statistics(&self) {
        let mut stats = self.statistics.lock().unwrap();
        *stats = UpdateStatistics::default();
    }
    
    // Get pending update count for an agent
    pub fn get_pending_update_count(&self, agent_name: &str) -> usize {
        let pending = self.pending_updates.lock().unwrap();
        
        if let Some(agent_updates) = pending.get(agent_name) {
            agent_updates.len()
        } else {
            0
        }
    }
    
    // Get total pending update count
    pub fn get_total_pending_update_count(&self) -> usize {
        let pending = self.pending_updates.lock().unwrap();
        
        pending.values().map(|queue| queue.len()).sum()
    }
    
    // Clear pending updates for an agent
    pub fn clear_pending_updates(&self, agent_name: &str) {
        let mut pending = self.pending_updates.lock().unwrap();
        
        if let Some(agent_updates) = pending.get_mut(agent_name) {
            agent_updates.clear();
        }
    }
    
    // Update configuration
    pub fn update_config(&mut self, config: MemoryUpdaterConfig) {
        self.config = config;
    }
}