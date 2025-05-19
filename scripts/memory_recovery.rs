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

// Import from other crate modules
use crate::memory_segment_parser::{MemorySegment, MemorySegmentParser, MemoryTier, MemoryType};
use crate::memory_loader::MemoryLoader;
use crate::memory_updater::{MemoryUpdater, MemoryUpdate, UpdateType};
use crate::memory_checksum_cache::ChecksumCache;

// Memory recovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecoveryConfig {
    pub agents_dir: PathBuf,
    pub backup_dir: PathBuf,
    pub auto_backup_interval_hours: u64,
    pub max_backups_per_agent: usize,
    pub auto_recovery: bool,
    pub recovery_threshold: f64,         // 0.0-1.0: how damaged before auto-recovery
    pub integrity_check_interval_hours: u64,
    pub enable_background_checks: bool,
    pub enable_segment_repair: bool,
    pub enable_quarantine: bool,
    pub repair_templates_dir: PathBuf,
    pub verbose_logging: bool,
}

impl Default for MemoryRecoveryConfig {
    fn default() -> Self {
        MemoryRecoveryConfig {
            agents_dir: PathBuf::from("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS"),
            backup_dir: PathBuf::from("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CACHE/BACKUPS"),
            auto_backup_interval_hours: 24,  // Daily backups
            max_backups_per_agent: 10,      // Keep 10 backups per agent
            auto_recovery: true,
            recovery_threshold: 0.1,        // 10% damaged segments trigger auto-recovery
            integrity_check_interval_hours: 12, // Twice-daily integrity checks
            enable_background_checks: true,
            enable_segment_repair: true,
            enable_quarantine: true,
            repair_templates_dir: PathBuf::from("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CACHE/REPAIR_TEMPLATES"),
            verbose_logging: false,
        }
    }
}

// Memory integrity status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrityStatus {
    Good,              // No issues detected
    Minor,             // Minor issues (recoverable)
    Major,             // Major issues (potentially recoverable)
    Critical,          // Critical issues (may require manual intervention)
    Corrupted,         // File corrupted
}

// Memory segment integrity issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentIssue {
    pub segment_id: String,
    pub issue_type: String,
    pub description: String,
    pub severity: IntegrityStatus,
    pub detected_at: SystemTime,
    pub resolved_at: Option<SystemTime>,
    pub repair_attempts: u32,
    pub is_quarantined: bool,
}

// File backup information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub original_path: PathBuf,
    pub backup_path: PathBuf,
    pub created_at: SystemTime,
    pub size_bytes: u64,
    pub hash: String,
    pub integrity_status: IntegrityStatus,
    pub metadata: HashMap<String, String>,
}

// Memory file integrity check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityCheckResult {
    pub file_path: PathBuf,
    pub agent_name: String,
    pub check_time: SystemTime,
    pub status: IntegrityStatus,
    pub issues: Vec<SegmentIssue>,
    pub segments_checked: usize,
    pub segments_with_issues: usize,
    pub error_percentage: f64,
    pub is_recovered: bool,
    pub recovery_source: Option<String>,
    pub recovery_time: Option<SystemTime>,
}

// Recovery system statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecoveryStats {
    pub total_checks: u64,
    pub files_checked: u64,
    pub issues_detected: u64,
    pub auto_recoveries: u64,
    pub manual_recoveries: u64,
    pub segments_repaired: u64,
    pub segments_quarantined: u64,
    pub backups_created: u64,
    pub last_check: Option<SystemTime>,
    pub agent_check_results: HashMap<String, IntegrityCheckResult>,
    pub check_duration_ms: u64,
}

// Memory recovery system implementation
pub struct MemoryRecovery {
    config: MemoryRecoveryConfig,
    parser: MemorySegmentParser,
    integrity_results: HashMap<PathBuf, IntegrityCheckResult>,
    backups: HashMap<String, Vec<BackupInfo>>, // agent_name -> backups
    quarantined_segments: HashMap<String, Vec<MemorySegment>>, // agent_name -> segments
    repair_templates: HashMap<String, String>, // issue_type -> template
    stats: Arc<RwLock<RecoveryStats>>,
    memory_loader: Option<Arc<Mutex<MemoryLoader>>>,
    memory_updater: Option<Arc<Mutex<MemoryUpdater>>>,
    checksum_cache: Option<Arc<Mutex<ChecksumCache>>>,
    check_thread: Option<thread::JoinHandle<()>>,
    running: Arc<RwLock<bool>>,
}

impl MemoryRecovery {
    // Create a new memory recovery system
    pub fn new(config: MemoryRecoveryConfig) -> Result<Self, Box<dyn Error>> {
        // Create backup and quarantine directories
        fs::create_dir_all(&config.backup_dir)?;
        fs::create_dir_all(&config.repair_templates_dir)?;
        
        // Initialize parser
        let mut parser = MemorySegmentParser::new();
        
        // Register common domains
        parser.register_domain("identity", vec!["identity".to_string(), "purpose".to_string(), "core".to_string()]);
        parser.register_domain("learning", vec!["learn".to_string(), "knowledge".to_string(), "education".to_string()]);
        parser.register_domain("system", vec!["system".to_string(), "framework".to_string(), "architecture".to_string()]);
        parser.register_domain("memory", vec!["memory".to_string(), "recall".to_string(), "remember".to_string()]);
        parser.register_domain("technical", vec!["code".to_string(), "implementation".to_string(), "software".to_string()]);
        
        // Load repair templates
        let repair_templates = load_repair_templates(&config.repair_templates_dir)?;
        
        let recovery = MemoryRecovery {
            config,
            parser,
            integrity_results: HashMap::new(),
            backups: HashMap::new(),
            quarantined_segments: HashMap::new(),
            repair_templates,
            stats: Arc::new(RwLock::new(RecoveryStats::default())),
            memory_loader: None,
            memory_updater: None,
            checksum_cache: None,
            check_thread: None,
            running: Arc::new(RwLock::new(false)),
        };
        
        // Load existing backup info
        recovery.load_backup_info()?;
        
        // Load existing integrity results
        recovery.load_integrity_results()?;
        
        Ok(recovery)
    }
    
    // Connect memory components
    pub fn connect_memory_components(
        &mut self,
        loader: Option<Arc<Mutex<MemoryLoader>>>,
        updater: Option<Arc<Mutex<MemoryUpdater>>>,
        checksum_cache: Option<Arc<Mutex<ChecksumCache>>>
    ) {
        self.memory_loader = loader;
        self.memory_updater = updater;
        self.checksum_cache = checksum_cache;
    }
    
