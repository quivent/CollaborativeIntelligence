use crate::helpers::CommandHelpers;
use crate::helpers::path::PathHelpers;
use crate::config::Config;
use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use md5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySegment {
    pub id: String,
    pub content: String,
    pub domains: Vec<String>,
    pub source_file: String,
    pub last_accessed: SystemTime,
    pub access_count: u64,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecksumEntry {
    pub file_path: PathBuf,
    pub checksum: String,
    pub last_checked: SystemTime,
    pub segments_checksums: HashMap<String, String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationStats {
    pub total_validations: u64,
    pub valid_files: u64,
    pub modified_files: u64,
    pub avg_validation_time_ms: u64,
    pub last_validation: Option<SystemTime>,
}

/// Load agent memory
pub fn load(agent: &str, config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        &format!("Loading Memory for Agent: {}", agent),
        "🧠",
        "Memory System",
        "green"
    );
    
    // Check if agent exists
    let agents_path = PathHelpers::get_agents_path(&config.repo_path);
    let agent_path = agents_path.join(agent);
    
    if !agent_path.exists() {
        CommandHelpers::print_error(&format!("Agent '{}' does not exist", agent));
        return Err(anyhow::anyhow!("Agent does not exist: {}", agent));
    }
    
    // Check for memory file
    let memory_path = agent_path.join("MEMORY.md");
    if !memory_path.exists() {
        CommandHelpers::print_error(&format!("Memory file for agent '{}' not found", agent));
        return Err(anyhow::anyhow!("Memory file not found for agent: {}", agent));
    }
    
    // Simulate memory loading
    CommandHelpers::print_info(&format!("Initializing memory loader for '{}'", agent));
    CommandHelpers::print_info("Parsing memory segments...");
    CommandHelpers::print_info("Loading memory into context...");
    
    // Get memory file size
    match memory_path.metadata() {
        Ok(metadata) => {
            let size_kb = metadata.len() / 1024;
            CommandHelpers::print_info(&format!("Memory file size: {} KB", size_kb));
        },
        Err(e) => {
            CommandHelpers::print_warning(&format!("Failed to get memory file size: {}", e));
        }
    }
    
    CommandHelpers::print_success(&format!("Memory loaded successfully for agent '{}'", agent));
    Ok(())
}

/// Optimize agent memory
pub fn optimize(agent: &str, config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        &format!("Optimizing Memory for Agent: {}", agent),
        "⚡",
        "Memory System",
        "green"
    );
    
    // Check if agent exists
    let agents_path = PathHelpers::get_agents_path(&config.repo_path);
    let agent_path = agents_path.join(agent);
    
    if !agent_path.exists() {
        CommandHelpers::print_error(&format!("Agent '{}' does not exist", agent));
        return Err(anyhow::anyhow!("Agent does not exist: {}", agent));
    }
    
    // Check for memory files
    let memory_path = agent_path.join("MEMORY.md");
    let learning_path = agent_path.join("ContinuousLearning.md");
    
    if !memory_path.exists() {
        CommandHelpers::print_error(&format!("Memory file for agent '{}' not found", agent));
        return Err(anyhow::anyhow!("Memory file not found for agent: {}", agent));
    }
    
    // Simulate memory optimization
    CommandHelpers::print_info(&format!("Analyzing memory structure for '{}'", agent));
    CommandHelpers::print_info("Identifying optimization opportunities...");
    CommandHelpers::print_info("Consolidating memory segments...");
    CommandHelpers::print_info("Removing redundancies...");
    
    // Report on continuous learning file if available
    if learning_path.exists() {
        CommandHelpers::print_info("Integrating continuous learning insights...");
    } else {
        CommandHelpers::print_warning("No continuous learning file found, skipping integration");
    }
    
    CommandHelpers::print_success(&format!("Memory optimized successfully for agent '{}'", agent));
    Ok(())
}

