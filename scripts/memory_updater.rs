use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};
use serde::{Deserialize, Serialize};
use serde_json;
use regex::Regex;
use lazy_static::lazy_static;

// Import from other crate modules
use crate::memory_segment_parser::{MemorySegment, MemorySegmentParser, MemoryTier, MemoryType};
use crate::memory_activation_model::MemoryActivationModel;
use crate::memory_loader::{MemoryLoader, MemoryLoaderConfig};

// Constants for update thresholds
const IMMEDIATE_UPDATE_THRESHOLD: f64 = 0.8;
const SIGNIFICANT_UPDATE_THRESHOLD: f64 = 0.5;
const MINOR_UPDATE_THRESHOLD: f64 = 0.2;

// Update type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdateType {
    Add,
    Modify,
    Remove,
    Consolidate,
}

// Update description structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUpdate {
    pub update_id: String,
    pub update_type: UpdateType,
    pub segment_id: Option<String>,
    pub segment: Option<MemorySegment>,
    pub timestamp: SystemTime,
    pub importance: f64,
    pub source: String,
    pub is_applied: bool,
    pub target_file: Option<PathBuf>,
}

// Update batch for transaction-like operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBatch {
    pub batch_id: String,
    pub updates: Vec<MemoryUpdate>,
    pub timestamp: SystemTime,
    pub is_applied: bool,
    pub target_agent: String,
}

// Update statistics for monitoring
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateStatistics {
    pub total_updates: u64,
    pub successful_updates: u64,
    pub failed_updates: u64,
    pub updates_by_type: HashMap<UpdateType, u64>,
    pub updates_by_agent: HashMap<String, u64>,
    pub updates_by_segment_type: HashMap<MemoryType, u64>,
    pub last_update_duration_ms: u64,
}

// Configuration for memory updater
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUpdaterConfig {
    pub agents_dir: PathBuf,
    pub backup_dir: PathBuf,
    pub auto_consolidation: bool,
    pub max_batch_size: usize,
    pub create_backups: bool,
    pub verbose_logging: bool,
    pub update_cooldown_seconds: u64,
}

impl Default for MemoryUpdaterConfig {
    fn default() -> Self {
        MemoryUpdaterConfig {
            agents_dir: PathBuf::from("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS"),
            backup_dir: PathBuf::from("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CACHE/BACKUPS"),
            auto_consolidation: true,
            max_batch_size: 10,
            create_backups: true,
            verbose_logging: false,
            update_cooldown_seconds: 5,
        }
    }
}

// Main memory updater implementation
pub struct MemoryUpdater {
    config: MemoryUpdaterConfig,
    parser: MemorySegmentParser,
    pending_updates: HashMap<String, Vec<MemoryUpdate>>,
    update_batches: Vec<UpdateBatch>,
    last_update: HashMap<String, SystemTime>, // Agent -> Last update time
    activation_model: Arc<Mutex<MemoryActivationModel>>,
    stats: UpdateStatistics,
}

impl MemoryUpdater {
    // Create a new memory updater
    pub fn new(config: MemoryUpdaterConfig, activation_model: Arc<Mutex<MemoryActivationModel>>) -> Result<Self, Box<dyn Error>> {
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
        
        Ok(MemoryUpdater {
            config,
            parser,
            pending_updates: HashMap::new(),
            update_batches: Vec::new(),
            last_update: HashMap::new(),
            activation_model,
            stats: UpdateStatistics::default(),
        })
    }
    
    // Queue an update for later processing
    pub fn queue_update(&mut self, agent_name: &str, update: MemoryUpdate) -> Result<(), Box<dyn Error>> {
        self.pending_updates
            .entry(agent_name.to_string())
            .or_insert_with(Vec::new)
            .push(update);
        
        // Process updates immediately if they exceed importance threshold
        if let Some(updates) = self.pending_updates.get(agent_name) {
            let has_critical_updates = updates.iter().any(|u| u.importance >= IMMEDIATE_UPDATE_THRESHOLD);
            
            if has_critical_updates || updates.len() >= self.config.max_batch_size {
                self.process_pending_updates(agent_name)?;
            }
        }
        
        Ok(())
    }
    