    // Start background integrity checks
    pub fn start_background_checks(&mut self) -> Result<(), Box<dyn Error>> {
        if self.config.enable_background_checks {
            // Signal that background checks are running
            *self.running.write().unwrap() = true;
            
            // Clone references needed for the thread
            let running = self.running.clone();
            let config = self.config.clone();
            let stats = self.stats.clone();
            let memory_loader = self.memory_loader.clone();
            let memory_updater = self.memory_updater.clone();
            let checksum_cache = self.checksum_cache.clone();
            
            // Spawn the background thread
            let handle = thread::spawn(move || {
                info!("Background memory integrity checker started");
                
                // Create thread-local parser
                let mut parser = MemorySegmentParser::new();
                parser.register_domain("identity", vec!["identity".to_string(), "purpose".to_string(), "core".to_string()]);
                parser.register_domain("learning", vec!["learn".to_string(), "knowledge".to_string(), "education".to_string()]);
                parser.register_domain("system", vec!["system".to_string(), "framework".to_string(), "architecture".to_string()]);
                parser.register_domain("memory", vec!["memory".to_string(), "recall".to_string(), "remember".to_string()]);
                parser.register_domain("technical", vec!["code".to_string(), "implementation".to_string(), "software".to_string()]);
                
                // Load repair templates
                let repair_templates = match load_repair_templates(&config.repair_templates_dir) {
                    Ok(templates) => templates,
                    Err(e) => {
                        error!("Failed to load repair templates: {}", e);
                        HashMap::new()
                    }
                };
                
                // Integrity results storage
                let mut integrity_results = HashMap::new();
                
                // Storage for backup info
                let mut backups = HashMap::new();
                
                // Load existing backup info
                match load_backup_info_from_dir(&config.backup_dir) {
                    Ok(loaded_backups) => {
                        backups = loaded_backups;
                    },
                    Err(e) => {
                        error!("Failed to load backup info: {}", e);
                    }
                }
                
                // Initialize last check times
                let mut last_integrity_check = SystemTime::now()
                    .checked_sub(Duration::from_secs(3600 * config.integrity_check_interval_hours))
                    .unwrap_or_else(|| SystemTime::now());
                
                let mut last_backup = SystemTime::now()
                    .checked_sub(Duration::from_secs(3600 * config.auto_backup_interval_hours))
                    .unwrap_or_else(|| SystemTime::now());
                
                while *running.read().unwrap() {
                    let now = SystemTime::now();
                    
                    // Check if it's time for integrity check
                    if let Ok(duration) = now.duration_since(last_integrity_check) {
                        if duration.as_secs() >= 3600 * config.integrity_check_interval_hours {
                            info!("Running scheduled integrity check");
                            
                            // Get list of agents
                            if let Ok(entries) = fs::read_dir(&config.agents_dir) {
                                let agents: Vec<_> = entries
                                    .filter_map(|e| e.ok())
                                    .filter(|e| e.path().is_dir())
                                    .collect();
                                
                                for entry in agents {
                                    let agent_name = entry.file_name().to_string_lossy().to_string();
                                    let agent_dir = entry.path();
                                    
                                    // Check integrity for this agent
                                    match check_agent_integrity(
                                        &agent_name,
                                        &agent_dir,
                                        &mut parser, 
                                        &config,
                                        &repair_templates,
                                        &memory_updater,
                                        &stats
                                    ) {
                                        Ok(result) => {
                                            // Store result
                                            for (file_path, check_result) in result {
                                                integrity_results.insert(file_path, check_result.clone());
                                                
                                                // Trigger auto-recovery if needed
                                                if config.auto_recovery && 
                                                   check_result.error_percentage >= config.recovery_threshold &&
                                                   check_result.status != IntegrityStatus::Good {
                                                    
                                                    info!("Auto-recovery triggered for agent {} (error rate: {:.1}%)", 
                                                        agent_name, check_result.error_percentage * 100.0);
                                                    
                                                    // Perform recovery
                                                    if let Err(e) = recover_agent_memory(
                                                        &agent_name,
                                                        &check_result.file_path,
                                                        &backups,
                                                        &config,
                                                        &mut parser,
                                                        &memory_updater,
                                                        &stats
                                                    ) {
                                                        error!("Auto-recovery failed for agent {}: {}", agent_name, e);
                                                    }
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            error!("Integrity check failed for agent {}: {}", agent_name, e);
                                        }
                                    }
                                }
                            }
                            
                            // Save integrity results
                            match save_integrity_results(&config.backup_dir, &integrity_results) {
                                Ok(_) => {
                                    if config.verbose_logging {
                                        debug!("Saved integrity results");
                                    }
                                },
                                Err(e) => {
                                    error!("Failed to save integrity results: {}", e);
                                }
                            }
                            
                            // Update last integrity check time
                            last_integrity_check = now;
                        }
                    }
                    
                    // Check if it's time for backup
                    if let Ok(duration) = now.duration_since(last_backup) {
                        if duration.as_secs() >= 3600 * config.auto_backup_interval_hours {
                            info!("Creating scheduled memory backups");
                            
                            // Get list of agents
                            if let Ok(entries) = fs::read_dir(&config.agents_dir) {
                                let agents: Vec<_> = entries
                                    .filter_map(|e| e.ok())
                                    .filter(|e| e.path().is_dir())
                                    .collect();
                                
                                for entry in agents {
                                    let agent_name = entry.file_name().to_string_lossy().to_string();
                                    let agent_dir = entry.path();
                                    
                                    // Create backup for this agent
                                    match backup_agent_memory(&agent_name, &agent_dir, &config, &mut backups, &stats) {
                                        Ok(_) => {
                                            if config.verbose_logging {
                                                debug!("Created backup for agent {}", agent_name);
                                            }
                                        },
                                        Err(e) => {
                                            error!("Failed to create backup for agent {}: {}", agent_name, e);
                                        }
                                    }
                                }
                            }
                            
                            // Save backup info
                            match save_backup_info(&config.backup_dir, &backups) {
                                Ok(_) => {
                                    if config.verbose_logging {
                                        debug!("Saved backup info");
                                    }
                                },
                                Err(e) => {
                                    error!("Failed to save backup info: {}", e);
                                }
                            }
                            
                            // Update last backup time
                            last_backup = now;
                        }
                    }
                    
                    // Sleep for a while
                    thread::sleep(Duration::from_secs(60));
                }
                
                info!("Background memory integrity checker stopped");
            });
            
            self.check_thread = Some(handle);
        }
        
        Ok(())
    }
    
    // Stop background integrity checks
    pub fn stop_background_checks(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(_) = self.check_thread {
            // Signal the thread to stop
            *self.running.write().unwrap() = false;
            
            // Wait for the thread to finish
            if let Some(handle) = self.check_thread.take() {
                handle.join().map_err(|_| "Failed to join integrity check thread")?;
            }
            
            info!("Background memory integrity checker stopped");
        }
        
        Ok(())
    }
    
    // Load backup information
    fn load_backup_info(&self) -> Result<(), Box<dyn Error>> {
        // Backup info file
        let backups_file = self.config.backup_dir.join("backups.json");
        
        if backups_file.exists() {
            let content = fs::read_to_string(&backups_file)?;
            let backups: HashMap<String, Vec<BackupInfo>> = serde_json::from_str(&content)?;
            
            // Store backup info
            for (agent, backup_list) in backups {
                self.backups.insert(agent, backup_list);
            }
        }
        
        Ok(())
    }
    
    // Load integrity results
    fn load_integrity_results(&self) -> Result<(), Box<dyn Error>> {
        // Integrity results file
        let results_file = self.config.backup_dir.join("integrity_results.json");
        
        if results_file.exists() {
            let content = fs::read_to_string(&results_file)?;
            let results: HashMap<String, IntegrityCheckResult> = serde_json::from_str(&content)?;
            
            // Store integrity results
            for (path_str, result) in results {
                let path = PathBuf::from(path_str);
                self.integrity_results.insert(path, result);
            }
        }
        
        Ok(())
    }
    
    // Save backup information
    fn save_backup_info(&self) -> Result<(), Box<dyn Error>> {
        // Create a serializable structure
        let backups_json = serde_json::to_string_pretty(&self.backups)?;
        
        // Save to file
        let backups_file = self.config.backup_dir.join("backups.json");
        fs::write(&backups_file, backups_json)?;
        
        Ok(())
    }
    
    // Save integrity results
    fn save_integrity_results(&self) -> Result<(), Box<dyn Error>> {
        // Create a serializable structure
        let mut results_map = HashMap::new();
        
        for (path, result) in &self.integrity_results {
            results_map.insert(path.to_string_lossy().to_string(), result.clone());
        }
        
        let results_json = serde_json::to_string_pretty(&results_map)?;
        
        // Save to file
        let results_file = self.config.backup_dir.join("integrity_results.json");
        fs::write(&results_file, results_json)?;
        
        Ok(())
    }
    
    // Create a backup of agent memory
    pub fn backup_agent_memory(&mut self, agent_name: &str) -> Result<Vec<BackupInfo>, Box<dyn Error>> {
        let agent_dir = self.config.agents_dir.join(agent_name);
        
        if !agent_dir.exists() || !agent_dir.is_dir() {
            return Err(format!("Agent directory not found: {}", agent_dir.display()).into());
        }
        
        let mut backups = Vec::new();
        
        // Get agent memory files
        let memory_files = get_agent_memory_files(&agent_dir)?;
        
        // Create backup dir for this agent
        let agent_backup_dir = self.config.backup_dir.join(agent_name);
        fs::create_dir_all(&agent_backup_dir)?;
        
        // Generate timestamp for backup files
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Backup each memory file
        for file_path in memory_files {
            if !file_path.exists() {
                continue;
            }
            
            let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
            let backup_name = format!("{}_{}.bak", file_name, timestamp);
            let backup_path = agent_backup_dir.join(backup_name);
            
            // Copy file to backup
            fs::copy(&file_path, &backup_path)?;
            
            // Get file size
            let metadata = fs::metadata(&backup_path)?;
            let size_bytes = metadata.len();
            
            // Calculate file hash
            let content = fs::read_to_string(&file_path)?;
            let hash = format!("{:x}", md5::compute(&content));
            
            // Create backup info
            let backup_info = BackupInfo {
                original_path: file_path.clone(),
                backup_path: backup_path.clone(),
                created_at: SystemTime::now(),
                size_bytes,
                hash,
                integrity_status: IntegrityStatus::Good, // Assume backup is good
                metadata: HashMap::new(),
            };
            
            backups.push(backup_info);
        }
        
        // Update backup list for this agent
        if !self.backups.contains_key(agent_name) {
            self.backups.insert(agent_name.to_string(), Vec::new());
        }
        
        let agent_backups = self.backups.get_mut(agent_name).unwrap();
        
        // Add new backups
        for backup in &backups {
            agent_backups.push(backup.clone());
        }
        
        // Limit number of backups
        if agent_backups.len() > self.config.max_backups_per_agent {
            // Sort by creation time (oldest first)
            agent_backups.sort_by(|a, b| a.created_at.cmp(&b.created_at));
            
            // Remove excess backups
            while agent_backups.len() > self.config.max_backups_per_agent {
                if let Some(old_backup) = agent_backups.first() {
                    // Try to remove the backup file
                    if old_backup.backup_path.exists() {
                        if let Err(e) = fs::remove_file(&old_backup.backup_path) {
                            warn!("Failed to remove old backup file {}: {}", old_backup.backup_path.display(), e);
                        }
                    }
                }
                
                agent_backups.remove(0);
            }
        }
        
        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.backups_created += backups.len() as u64;
        }
        
        // Save backup info
        self.save_backup_info()?;
        
        Ok(backups)
    }
    
    // Check integrity of agent memory
    pub fn check_agent_integrity(&mut self, agent_name: &str) -> Result<HashMap<PathBuf, IntegrityCheckResult>, Box<dyn Error>> {
        let start_time = Instant::now();
        let agent_dir = self.config.agents_dir.join(agent_name);
        
        if !agent_dir.exists() || !agent_dir.is_dir() {
            return Err(format!("Agent directory not found: {}", agent_dir.display()).into());
        }
        
        let mut results = HashMap::new();
        
        // Get agent memory files
        let memory_files = get_agent_memory_files(&agent_dir)?;
        
        // Check each memory file
        for file_path in memory_files {
            if !file_path.exists() {
                continue;
            }
            
            // Check file integrity
            match self.check_file_integrity(&file_path, agent_name) {
                Ok(result) => {
                    // Store result
                    results.insert(file_path.clone(), result.clone());
                    
                    // Update global integrity results
                    self.integrity_results.insert(file_path.clone(), result.clone());
                    
                    // Trigger auto-recovery if needed
                    if self.config.auto_recovery && 
                       result.error_percentage >= self.config.recovery_threshold &&
                       result.status != IntegrityStatus::Good {
                        
                        info!("Auto-recovery triggered for file {} (error rate: {:.1}%)", 
                            file_path.display(), result.error_percentage * 100.0);
                        
                        // Perform recovery
                        if let Err(e) = self.recover_file_from_backup(&file_path, agent_name) {
                            error!("Auto-recovery failed for file {}: {}", file_path.display(), e);
                        }
                    }
                },
                Err(e) => {
                    error!("Failed to check integrity of file {}: {}", file_path.display(), e);
                }
            }
        }
        
        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.total_checks += 1;
            stats.files_checked += results.len() as u64;
            stats.last_check = Some(SystemTime::now());
            stats.check_duration_ms = start_time.elapsed().as_millis() as u64;
            
            // Update agent check results
            for (_, result) in &results {
                stats.agent_check_results.insert(agent_name.to_string(), result.clone());
                stats.issues_detected += result.segments_with_issues as u64;
            }
        }
        
        // Save integrity results
        self.save_integrity_results()?;
        
        Ok(results)
    }
    