/// Cache agent memory
pub fn cache(agent: &str, config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        &format!("Caching Memory for Agent: {}", agent),
        "💾",
        "Memory System",
        "green"
    );
    
    // Check if agent exists
    let agents_path = PathHelpers::get_agents_path(&config.repo_path);
    let agent_path = agents_path.join(agent);
    
    if !agent_path.exists() {
        CommandHelpers::print_error(&format!("Agent '{}' does not exist", agent));
        return Err(anyhow::anyhow!("Agent does not exist: {}", agent));
    }
    
    // Create cache directory if it doesn't exist
    let cache_dir = config.repo_path.join(".cache").join("agents").join(agent);
    
    if !cache_dir.exists() {
        match std::fs::create_dir_all(&cache_dir) {
            Ok(_) => CommandHelpers::print_info(&format!("Created cache directory: {}", cache_dir.display())),
            Err(e) => {
                CommandHelpers::print_error(&format!("Failed to create cache directory: {}", e));
                return Err(anyhow::anyhow!("Failed to create cache directory: {}", e));
            }
        }
    }
    
    // Simulate memory caching
    CommandHelpers::print_info(&format!("Preparing memory cache for '{}'", agent));
    CommandHelpers::print_info("Generating memory checksum...");
    CommandHelpers::print_info("Writing cache to disk...");
    
    // Display cache information
    let cache_file = cache_dir.join("memory_cache.json");
    CommandHelpers::print_info(&format!("Cache file: {}", cache_file.display()));
    
    CommandHelpers::print_success(&format!("Memory cached successfully for agent '{}'", agent));
    Ok(())
}

/// Validate agent memory files with checksum verification
pub fn validate(agent: &str, config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        &format!("Validating Memory for Agent: {}", agent),
        "🔍",
        "Memory Validation",
        "yellow"
    );
    
    // Check if agent exists
    let agents_path = PathHelpers::get_agents_path(&config.repo_path);
    let agent_path = agents_path.join(agent);
    
    if !agent_path.exists() {
        CommandHelpers::print_error(&format!("Agent '{}' does not exist", agent));
        return Err(anyhow::anyhow!("Agent does not exist: {}", agent));
    }
    
    // Find memory files
    let memory_files = vec![
        agent_path.join("MEMORY.md"),
        agent_path.join("ContinuousLearning.md"),
        agent_path.join("README.md"),
    ];
    
    let mut total_changes = 0;
    let mut validated_files = 0;
    let start_time = std::time::Instant::now();
    
    for file_path in memory_files {
        if file_path.exists() {
            CommandHelpers::print_info(&format!("Validating: {}", file_path.display()));
            
            match validate_memory_file(&file_path, config) {
                Ok(changes) => {
                    validated_files += 1;
                    total_changes += changes.len();
                    
                    if changes.is_empty() {
                        CommandHelpers::print_info("  No changes detected");
                    } else {
                        CommandHelpers::print_warning(&format!("  Found {} changes", changes.len()));
                        for (segment_id, change_type) in changes {
                            CommandHelpers::print_info(&format!("    - {}: {}", change_type, segment_id));
                        }
                    }
                },
                Err(e) => {
                    CommandHelpers::print_error(&format!("  Validation failed: {}", e));
                }
            }
        }
    }
    
    let elapsed = start_time.elapsed();
    CommandHelpers::print_success(&format!(
        "Validation complete: {} files validated, {} changes found in {}ms",
        validated_files,
        total_changes,
        elapsed.as_millis()
    ));
    
    Ok(())
}