    // Process pending updates for an agent
    pub fn process_pending_updates(&mut self, agent_name: &str) -> Result<bool, Box<dyn Error>> {
        // Check if updates exist for this agent
        if !self.pending_updates.contains_key(agent_name) || self.pending_updates[agent_name].is_empty() {
            return Ok(false);
        }
        
        // Check cooldown period
        if let Some(last_time) = self.last_update.get(agent_name) {
            if let Ok(duration) = SystemTime::now().duration_since(*last_time) {
                if duration.as_secs() < self.config.update_cooldown_seconds {
                    return Ok(false); // Still in cooldown period
                }
            }
        }
        
        // Create update batch
        let batch_id = format!("batch_{}_{}", agent_name, SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());
        
        let updates = self.pending_updates.remove(agent_name).unwrap_or_default();
        
        if updates.is_empty() {
            return Ok(false);
        }
        
        let batch = UpdateBatch {
            batch_id: batch_id.clone(),
            updates,
            timestamp: SystemTime::now(),
            is_applied: false,
            target_agent: agent_name.to_string(),
        };
        
        // Apply the batch
        let start_time = Instant::now();
        let success = self.apply_update_batch(&batch)?;
        
        // Update batch status
        let mut applied_batch = batch.clone();
        applied_batch.is_applied = success;
        self.update_batches.push(applied_batch);
        
        // Update statistics
        self.stats.total_updates += batch.updates.len() as u64;
        if success {
            self.stats.successful_updates += batch.updates.len() as u64;
            
            // Update type-specific statistics
            for update in &batch.updates {
                self.stats.updates_by_type
                    .entry(update.update_type)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                
                if let Some(segment) = &update.segment {
                    self.stats.updates_by_segment_type
                        .entry(segment.memory_type)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        } else {
            self.stats.failed_updates += batch.updates.len() as u64;
        }
        
        self.stats.updates_by_agent
            .entry(agent_name.to_string())
            .and_modify(|count| *count += batch.updates.len() as u64)
            .or_insert(batch.updates.len() as u64);
        
        self.stats.last_update_duration_ms = start_time.elapsed().as_millis() as u64;
        
        // Update last update time
        self.last_update.insert(agent_name.to_string(), SystemTime::now());
        
        Ok(success)
    }
    
    // Apply a batch of updates atomically
    fn apply_update_batch(&mut self, batch: &UpdateBatch) -> Result<bool, Box<dyn Error>> {
        let agent_name = &batch.target_agent;
        let agent_dir = self.config.agents_dir.join(agent_name);
        
        if !agent_dir.exists() || !agent_dir.is_dir() {
            return Err(format!("Agent directory not found: {}", agent_dir.display()).into());
        }
        
        // Group updates by target file
        let mut updates_by_file: HashMap<PathBuf, Vec<&MemoryUpdate>> = HashMap::new();
        
        for update in &batch.updates {
            if let Some(target_file) = &update.target_file {
                updates_by_file
                    .entry(target_file.clone())
                    .or_insert_with(Vec::new)
                    .push(update);
            } else {
                // Handle updates without specific target file
                match update.update_type {
                    UpdateType::Add => {
                        // For new segments, determine best target file
                        if let Some(segment) = &update.segment {
                            let target_file = self.determine_target_file(agent_name, segment)?;
                            updates_by_file
                                .entry(target_file)
                                .or_insert_with(Vec::new)
                                .push(update);
                        }
                    },
                    UpdateType::Consolidate => {
                        // Consolidation is handled separately
                    },
                    _ => {
                        // For modify/remove, segment_id should include source file
                        if let Some(segment_id) = &update.segment_id {
                            if let Some(segment) = &update.segment {
                                let source_file = agent_dir.join(&segment.source_file);
                                if source_file.exists() {
                                    updates_by_file
                                        .entry(source_file)
                                        .or_insert_with(Vec::new)
                                        .push(update);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Create backups before making changes
        if self.config.create_backups {
            for file_path in updates_by_file.keys() {
                if file_path.exists() {
                    let backup_path = self.create_backup(file_path)?;
                    if self.config.verbose_logging {
                        println!("Created backup: {}", backup_path.display());
                    }
                }
            }
        }
        
        // Apply updates to each file
        let mut all_successful = true;
        
        for (file_path, updates) in updates_by_file {
            if let Err(e) = self.apply_updates_to_file(&file_path, updates) {
                eprintln!("Error updating file {}: {}", file_path.display(), e);
                all_successful = false;
                
                // Restore backups on failure
                if self.config.create_backups {
                    self.restore_backups(agent_name)?;
                }
                
                break;
            }
        }
        
        // Handle consolidation separately if needed
        if all_successful && self.config.auto_consolidation {
            let consolidation_updates: Vec<&MemoryUpdate> = batch.updates.iter()
                .filter(|u| u.update_type == UpdateType::Consolidate)
                .collect();
            
            if !consolidation_updates.is_empty() {
                if let Err(e) = self.consolidate_memory(agent_name, &consolidation_updates) {
                    eprintln!("Error during memory consolidation: {}", e);
                    all_successful = false;
                    
                    // Restore backups on failure
                    if self.config.create_backups {
                        self.restore_backups(agent_name)?;
                    }
                }
            }
        }
        
        Ok(all_successful)
    }
    
    // Apply multiple updates to a single file
    fn apply_updates_to_file(&mut self, file_path: &Path, updates: Vec<&MemoryUpdate>) -> Result<(), Box<dyn Error>> {
        // Read the current file content
        let mut content = if file_path.exists() {
            fs::read_to_string(file_path)?
        } else {
            String::new()
        };
        
        // Parse existing segments
        let existing_segments = if !content.is_empty() {
            self.parser.parse_file(file_path)?
        } else {
            Vec::new()
        };
        
        // Create a map of segment IDs to their content and positions
        let mut segment_map: HashMap<String, (String, usize, usize)> = HashMap::new();
        for segment in &existing_segments {
            segment_map.insert(
                segment.id.clone(),
                (segment.content.clone(), segment.line_start, segment.line_end)
            );
        }
        
        // Process updates in order
        let mut modified = false;
        
        for update in updates {
            match update.update_type {
                UpdateType::Add => {
                    if let Some(segment) = &update.segment {
                        // Create segment content with markers
                        let segment_text = self.format_segment_with_markers(segment);
                        
                        // Append to file
                        if !content.is_empty() && !content.ends_with('\n') {
                            content.push('\n');
                        }
                        
                        content.push_str(&segment_text);
                        modified = true;
                    }
                },
                UpdateType::Modify => {
                    if let Some(segment_id) = &update.segment_id {
                        if let Some(segment) = &update.segment {
                            if let Some((old_content, start, end)) = segment_map.get(segment_id) {
                                // Calculate line numbers
                                let lines: Vec<&str> = content.lines().collect();
                                
                                // Extract the old segment with markers
                                let old_segment_text = if *start > 0 && *end <= lines.len() {
                                    lines[*start - 1..*end].join("\n")
                                } else {
                                    // Fall back to regex-based extraction if line numbers are incorrect
                                    self.extract_segment_by_id(&content, segment_id)?
                                };
                                
                                // Create new segment content with markers
                                let new_segment_text = self.format_segment_with_markers(segment);
                                
                                // Replace the old segment with the new one
                                content = content.replace(&old_segment_text, &new_segment_text);
                                modified = true;
                            }
                        }
                    }
                },
                UpdateType::Remove => {
                    if let Some(segment_id) = &update.segment_id {
                        if let Some((_, start, end)) = segment_map.get(segment_id) {
                            // Calculate line numbers
                            let lines: Vec<&str> = content.lines().collect();
                            
                            // Extract the segment with markers
                            let segment_text = if *start > 0 && *end <= lines.len() {
                                lines[*start - 1..*end].join("\n")
                            } else {
                                // Fall back to regex-based extraction if line numbers are incorrect
                                self.extract_segment_by_id(&content, segment_id)?
                            };
                            
                            // Remove the segment
                            content = content.replace(&segment_text, "");
                            
                            // Clean up any double newlines
                            content = content.replace("\n\n\n", "\n\n");
                            modified = true;
                        }
                    }
                },
                _ => {} // Consolidation handled separately
            }
        }
        
        // Write the updated content back to the file if modified
        if modified {
            fs::write(file_path, &content)?;
        }
        
        Ok(())
    }
    
    // Extract a segment by ID using regex
    fn extract_segment_by_id(&self, content: &str, segment_id: &str) -> Result<String, Box<dyn Error>> {
        lazy_static! {
            static ref SEGMENT_PATTERN: Regex = Regex::new(
                r"(?s)<!--\s*MEMORY_SEGMENT\s*.*?ID=([a-zA-Z0-9_]+).*?-->.*?<!--\s*END_MEMORY_SEGMENT\s*-->"
            ).expect("Invalid regex pattern");
        }
        
        for cap in SEGMENT_PATTERN.captures_iter(content) {
            if let Some(id_match) = cap.get(1) {
                if id_match.as_str() == segment_id {
                    return Ok(cap.get(0).unwrap().as_str().to_string());
                }
            }
        }
        
        Err(format!("Segment with ID {} not found", segment_id).into())
    }
    
    // Format a segment with appropriate markers
    fn format_segment_with_markers(&self, segment: &MemorySegment) -> String {
        let tier_str = match segment.tier {
            MemoryTier::Tier1 => "1",
            MemoryTier::Tier2 => "2",
            MemoryTier::Tier3 => "3",
        };
        
        let type_str = match segment.memory_type {
            MemoryType::Identity => "IDENTITY",
            MemoryType::Semantic => "SEMANTIC",
            MemoryType::Episodic => "EPISODIC",
            MemoryType::Procedural => "PROCEDURAL",
            MemoryType::Relational => "RELATIONAL",
        };
        
        let mut marker = format!(
            "<!-- MEMORY_SEGMENT\nTYPE={}\nTIER={}\nDOMAIN={}\nID={}\n",
            type_str,
            tier_str,
            segment.domains.join(","),
            segment.id
        );
        
        // Add metadata
        for (key, value) in &segment.metadata {
            marker.push_str(&format!("{}={}\n", key, value));
        }
        
        // Close opening marker and add content
        marker.push_str("-->\n");
        marker.push_str(&segment.content);
        marker.push_str("\n<!-- END_MEMORY_SEGMENT -->\n");
        
        marker
    }
    
    // Determine the best target file for a new segment
    fn determine_target_file(&self, agent_name: &str, segment: &MemorySegment) -> Result<PathBuf, Box<dyn Error>> {
        let agent_dir = self.config.agents_dir.join(agent_name);
        
        // Choose file based on segment type
        match segment.memory_type {
            MemoryType::Identity => {
                // Identity segments go to README.md or MEMORY.md
                let readme = agent_dir.join("README.md");
                if readme.exists() {
                    return Ok(readme);
                }
            },
            MemoryType::Episodic => {
                // Episodic memory typically goes to ContinuousLearning.md
                let learning = agent_dir.join("ContinuousLearning.md");
                if learning.exists() {
                    return Ok(learning);
                }
            },
            _ => {}
        }
        
        // Default to MEMORY.md
        let memory = agent_dir.join("MEMORY.md");
        if memory.exists() {
            return Ok(memory);
        }
        
        // If MEMORY.md doesn't exist, create it
        return Ok(memory);
    }
    
    // Create a backup of a file
    fn create_backup(&self, file_path: &Path) -> Result<PathBuf, Box<dyn Error>> {
        if !file_path.exists() {
            return Err(format!("File does not exist: {}", file_path.display()).into());
        }
        
        let file_name = file_path.file_name()
            .ok_or("Invalid file path")?
            .to_string_lossy()
            .to_string();
        
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        
        let backup_name = format!("{}_{}.bak", file_name, timestamp);
        let backup_path = self.config.backup_dir.join(backup_name);
        
        fs::copy(file_path, &backup_path)?;
        
        Ok(backup_path)
    }
    
    // Restore backups for an agent
    fn restore_backups(&self, agent_name: &str) -> Result<(), Box<dyn Error>> {
        let agent_dir = self.config.agents_dir.join(agent_name);
        
        if !self.config.create_backups {
            return Ok(());
        }
        
        // Find the latest backups for each file in the agent directory
        let mut latest_backups: HashMap<String, (u64, PathBuf)> = HashMap::new();
        
        for entry in fs::read_dir(&self.config.backup_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if !path.is_file() || path.extension().map_or(false, |ext| ext != "bak") {
                continue;
            }
            
            let file_name = path.file_stem()
                .ok_or("Invalid backup file")?
                .to_string_lossy();
            
            // Parse the timestamp from the backup name
            let parts: Vec<&str> = file_name.split('_').collect();
            if parts.len() < 2 {
                continue;
            }
            
            let base_name = parts[0..parts.len()-1].join("_");
            if let Ok(timestamp) = parts.last().unwrap().parse::<u64>() {
                if let Some((existing_ts, _)) = latest_backups.get(&base_name) {
                    if timestamp > *existing_ts {
                        latest_backups.insert(base_name, (timestamp, path.clone()));
                    }
                } else {
                    latest_backups.insert(base_name, (timestamp, path.clone()));
                }
            }
        }
        
        // Restore each file from its latest backup
        for (file_name, (_, backup_path)) in latest_backups {
            let original_path = agent_dir.join(&file_name);
            fs::copy(backup_path, original_path)?;
        }
        
        Ok(())
    }
    
    // Consolidate memory segments for an agent
    fn consolidate_memory(&self, agent_name: &str, updates: &[&MemoryUpdate]) -> Result<(), Box<dyn Error>> {
        let agent_dir = self.config.agents_dir.join(agent_name);
        
        // Collect all relevant memory files
        let mut memory_files = Vec::new();
        
        let memory_md = agent_dir.join("MEMORY.md");
        if memory_md.exists() {
            memory_files.push(memory_md);
        }
        
        let learning_md = agent_dir.join("ContinuousLearning.md");
        if learning_md.exists() {
            memory_files.push(learning_md);
        }
        
        // Read all segments
        let mut all_segments = Vec::new();
        for file_path in &memory_files {
            let file_segments = self.parser.parse_file(file_path)?;
            all_segments.extend(file_segments);
        }
        
        // Group segments by type
        let mut segments_by_type: HashMap<MemoryType, Vec<MemorySegment>> = HashMap::new();
        for segment in all_segments {
            segments_by_type
                .entry(segment.memory_type)
                .or_insert_with(Vec::new)
                .push(segment);
        }
        
        // Create organized content for each memory type
        let mut identity_content = String::new();
        let mut semantic_content = String::new();
        let mut episodic_content = String::new();
        let mut procedural_content = String::new();
        let mut relational_content = String::new();
        
        // Process each type
        if let Some(segments) = segments_by_type.get(&MemoryType::Identity) {
            identity_content.push_str("# Core Identity\n\n");
            for segment in segments {
                identity_content.push_str(&self.format_segment_with_markers(segment));
                identity_content.push('\n');
            }
        }
        
        if let Some(segments) = segments_by_type.get(&MemoryType::Semantic) {
            semantic_content.push_str("# Semantic Knowledge\n\n");
            for segment in segments {
                semantic_content.push_str(&self.format_segment_with_markers(segment));
                semantic_content.push('\n');
            }
        }
        
        if let Some(segments) = segments_by_type.get(&MemoryType::Episodic) {
            episodic_content.push_str("# Experience Records\n\n");
            // Sort episodic memory by timestamp if available
            let mut sorted_segments = segments.clone();
            sorted_segments.sort_by(|a, b| {
                if let (Some(a_time), Some(b_time)) = (a.metadata.get("timestamp"), b.metadata.get("timestamp")) {
                    b_time.cmp(a_time) // Reverse order (newest first)
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            
            for segment in sorted_segments {
                episodic_content.push_str(&self.format_segment_with_markers(segment));
                episodic_content.push('\n');
            }
        }
        
        if let Some(segments) = segments_by_type.get(&MemoryType::Procedural) {
            procedural_content.push_str("# Procedural Knowledge\n\n");
            for segment in segments {
                procedural_content.push_str(&self.format_segment_with_markers(segment));
                procedural_content.push('\n');
            }
        }
        
        if let Some(segments) = segments_by_type.get(&MemoryType::Relational) {
            relational_content.push_str("# Relational Knowledge\n\n");
            for segment in segments {
                relational_content.push_str(&self.format_segment_with_markers(segment));
                relational_content.push('\n');
            }
        }
        
        // Write to consolidated files
        if !memory_md.exists() {
            // Create MEMORY.md if it doesn't exist
            let memory_content = format!(
                "# {}'s Memory\n\n{}\n{}\n{}\n",
                agent_name,
                identity_content,
                semantic_content,
                relational_content
            );
            fs::write(&memory_md, memory_content)?;
        } else {
            // Update existing MEMORY.md
            let mut memory_content = fs::read_to_string(&memory_md)?;
            
            // Keep any non-segmented content at the top
            let segmented_start = memory_content.find("<!-- MEMORY_SEGMENT");
            let header_content = if let Some(pos) = segmented_start {
                memory_content[..pos].to_string()
            } else {
                memory_content.clone()
            };
            
            // Create new content
            let new_content = format!(
                "{}\n{}\n{}\n{}\n",
                header_content.trim(),
                identity_content,
                semantic_content,
                relational_content
            );
            
            fs::write(&memory_md, new_content)?;
        }
        
        // Update ContinuousLearning.md if it exists
        if learning_md.exists() {
            let learning_content = format!(
                "# {}'s Learning Records\n\n{}\n{}\n",
                agent_name,
                episodic_content,
                procedural_content
            );
            fs::write(&learning_md, learning_content)?;
        } else if !episodic_content.is_empty() || !procedural_content.is_empty() {
            // Create it if it doesn't exist but we have content for it
            let learning_content = format!(
                "# {}'s Learning Records\n\n{}\n{}\n",
                agent_name,
                episodic_content,
                procedural_content
            );
            fs::write(&learning_md, learning_content)?;
        }
        
        Ok(())
    }
    
    // Create a new memory update
    pub fn create_update(&self, update_type: UpdateType, segment: Option<MemorySegment>, importance: f64, source: &str, target_file: Option<PathBuf>) -> MemoryUpdate {
        let update_id = format!("update_{}", SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());
        
        MemoryUpdate {
            update_id,
            update_type,
            segment_id: segment.as_ref().map(|s| s.id.clone()),
            segment,
            timestamp: SystemTime::now(),
            importance,
            source: source.to_string(),
            is_applied: false,
            target_file,
        }
    }
    
    // Get update statistics
    pub fn get_statistics(&self) -> &UpdateStatistics {
        &self.stats
    }
    
    // Reset statistics
    pub fn reset_statistics(&mut self) {
        self.stats = UpdateStatistics::default();
    }
}

// Example usage function
pub fn main() -> Result<(), Box<dyn Error>> {
    // Create an activation model
    let activation_model = Arc::new(Mutex::new(MemoryActivationModel::new()));
    
    // Create updater with default config
    let config = MemoryUpdaterConfig::default();
    let mut updater = MemoryUpdater::new(config, activation_model)?;
    
    // Example: Add a new memory segment
    let agent_name = "Athena";
    
    let new_segment = MemorySegment {
        id: format!("test_segment_{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs()),
        memory_type: MemoryType::Episodic,
        tier: MemoryTier::Tier2,
        domains: vec!["test".to_string(), "example".to_string()],
        content: "## Test Memory Segment\n\nThis is a test segment created by the memory updater.".to_string(),
        last_accessed: SystemTime::now(),
        access_count: 0,
        source_file: "ContinuousLearning.md".to_string(),
        line_start: 0,
        line_end: 0,
        metadata: [("created_by".to_string(), "updater_test".to_string())].iter().cloned().collect(),
    };
    
    // Create an update
    let update = updater.create_update(
        UpdateType::Add,
        Some(new_segment),
        0.7, // High importance but not immediate
        "test",
        None // Let the system determine the target file
    );
    
    // Queue the update
    updater.queue_update(agent_name, update)?;
    
    // Process pending updates
    let result = updater.process_pending_updates(agent_name)?;
    
    println!("Update applied: {}", result);
    println!("Updates processed: {}", updater.get_statistics().total_updates);
    
    Ok(())
}