    // Check integrity of a memory file
    fn check_file_integrity(&self, file_path: &Path, agent_name: &str) -> Result<IntegrityCheckResult, Box<dyn Error>> {
        // Try to read file content
        let content = fs::read_to_string(file_path)?;
        
        // Parse segments
        let segments = match self.parser.parse_file(file_path) {
            Ok(segs) => segs,
            Err(e) => {
                // File is corrupted if we can't parse it
                return Ok(IntegrityCheckResult {
                    file_path: file_path.to_path_buf(),
                    agent_name: agent_name.to_string(),
                    check_time: SystemTime::now(),
                    status: IntegrityStatus::Corrupted,
                    issues: vec![
                        SegmentIssue {
                            segment_id: "file".to_string(),
                            issue_type: "parse_error".to_string(),
                            description: format!("File parsing failed: {}", e),
                            severity: IntegrityStatus::Critical,
                            detected_at: SystemTime::now(),
                            resolved_at: None,
                            repair_attempts: 0,
                            is_quarantined: false,
                        }
                    ],
                    segments_checked: 0,
                    segments_with_issues: 1,
                    error_percentage: 1.0,
                    is_recovered: false,
                    recovery_source: None,
                    recovery_time: None,
                });
            }
        };
        
        // Check each segment for issues
        let mut issues = Vec::new();
        
        for segment in &segments {
            // Perform integrity checks on segment
            let segment_issues = self.check_segment_integrity(segment);
            issues.extend(segment_issues);
        }
        
        // Calculate error percentage
        let segments_with_issues = issues.len();
        let error_percentage = if segments.len() > 0 {
            segments_with_issues as f64 / segments.len() as f64
        } else {
            0.0
        };
        
        // Determine overall status
        let status = if segments_with_issues == 0 {
            IntegrityStatus::Good
        } else if error_percentage < 0.05 {
            IntegrityStatus::Minor
        } else if error_percentage < 0.2 {
            IntegrityStatus::Major
        } else {
            IntegrityStatus::Critical
        };
        
        // Create result
        let result = IntegrityCheckResult {
            file_path: file_path.to_path_buf(),
            agent_name: agent_name.to_string(),
            check_time: SystemTime::now(),
            status,
            issues,
            segments_checked: segments.len(),
            segments_with_issues,
            error_percentage,
            is_recovered: false,
            recovery_source: None,
            recovery_time: None,
        };
        
        Ok(result)
    }
    