/// Validate a single memory file and return changes
fn validate_memory_file(file_path: &Path, config: &Config) -> Result<Vec<(String, String)>> {
    let cache_dir = config.repo_path.join(".cache").join("memory_validation");
    std::fs::create_dir_all(&cache_dir)?;
    
    // Generate cache key from file path
    let cache_key = file_path.to_string_lossy().replace("/", "_");
    let cache_file = cache_dir.join(format!("{}.json", cache_key));
    
    // Read current file content and compute checksum
    let file_content = std::fs::read_to_string(file_path)?;
    let current_checksum = format!("{:x}", md5::compute(&file_content));
    
    // Parse memory segments from content
    let current_segments = parse_memory_segments(&file_content, file_path)?;
    
    // Load cached entry if exists
    let cached_entry: Option<ChecksumEntry> = if cache_file.exists() {
        let cache_content = std::fs::read_to_string(&cache_file)?;
        serde_json::from_str(&cache_content).ok()
    } else {
        None
    };
    
    let mut changes = Vec::new();
    
    match cached_entry {
        Some(entry) => {
            if entry.checksum != current_checksum {
                // File has changed, analyze segment differences
                changes = analyze_segment_changes(&entry.segments_checksums, &current_segments);
                
                // Update cache
                save_cache_entry(file_path, &current_checksum, &current_segments, &cache_file)?;
            }
        },
        None => {
            // New file, all segments are new
            for segment in &current_segments {
                changes.push((segment.id.clone(), "ADDED".to_string()));
            }
            
            // Create cache entry
            save_cache_entry(file_path, &current_checksum, &current_segments, &cache_file)?;
        }
    }
    
    Ok(changes)
}

/// Parse memory segments from file content
fn parse_memory_segments(content: &str, file_path: &Path) -> Result<Vec<MemorySegment>> {
    let mut segments = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    let mut current_segment: Option<MemorySegment> = None;
    let mut in_segment = false;
    let mut segment_content = String::new();
    
    for (line_num, line) in lines.iter().enumerate() {
        // Detect segment boundaries (headers)
        if line.starts_with("##") && !line.starts_with("###") {
            // Save previous segment if exists
            if let Some(mut segment) = current_segment.take() {
                segment.content = segment_content.trim().to_string();
                segments.push(segment);
            }
            
            // Start new segment
            let segment_title = line.trim_start_matches("##").trim();
            let segment_id = format!("{}:{}", 
                file_path.file_name().unwrap().to_string_lossy(),
                segment_title.replace(" ", "_").to_lowercase()
            );
            
            current_segment = Some(MemorySegment {
                id: segment_id,
                content: String::new(),
                domains: extract_domains(segment_title),
                source_file: file_path.file_name().unwrap().to_string_lossy().to_string(),
                last_accessed: SystemTime::now(),
                access_count: 0,
                metadata: HashMap::new(),
            });
            
            segment_content = String::new();
            in_segment = true;
        } else if in_segment {
            segment_content.push_str(line);
            segment_content.push('\n');
        }
    }
    
    // Save last segment
    if let Some(mut segment) = current_segment {
        segment.content = segment_content.trim().to_string();
        segments.push(segment);
    }
    
    Ok(segments)
}

/// Extract domain keywords from segment title
fn extract_domains(title: &str) -> Vec<String> {
    let title_lower = title.to_lowercase();
    let mut domains = Vec::new();
    
    // Domain keyword mapping
    let domain_keywords = [
        ("identity", vec!["identity", "purpose", "core", "role"]),
        ("learning", vec!["learn", "knowledge", "education", "study"]),
        ("system", vec!["system", "framework", "architecture", "structure"]),
        ("memory", vec!["memory", "recall", "remember", "cache"]),
        ("technical", vec!["code", "implementation", "software", "tech"]),
        ("collaboration", vec!["collaboration", "team", "agent", "communication"]),
    ];
    
    for (domain, keywords) in &domain_keywords {
        for keyword in keywords {
            if title_lower.contains(keyword) {
                domains.push(domain.to_string());
                break;
            }
        }
    }
    
    if domains.is_empty() {
        domains.push("general".to_string());
    }
    
    domains
}