    // Check integrity of a memory segment
    fn check_segment_integrity(&self, segment: &MemorySegment) -> Vec<SegmentIssue> {
        let mut issues = Vec::new();
        
        // Check for empty content
        if segment.content.trim().is_empty() {
            issues.push(SegmentIssue {
                segment_id: segment.id.clone(),
                issue_type: "empty_content".to_string(),
                description: "Segment has empty content".to_string(),
                severity: IntegrityStatus::Minor,
                detected_at: SystemTime::now(),
                resolved_at: None,
                repair_attempts: 0,
                is_quarantined: false,
            });
        }
        
        // Check for malformed markdown (simple checks)
        if segment.content.contains("# ") && !segment.content.contains("\n# ") && !segment.content.starts_with("# ") {
            issues.push(SegmentIssue {
                segment_id: segment.id.clone(),
                issue_type: "malformed_markdown".to_string(),
                description: "Segment contains malformed markdown headers".to_string(),
                severity: IntegrityStatus::Minor,
                detected_at: SystemTime::now(),
                resolved_at: None,
                repair_attempts: 0,
                is_quarantined: false,
            });
        }
        
        // Check for unterminated code blocks
        let code_block_starts = segment.content.matches("```").count();
        if code_block_starts % 2 != 0 {
            issues.push(SegmentIssue {
                segment_id: segment.id.clone(),
                issue_type: "unterminated_code_block".to_string(),
                description: "Segment contains unterminated code blocks".to_string(),
                severity: IntegrityStatus::Minor,
                detected_at: SystemTime::now(),
                resolved_at: None,
                repair_attempts: 0,
                is_quarantined: false,
            });
        }
        
        // Check for invalid tier levels
        match segment.tier {
            MemoryTier::Tier1 | MemoryTier::Tier2 | MemoryTier::Tier3 => {}, // Valid tiers
            _ => {
                issues.push(SegmentIssue {
                    segment_id: segment.id.clone(),
                    issue_type: "invalid_tier".to_string(),
                    description: format!("Segment has invalid tier: {:?}", segment.tier),
                    severity: IntegrityStatus::Major,
                    detected_at: SystemTime::now(),
                    resolved_at: None,
                    repair_attempts: 0,
                    is_quarantined: false,
                });
            }
        }
        
        // Check for invalid memory types
        match segment.memory_type {
            MemoryType::Identity | MemoryType::Semantic | MemoryType::Episodic |
            MemoryType::Procedural | MemoryType::Relational => {}, // Valid types
            _ => {
                issues.push(SegmentIssue {
                    segment_id: segment.id.clone(),
                    issue_type: "invalid_memory_type".to_string(),
                    description: format!("Segment has invalid memory type: {:?}", segment.memory_type),
                    severity: IntegrityStatus::Major,
                    detected_at: SystemTime::now(),
                    resolved_at: None,
                    repair_attempts: 0,
                    is_quarantined: false,
                });
            }
        }
        
        // Check for unbalanced parentheses, brackets, etc.
        let parentheses_balance = segment.content.chars().filter(|&c| c == '(').count() - 
                                 segment.content.chars().filter(|&c| c == ')').count();
        let brackets_balance = segment.content.chars().filter(|&c| c == '[').count() - 
                              segment.content.chars().filter(|&c| c == ']').count();
        let braces_balance = segment.content.chars().filter(|&c| c == '{').count() - 
                            segment.content.chars().filter(|&c| c == '}').count();
        
        if parentheses_balance != 0 || brackets_balance != 0 || braces_balance != 0 {
            issues.push(SegmentIssue {
                segment_id: segment.id.clone(),
                issue_type: "unbalanced_delimiters".to_string(),
                description: "Segment contains unbalanced parentheses, brackets, or braces".to_string(),
                severity: IntegrityStatus::Minor,
                detected_at: SystemTime::now(),
                resolved_at: None,
                repair_attempts: 0,
                is_quarantined: false,
            });
        }
        
        // Check for missing metadata
        if segment.metadata.is_empty() {
            issues.push(SegmentIssue {
                segment_id: segment.id.clone(),
                issue_type: "missing_metadata".to_string(),
                description: "Segment has no metadata".to_string(),
                severity: IntegrityStatus::Minor,
                detected_at: SystemTime::now(),
                resolved_at: None,
                repair_attempts: 0,
                is_quarantined: false,
            });
        }
        
        issues
    }
    
    // Recover a file from backup
    pub fn recover_file_from_backup(&mut self, file_path: &Path, agent_name: &str) -> Result<(), Box<dyn Error>> {
        // Get backups for this agent
        let agent_backups = match self.backups.get(agent_name) {
            Some(backups) => backups,
            None => return Err(format!("No backups found for agent: {}", agent_name).into()),
        };
        
        // Find backups for this file
        let file_backups: Vec<&BackupInfo> = agent_backups.iter()
            .filter(|b| b.original_path == file_path)
            .collect();
        
        if file_backups.is_empty() {
            return Err(format!("No backups found for file: {}", file_path.display()).into());
        }
        
        // Sort backups by creation time (newest first)
        let mut sorted_backups = file_backups.clone();
        sorted_backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        // Use the newest backup with good integrity
        let good_backup = sorted_backups.iter()
            .find(|b| matches!(b.integrity_status, IntegrityStatus::Good));
        
        let backup_to_use = good_backup.unwrap_or(&sorted_backups[0]);
        
        // Create a timestamped backup of the current file before replacing
        let current_backup_dir = self.config.backup_dir.join(format!("recovery_backups/{}", agent_name));
        fs::create_dir_all(&current_backup_dir)?;
        
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
        let current_backup_path = current_backup_dir.join(format!("{}_{}_pre_recovery.bak", file_name, timestamp));
        
        // Backup current file if it exists
        if file_path.exists() {
            fs::copy(file_path, &current_backup_path)?;
        }
        
        // Restore from backup
        fs::copy(&backup_to_use.backup_path, file_path)?;
        
        info!("Recovered file {} from backup {}", file_path.display(), backup_to_use.backup_path.display());
        
        // Update integrity result
        if let Some(result) = self.integrity_results.get_mut(file_path) {
            result.is_recovered = true;
            result.recovery_source = Some(backup_to_use.backup_path.to_string_lossy().to_string());
            result.recovery_time = Some(SystemTime::now());
        }
        
        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.manual_recoveries += 1;
        }
        
        // Save integrity results
        self.save_integrity_results()?;
        
        Ok(())
    }
    
    // Repair a segment
    pub fn repair_segment(&mut self, file_path: &Path, segment_id: &str) -> Result<bool, Box<dyn Error>> {
        // Check if segment repair is enabled
        if !self.config.enable_segment_repair {
            return Err("Segment repair is disabled".into());
        }
        
        // Get integrity result for this file
        let result = match self.integrity_results.get(file_path) {
            Some(r) => r,
            None => return Err(format!("No integrity check result for file: {}", file_path.display()).into()),
        };
        
        // Find the issue for this segment
        let issue = result.issues.iter()
            .find(|i| i.segment_id == segment_id);
        
        let issue = match issue {
            Some(i) => i,
            None => return Err(format!("No issues found for segment: {}", segment_id).into()),
        };
        
        // Get repair template for this issue type
        let template = match self.repair_templates.get(&issue.issue_type) {
            Some(t) => t,
            None => return Err(format!("No repair template for issue type: {}", issue.issue_type).into()),
        };
        
        // Read file content
        let content = fs::read_to_string(file_path)?;
        
        // Parse segments
        let segments = self.parser.parse_file(file_path)?;
        
        // Find the segment to repair
        let segment = segments.iter()
            .find(|s| s.id == segment_id);
        
        let segment = match segment {
            Some(s) => s,
            None => return Err(format!("Segment not found: {}", segment_id).into()),
        };
        
        // Apply repair template
        let repaired_content = match issue.issue_type.as_str() {
            "empty_content" => {
                // Replace empty content with template
                template.replace("{{SEGMENT_ID}}", &segment.id)
                       .replace("{{MEMORY_TYPE}}", &format!("{:?}", segment.memory_type))
                       .replace("{{TIER}}", &format!("{:?}", segment.tier))
            },
            "malformed_markdown" => {
                // Fix markdown headers
                let mut fixed = segment.content.clone();
                // Ensure headers start at beginning of line or after newline
                let header_regex = regex::Regex::new(r"([^\n])# ").unwrap();
                fixed = header_regex.replace_all(&fixed, "$1\n# ").to_string();
                fixed
            },
            "unterminated_code_block" => {
                // Add missing code block terminators
                let mut fixed = segment.content.clone();
                let code_block_starts = fixed.matches("```").count();
                if code_block_starts % 2 != 0 {
                    fixed.push_str("\n```");
                }
                fixed
            },
            "unbalanced_delimiters" => {
                // Simple balancing of delimiters
                let mut fixed = segment.content.clone();
                let parentheses_balance = fixed.chars().filter(|&c| c == '(').count() - 
                                        fixed.chars().filter(|&c| c == ')').count();
                let brackets_balance = fixed.chars().filter(|&c| c == '[').count() - 
                                      fixed.chars().filter(|&c| c == ']').count();
                let braces_balance = fixed.chars().filter(|&c| c == '{').count() - 
                                    fixed.chars().filter(|&c| c == '}').count();
                
                // Add missing closing delimiters
                for _ in 0..parentheses_balance {
                    fixed.push(')');
                }
                for _ in 0..brackets_balance {
                    fixed.push(']');
                }
                for _ in 0..braces_balance {
                    fixed.push('}');
                }
                
                // Add missing opening delimiters (prepend)
                let mut prefix = String::new();
                for _ in 0..-parentheses_balance {
                    prefix.push('(');
                }
                for _ in 0..-brackets_balance {
                    prefix.push('[');
                }
                for _ in 0..-braces_balance {
                    prefix.push('{');
                }
                
                if !prefix.is_empty() {
                    prefix.push_str(&fixed);
                    fixed = prefix;
                }
                
                fixed
            },
            _ => {
                // For other issues, use the template
                template.replace("{{SEGMENT_ID}}", &segment.id)
                       .replace("{{MEMORY_TYPE}}", &format!("{:?}", segment.memory_type))
                       .replace("{{TIER}}", &format!("{:?}", segment.tier))
            }
        };
        
        // Create update for repaired segment
        if let Some(updater) = &self.memory_updater {
            if let Ok(mut updater_lock) = updater.lock() {
                // Create a modified segment
                let mut repaired_segment = segment.clone();
                repaired_segment.content = repaired_content;
                
                // Create update
                let update = updater_lock.create_update(
                    UpdateType::Modify,
                    Some(repaired_segment),
                    0.8, // High importance
                    "memory_recovery",
                    Some(file_path.to_path_buf())
                );
                
                // Queue the update
                updater_lock.queue_update(&result.agent_name, update)?;
                
                // Process the update
                updater_lock.process_pending_updates(&result.agent_name)?;
                
                // Update statistics
                if let Ok(mut stats) = self.stats.write() {
                    stats.segments_repaired += 1;
                }
                
                info!("Repaired segment {} in file {}", segment_id, file_path.display());
                
                Ok(true)
            } else {
                Err("Failed to lock memory updater".into())
            }
        } else {
            Err("No memory updater connected".into())
        }
    }
    
    // Quarantine a segment
    pub fn quarantine_segment(&mut self, file_path: &Path, segment_id: &str) -> Result<(), Box<dyn Error>> {
        // Check if quarantine is enabled
        if !self.config.enable_quarantine {
            return Err("Quarantine is disabled".into());
        }
        
        // Get integrity result for this file
        let result = match self.integrity_results.get(file_path) {
            Some(r) => r,
            None => return Err(format!("No integrity check result for file: {}", file_path.display()).into()),
        };
        
        // Parse segments
        let segments = self.parser.parse_file(file_path)?;
        
        // Find the segment to quarantine
        let segment = segments.iter()
            .find(|s| s.id == segment_id);
        
        let segment = match segment {
            Some(s) => s.clone(),
            None => return Err(format!("Segment not found: {}", segment_id).into()),
        };
        
        // Add to quarantined segments
        if !self.quarantined_segments.contains_key(&result.agent_name) {
            self.quarantined_segments.insert(result.agent_name.clone(), Vec::new());
        }
        
        let agent_quarantine = self.quarantined_segments.get_mut(&result.agent_name).unwrap();
        agent_quarantine.push(segment.clone());
        
        // Remove segment from file
        if let Some(updater) = &self.memory_updater {
            if let Ok(mut updater_lock) = updater.lock() {
                // Create update to remove segment
                let update = updater_lock.create_update(
                    UpdateType::Remove,
                    Some(segment),
                    0.9, // Very high importance
                    "memory_recovery_quarantine",
                    Some(file_path.to_path_buf())
                );
                
                // Queue the update
                updater_lock.queue_update(&result.agent_name, update)?;
                
                // Process the update
                updater_lock.process_pending_updates(&result.agent_name)?;
                
                // Update statistics
                if let Ok(mut stats) = self.stats.write() {
                    stats.segments_quarantined += 1;
                }
                
                info!("Quarantined segment {} from file {}", segment_id, file_path.display());
                
                Ok(())
            } else {
                Err("Failed to lock memory updater".into())
            }
        } else {
            Err("No memory updater connected".into())
        }
    }
    
    // Restore a quarantined segment
    pub fn restore_quarantined_segment(&mut self, agent_name: &str, segment_id: &str) -> Result<(), Box<dyn Error>> {
        // Get quarantined segments for this agent
        let agent_quarantine = match self.quarantined_segments.get_mut(agent_name) {
            Some(q) => q,
            None => return Err(format!("No quarantined segments for agent: {}", agent_name).into()),
        };
        
        // Find the segment to restore
        let segment_index = agent_quarantine.iter()
            .position(|s| s.id == segment_id);
        
        let segment_index = match segment_index {
            Some(i) => i,
            None => return Err(format!("Quarantined segment not found: {}", segment_id).into()),
        };
        
        // Remove from quarantine
        let segment = agent_quarantine.remove(segment_index);
        
        // Get target file
        let file_path = self.config.agents_dir.join(agent_name).join(&segment.source_file);
        
        // Add segment back to file
        if let Some(updater) = &self.memory_updater {
            if let Ok(mut updater_lock) = updater.lock() {
                // Create update to add segment
                let update = updater_lock.create_update(
                    UpdateType::Add,
                    Some(segment),
                    0.7, // High importance
                    "memory_recovery_restore",
                    Some(file_path)
                );
                
                // Queue the update
                updater_lock.queue_update(agent_name, update)?;
                
                // Process the update
                updater_lock.process_pending_updates(agent_name)?;
                
                info!("Restored quarantined segment {} for agent {}", segment_id, agent_name);
                
                Ok(())
            } else {
                Err("Failed to lock memory updater".into())
            }
        } else {
            Err("No memory updater connected".into())
        }
    }
    
    // Get recovery statistics
    pub fn get_statistics(&self) -> RecoveryStats {
        if let Ok(stats) = self.stats.read() {
            stats.clone()
        } else {
            RecoveryStats::default()
        }
    }
    
    // Format integrity report
    pub fn format_integrity_report(&self, agent_name: Option<&str>) -> String {
        let mut report = String::new();
        report.push_str("# Memory Integrity Report\n\n");
        
        // Get current time
        let now: DateTime<Local> = SystemTime::now().into();
        report.push_str(&format!("Generated: {}\n\n", now.format("%Y-%m-%d %H:%M:%S")));
        
        // Get statistics
        let stats = self.get_statistics();
        
        // Filter by agent if specified
        let results: Vec<(&PathBuf, &IntegrityCheckResult)> = if let Some(agent) = agent_name {
            self.integrity_results.iter()
                .filter(|(_, r)| r.agent_name == agent)
                .collect()
        } else {
            self.integrity_results.iter().collect()
        };
        
        // Overall statistics
        report.push_str("## Overall Statistics\n\n");
        report.push_str(&format!("Total checks: {}\n", stats.total_checks));
        report.push_str(&format!("Files checked: {}\n", stats.files_checked));
        report.push_str(&format!("Issues detected: {}\n", stats.issues_detected));
        report.push_str(&format!("Auto recoveries: {}\n", stats.auto_recoveries));
        report.push_str(&format!("Manual recoveries: {}\n", stats.manual_recoveries));
        report.push_str(&format!("Segments repaired: {}\n", stats.segments_repaired));
        report.push_str(&format!("Segments quarantined: {}\n", stats.segments_quarantined));
        
        if let Some(last_check) = stats.last_check {
            let dt: DateTime<Local> = last_check.into();
            report.push_str(&format!("Last check: {}\n", dt.format("%Y-%m-%d %H:%M:%S")));
        }
        
        report.push_str(&format!("Check duration: {}ms\n\n", stats.check_duration_ms));
        
        // File integrity results
        report.push_str("## File Integrity Results\n\n");
        report.push_str("| File | Agent | Status | Issues | Error % | Recovered |\n");
        report.push_str("|------|-------|--------|--------|---------|----------|\n");
        
        for (path, result) in results {
            let filename = path.file_name().unwrap_or_default().to_string_lossy();
            let status = format!("{:?}", result.status);
            let issues = result.segments_with_issues;
            let error_pct = format!("{:.1}%", result.error_percentage * 100.0);
            let recovered = if result.is_recovered { "Yes" } else { "No" };
            
            report.push_str(&format!("| {} | {} | {} | {} | {} | {} |\n",
                             filename, result.agent_name, status, issues, error_pct, recovered));
        }
        
        // Detailed issues
        report.push_str("\n## Detailed Issues\n\n");
        
        for (path, result) in results {
            if result.issues.is_empty() {
                continue;
            }
            
            let filename = path.file_name().unwrap_or_default().to_string_lossy();
            report.push_str(&format!("### {}\n\n", filename));
            
            report.push_str("| Segment ID | Issue Type | Description | Severity | Quarantined |\n");
            report.push_str("|------------|------------|-------------|----------|-------------|\n");
            
            for issue in &result.issues {
                let segment_id = &issue.segment_id;
                let issue_type = &issue.issue_type;
                let description = &issue.description;
                let severity = format!("{:?}", issue.severity);
                let quarantined = if issue.is_quarantined { "Yes" } else { "No" };
                
                report.push_str(&format!("| {} | {} | {} | {} | {} |\n",
                               segment_id, issue_type, description, severity, quarantined));
            }
            
            report.push_str("\n");
        }
        
        report
    }
}