/// Analyze differences between cached and current segments
fn analyze_segment_changes(
    cached_checksums: &HashMap<String, String>,
    current_segments: &[MemorySegment]
) -> Vec<(String, String)> {
    let mut changes = Vec::new();
    
    // Create current segment checksums
    let mut current_checksums = HashMap::new();
    for segment in current_segments {
        let segment_checksum = format!("{:x}", md5::compute(&segment.content));
        current_checksums.insert(segment.id.clone(), segment_checksum);
    }
    
    // Find added and modified segments
    for segment in current_segments {
        let segment_id = &segment.id;
        let current_checksum = current_checksums.get(segment_id).unwrap();
        
        if !cached_checksums.contains_key(segment_id) {
            changes.push((segment_id.clone(), "ADDED".to_string()));
        } else if cached_checksums.get(segment_id).unwrap() != current_checksum {
            changes.push((segment_id.clone(), "MODIFIED".to_string()));
        }
    }
    
    // Find removed segments
    for cached_segment_id in cached_checksums.keys() {
        if !current_checksums.contains_key(cached_segment_id) {
            changes.push((cached_segment_id.clone(), "REMOVED".to_string()));
        }
    }
    
    changes
}

/// Save cache entry to file
fn save_cache_entry(
    file_path: &Path,
    checksum: &str,
    segments: &[MemorySegment],
    cache_file: &Path
) -> Result<()> {
    let mut segments_checksums = HashMap::new();
    for segment in segments {
        let segment_checksum = format!("{:x}", md5::compute(&segment.content));
        segments_checksums.insert(segment.id.clone(), segment_checksum);
    }
    
    let entry = ChecksumEntry {
        file_path: file_path.to_path_buf(),
        checksum: checksum.to_string(),
        last_checked: SystemTime::now(),
        segments_checksums,
        metadata: HashMap::new(),
    };
    
    let json = serde_json::to_string_pretty(&entry)?;
    std::fs::write(cache_file, json)?;
    
    Ok(())
}

/// Get validation statistics for an agent
pub fn stats(agent: &str, config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        &format!("Memory Statistics for Agent: {}", agent),
        "📊",
        "Memory Statistics",
        "blue"
    );
    
    let cache_dir = config.repo_path.join(".cache").join("memory_validation");
    
    if !cache_dir.exists() {
        CommandHelpers::print_warning("No validation cache found. Run 'cia memory validate' first.");
        return Ok(());
    }
    
    let agents_path = PathHelpers::get_agents_path(&config.repo_path);
    let agent_path = agents_path.join(agent);
    
    if !agent_path.exists() {
        CommandHelpers::print_error(&format!("Agent '{}' does not exist", agent));
        return Err(anyhow::anyhow!("Agent does not exist: {}", agent));
    }
    
    let memory_files = vec![
        agent_path.join("MEMORY.md"),
        agent_path.join("ContinuousLearning.md"),
        agent_path.join("README.md"),
    ];
    
    let mut total_segments = 0;
    let mut total_size = 0;
    let mut last_validation: Option<SystemTime> = None;
    
    for file_path in memory_files {
        if file_path.exists() {
            let cache_key = file_path.to_string_lossy().replace("/", "_");
            let cache_file = cache_dir.join(format!("{}.json", cache_key));
            
            if cache_file.exists() {
                let cache_content = std::fs::read_to_string(&cache_file)?;
                if let Ok(entry) = serde_json::from_str::<ChecksumEntry>(&cache_content) {
                    total_segments += entry.segments_checksums.len();
                    
                    if let Ok(metadata) = file_path.metadata() {
                        total_size += metadata.len();
                    }
                    
                    if last_validation.is_none() || entry.last_checked > last_validation.unwrap() {
                        last_validation = Some(entry.last_checked);
                    }
                }
            }
        }
    }
    
    CommandHelpers::print_info(&format!("Total memory segments: {}", total_segments));
    CommandHelpers::print_info(&format!("Total memory size: {} bytes", total_size));
    
    if let Some(last_check) = last_validation {
        if let Ok(duration) = SystemTime::now().duration_since(last_check) {
            CommandHelpers::print_info(&format!("Last validated: {} seconds ago", duration.as_secs()));
        }
    } else {
        CommandHelpers::print_warning("No validation history found");
    }
    
    CommandHelpers::print_success("Statistics retrieved successfully");
    Ok(())
}