// Helper function to get agent memory files
fn get_agent_memory_files(agent_dir: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut memory_files = Vec::new();
    
    // Check for MEMORY.md
    let memory_md = agent_dir.join("MEMORY.md");
    if memory_md.exists() {
        memory_files.push(memory_md);
    }
    
    // Check for ContinuousLearning.md
    let learning_md = agent_dir.join("ContinuousLearning.md");
    if learning_md.exists() {
        memory_files.push(learning_md);
    }
    
    // Check for README.md
    let readme_md = agent_dir.join("README.md");
    if readme_md.exists() {
        memory_files.push(readme_md);
    }
    
    // Check for other markdown files
    for entry in fs::read_dir(agent_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            // Skip already added files
            if !memory_files.contains(&path) {
                memory_files.push(path);
            }
        }
    }
    
    Ok(memory_files)
}

// Helper function to load repair templates
fn load_repair_templates(templates_dir: &Path) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut templates = HashMap::new();
    
    // Create templates directory if it doesn't exist
    if !templates_dir.exists() {
        fs::create_dir_all(templates_dir)?;
        
        // Create default templates
        let empty_content_template = "# {{SEGMENT_ID}}\n\nThis content was automatically repaired.\n\nPlease add appropriate content for this {{MEMORY_TYPE}} segment.";
        fs::write(templates_dir.join("empty_content.template"), empty_content_template)?;
        
        let malformed_markdown_template = "# {{SEGMENT_ID}}\n\n## Subsection\n\nThis content was automatically repaired to fix markdown formatting.";
        fs::write(templates_dir.join("malformed_markdown.template"), malformed_markdown_template)?;
        
        let generic_template = "# {{SEGMENT_ID}}\n\nThis segment was automatically repaired.\n\nOriginal {{MEMORY_TYPE}} content was not recoverable.";
        fs::write(templates_dir.join("generic.template"), generic_template)?;
    }
    
    // Load templates from directory
    for entry in fs::read_dir(templates_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "template") {
            let issue_type = path.file_stem().unwrap().to_string_lossy().to_string();
            let template_content = fs::read_to_string(&path)?;
            
            templates.insert(issue_type, template_content);
        }
    }
    
    // Add generic template if not present
    if !templates.contains_key("generic") {
        templates.insert(
            "generic".to_string(),
            "# {{SEGMENT_ID}}\n\nThis segment was automatically repaired.\n\nOriginal {{MEMORY_TYPE}} content was not recoverable.".to_string()
        );
    }
    
    Ok(templates)
}

// Thread-local functions for background integrity checking

// Load backup information from directory
fn load_backup_info_from_dir(backup_dir: &Path) -> Result<HashMap<String, Vec<BackupInfo>>, Box<dyn Error>> {
    let backups_file = backup_dir.join("backups.json");
    
    if backups_file.exists() {
        let content = fs::read_to_string(&backups_file)?;
        let backups: HashMap<String, Vec<BackupInfo>> = serde_json::from_str(&content)?;
        Ok(backups)
    } else {
        Ok(HashMap::new())
    }
}

// Save backup information
fn save_backup_info(backup_dir: &Path, backups: &HashMap<String, Vec<BackupInfo>>) -> Result<(), Box<dyn Error>> {
    let backups_json = serde_json::to_string_pretty(backups)?;
    let backups_file = backup_dir.join("backups.json");
    fs::write(&backups_file, backups_json)?;
    Ok(())
}

// Save integrity results
fn save_integrity_results(backup_dir: &Path, results: &HashMap<PathBuf, IntegrityCheckResult>) -> Result<(), Box<dyn Error>> {
    let mut results_map = HashMap::new();
    
    for (path, result) in results {
        results_map.insert(path.to_string_lossy().to_string(), result.clone());
    }
    
    let results_json = serde_json::to_string_pretty(&results_map)?;
    let results_file = backup_dir.join("integrity_results.json");
    fs::write(&results_file, results_json)?;
    
    Ok(())
}

// Check integrity of all agent memory files
fn check_agent_integrity(
    agent_name: &str,
    agent_dir: &PathBuf,
    parser: &mut MemorySegmentParser,
    config: &MemoryRecoveryConfig,
    repair_templates: &HashMap<String, String>,
    memory_updater: &Option<Arc<Mutex<MemoryUpdater>>>,
    stats: &Arc<RwLock<RecoveryStats>>
) -> Result<HashMap<PathBuf, IntegrityCheckResult>, Box<dyn Error>> {
    let mut results = HashMap::new();
    
    // Get agent memory files
    let memory_files = get_agent_memory_files(agent_dir)?;
    
    // Check each memory file
    for file_path in memory_files {
        if !file_path.exists() {
            continue;
        }
        
        // Check file integrity
        match check_file_integrity(&file_path, agent_name, parser) {
            Ok(result) => {
                // Store result
                results.insert(file_path.clone(), result);
            },
            Err(e) => {
                error!("Failed to check integrity of file {}: {}", file_path.display(), e);
            }
        }
    }
    
    // Update statistics
    if let Ok(mut stats_write) = stats.write() {
        stats_write.total_checks += 1;
        stats_write.files_checked += results.len() as u64;
        stats_write.last_check = Some(SystemTime::now());
        
        // Update agent check results and count issues
        for (_, result) in &results {
            stats_write.agent_check_results.insert(agent_name.to_string(), result.clone());
            stats_write.issues_detected += result.segments_with_issues as u64;
        }
    }
    
    Ok(results)
}

// Check integrity of a file in thread
fn check_file_integrity(file_path: &Path, agent_name: &str, parser: &mut MemorySegmentParser) -> Result<IntegrityCheckResult, Box<dyn Error>> {
    // Try to read file content
    let content = fs::read_to_string(file_path)?;
    
    // Parse segments
    let segments = match parser.parse_file(file_path) {
        Ok(segs) => segs,
        Err(e) => {
            // File is corrupted if we can't parse it
            return Ok(IntegrityCheckResult {
                file_path: file_path.to_path_buf(),
                agent_name: agent_name.to_string(),
                check_time: SystemTime::now(),
                status: IntegrityStatus::Corrupted,
                issues: vec![
                    SegmentIssue {
                        segment_id: "file".to_string(),
                        issue_type: "parse_error".to_string(),
                        description: format!("File parsing failed: {}", e),
                        severity: IntegrityStatus::Critical,
                        detected_at: SystemTime::now(),
                        resolved_at: None,
                        repair_attempts: 0,
                        is_quarantined: false,
                    }
                ],
                segments_checked: 0,
                segments_with_issues: 1,
                error_percentage: 1.0,
                is_recovered: false,
                recovery_source: None,
                recovery_time: None,
            });
        }
    };
    
    // Check each segment for issues
    let mut issues = Vec::new();
    
    for segment in &segments {
        // Perform integrity checks on segment
        let segment_issues = check_segment_integrity(segment);
        issues.extend(segment_issues);
    }
    
    // Calculate error percentage
    let segments_with_issues = issues.len();
    let error_percentage = if segments.len() > 0 {
        segments_with_issues as f64 / segments.len() as f64
    } else {
        0.0
    };
    
    // Determine overall status
    let status = if segments_with_issues == 0 {
        IntegrityStatus::Good
    } else if error_percentage < 0.05 {
        IntegrityStatus::Minor
    } else if error_percentage < 0.2 {
        IntegrityStatus::Major
    } else {
        IntegrityStatus::Critical
    };
    
    // Create result
    let result = IntegrityCheckResult {
        file_path: file_path.to_path_buf(),
        agent_name: agent_name.to_string(),
        check_time: SystemTime::now(),
        status,
        issues,
        segments_checked: segments.len(),
        segments_with_issues,
        error_percentage,
        is_recovered: false,
        recovery_source: None,
        recovery_time: None,
    };
    
    Ok(result)
}

// Check segment integrity in thread
fn check_segment_integrity(segment: &MemorySegment) -> Vec<SegmentIssue> {
    let mut issues = Vec::new();
    
    // Check for empty content
    if segment.content.trim().is_empty() {
        issues.push(SegmentIssue {
            segment_id: segment.id.clone(),
            issue_type: "empty_content".to_string(),
            description: "Segment has empty content".to_string(),
            severity: IntegrityStatus::Minor,
            detected_at: SystemTime::now(),
            resolved_at: None,
            repair_attempts: 0,
            is_quarantined: false,
        });
    }
    
    // Check for malformed markdown (simple checks)
    if segment.content.contains("# ") && !segment.content.contains("\n# ") && !segment.content.starts_with("# ") {
        issues.push(SegmentIssue {
            segment_id: segment.id.clone(),
            issue_type: "malformed_markdown".to_string(),
            description: "Segment contains malformed markdown headers".to_string(),
            severity: IntegrityStatus::Minor,
            detected_at: SystemTime::now(),
            resolved_at: None,
            repair_attempts: 0,
            is_quarantined: false,
        });
    }
    
    // Check for unterminated code blocks
    let code_block_starts = segment.content.matches("```").count();
    if code_block_starts % 2 != 0 {
        issues.push(SegmentIssue {
            segment_id: segment.id.clone(),
            issue_type: "unterminated_code_block".to_string(),
            description: "Segment contains unterminated code blocks".to_string(),
            severity: IntegrityStatus::Minor,
            detected_at: SystemTime::now(),
            resolved_at: None,
            repair_attempts: 0,
            is_quarantined: false,
        });
    }
    
    // Check for unbalanced parentheses, brackets, etc.
    let parentheses_balance = segment.content.chars().filter(|&c| c == '(').count() - 
                             segment.content.chars().filter(|&c| c == ')').count();
    let brackets_balance = segment.content.chars().filter(|&c| c == '[').count() - 
                          segment.content.chars().filter(|&c| c == ']').count();
    let braces_balance = segment.content.chars().filter(|&c| c == '{').count() - 
                        segment.content.chars().filter(|&c| c == '}').count();
    
    if parentheses_balance != 0 || brackets_balance != 0 || braces_balance != 0 {
        issues.push(SegmentIssue {
            segment_id: segment.id.clone(),
            issue_type: "unbalanced_delimiters".to_string(),
            description: "Segment contains unbalanced parentheses, brackets, or braces".to_string(),
            severity: IntegrityStatus::Minor,
            detected_at: SystemTime::now(),
            resolved_at: None,
            repair_attempts: 0,
            is_quarantined: false,
        });
    }
    
    issues
}

// Create backup of agent memory
fn backup_agent_memory(
    agent_name: &str,
    agent_dir: &PathBuf,
    config: &MemoryRecoveryConfig,
    backups: &mut HashMap<String, Vec<BackupInfo>>,
    stats: &Arc<RwLock<RecoveryStats>>
) -> Result<Vec<BackupInfo>, Box<dyn Error>> {
    let mut new_backups = Vec::new();
    
    // Get agent memory files
    let memory_files = get_agent_memory_files(agent_dir)?;
    
    // Create backup dir for this agent
    let agent_backup_dir = config.backup_dir.join(agent_name);
    fs::create_dir_all(&agent_backup_dir)?;
    
    // Generate timestamp for backup files
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    // Backup each memory file
    for file_path in memory_files {
        if !file_path.exists() {
            continue;
        }
        
        let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
        let backup_name = format!("{}_{}.bak", file_name, timestamp);
        let backup_path = agent_backup_dir.join(backup_name);
        
        // Copy file to backup
        fs::copy(&file_path, &backup_path)?;
        
        // Get file size
        let metadata = fs::metadata(&backup_path)?;
        let size_bytes = metadata.len();
        
        // Calculate file hash
        let content = fs::read_to_string(&file_path)?;
        let hash = format!("{:x}", md5::compute(&content));
        
        // Create backup info
        let backup_info = BackupInfo {
            original_path: file_path.clone(),
            backup_path: backup_path.clone(),
            created_at: SystemTime::now(),
            size_bytes,
            hash,
            integrity_status: IntegrityStatus::Good, // Assume backup is good
            metadata: HashMap::new(),
        };
        
        new_backups.push(backup_info);
    }
    
    // Update backup list for this agent
    if !backups.contains_key(agent_name) {
        backups.insert(agent_name.to_string(), Vec::new());
    }
    
    let agent_backups = backups.get_mut(agent_name).unwrap();
    
    // Add new backups
    for backup in &new_backups {
        agent_backups.push(backup.clone());
    }
    
    // Limit number of backups
    if agent_backups.len() > config.max_backups_per_agent {
        // Sort by creation time (oldest first)
        agent_backups.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        
        // Remove excess backups
        while agent_backups.len() > config.max_backups_per_agent {
            if let Some(old_backup) = agent_backups.first() {
                // Try to remove the backup file
                if old_backup.backup_path.exists() {
                    if let Err(e) = fs::remove_file(&old_backup.backup_path) {
                        warn!("Failed to remove old backup file {}: {}", old_backup.backup_path.display(), e);
                    }
                }
            }
            
            agent_backups.remove(0);
        }
    }
    
    // Update statistics
    if let Ok(mut stats_write) = stats.write() {
        stats_write.backups_created += new_backups.len() as u64;
    }
    
    Ok(new_backups)
}

// Recover a file from backup in thread
fn recover_agent_memory(
    agent_name: &str,
    file_path: &Path,
    backups: &HashMap<String, Vec<BackupInfo>>,
    config: &MemoryRecoveryConfig,
    parser: &mut MemorySegmentParser,
    memory_updater: &Option<Arc<Mutex<MemoryUpdater>>>,
    stats: &Arc<RwLock<RecoveryStats>>
) -> Result<(), Box<dyn Error>> {
    // Get backups for this agent
    let agent_backups = match backups.get(agent_name) {
        Some(backups) => backups,
        None => return Err(format!("No backups found for agent: {}", agent_name).into()),
    };
    
    // Find backups for this file
    let file_backups: Vec<&BackupInfo> = agent_backups.iter()
        .filter(|b| b.original_path == file_path)
        .collect();
    
    if file_backups.is_empty() {
        return Err(format!("No backups found for file: {}", file_path.display()).into());
    }
    
    // Sort backups by creation time (newest first)
    let mut sorted_backups = file_backups.clone();
    sorted_backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    
    // Use the newest backup with good integrity
    let good_backup = sorted_backups.iter()
        .find(|b| matches!(b.integrity_status, IntegrityStatus::Good));
    
    let backup_to_use = good_backup.unwrap_or(&sorted_backups[0]);
    
    // Create a timestamped backup of the current file before replacing
    let current_backup_dir = config.backup_dir.join(format!("recovery_backups/{}", agent_name));
    fs::create_dir_all(&current_backup_dir)?;
    
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
    let current_backup_path = current_backup_dir.join(format!("{}_{}_pre_recovery.bak", file_name, timestamp));
    
    // Backup current file if it exists
    if file_path.exists() {
        fs::copy(file_path, &current_backup_path)?;
    }
    
    // Restore from backup
    fs::copy(&backup_to_use.backup_path, file_path)?;
    
    info!("Recovered file {} from backup {}", file_path.display(), backup_to_use.backup_path.display());
    
    // Update statistics
    if let Ok(mut stats_write) = stats.write() {
        stats_write.auto_recoveries += 1;
    }
    
    Ok(())
}

// Example usage function
pub fn main() -> Result<(), Box<dyn Error>> {
    // Set up logging
    env_logger::init();
    
    // Create memory recovery system with default config
    let config = MemoryRecoveryConfig::default();
    let mut recovery = MemoryRecovery::new(config)?;
    
    // Example: Check integrity of an agent
    let agent_name = "Athena";
    
    println!("Checking integrity for agent {}...", agent_name);
    match recovery.check_agent_integrity(agent_name) {
        Ok(results) => {
            println!("Integrity check completed for {} files", results.len());
            
            for (file_path, result) in &results {
                println!("  File: {}", file_path.display());
                println!("  Status: {:?}", result.status);
                println!("  Issues: {}/{} segments", result.segments_with_issues, result.segments_checked);
                println!("  Error rate: {:.1}%", result.error_percentage * 100.0);
                
                if !result.issues.is_empty() {
                    println!("  Issues:");
                    for issue in &result.issues {
                        println!("    - Segment {}: {} ({:?})", issue.segment_id, issue.issue_type, issue.severity);
                    }
                }
                
                println!();
            }
        },
        Err(e) => {
            println!("Error checking integrity: {}", e);
        }
    }
    
    // Example: Create a backup
    println!("Creating backup for agent {}...", agent_name);
    match recovery.backup_agent_memory(agent_name) {
        Ok(backups) => {
            println!("Created {} backups", backups.len());
            
            for backup in &backups {
                println!("  Backup: {}", backup.backup_path.display());
                println!("  Size: {} bytes", backup.size_bytes);
                println!("  Hash: {}", backup.hash);
                println!();
            }
        },
        Err(e) => {
            println!("Error creating backup: {}", e);
        }
    }
    
    // Example: Generate report
    let report = recovery.format_integrity_report(Some(agent_name));
    println!("\nIntegrity Report:\n{}", report);
    
    // Start background integrity checks
    recovery.start_background_checks()?;
    
    // In a real application, this would run indefinitely
    println!("Background integrity checks started. Press Ctrl+C to stop.");
    thread::sleep(Duration::from_secs(5));
    
    // Stop background checks
    recovery.stop_background_checks()?;
    
    Ok(())
}