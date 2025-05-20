use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use regex::Regex;
use serde::{Serialize, Deserialize};

// Memory segment types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryType {
    Core,
    Learning,
    Context,
    Session,
    Cache,
    Custom,
}

// Memory tier (importance)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryTier {
    Critical,    // Essential for agent function
    Important,   // Highly relevant information
    Standard,    // Normal memory content
    Peripheral,  // Less important context
    Archival,    // Historical, rarely used
}

// Memory segment representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySegment {
    pub id: String,
    pub content: String,
    pub source_file: String,
    pub domains: Vec<String>,
    pub memory_type: MemoryType,
    pub tier: MemoryTier,
    pub timestamp: u64,
    pub metadata: HashMap<String, String>,
}

impl MemorySegment {
    pub fn new(
        id: &str,
        content: &str,
        source_file: &str,
        domains: Vec<String>,
        memory_type: MemoryType,
        tier: MemoryTier,
    ) -> Self {
        // Generate timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        Self {
            id: id.to_string(),
            content: content.to_string(),
            source_file: source_file.to_string(),
            domains,
            memory_type,
            tier,
            timestamp,
            metadata: HashMap::new(),
        }
    }
    
    // Add metadata to segment
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
    
    // Check if segment contains a specific term
    pub fn contains(&self, term: &str) -> bool {
        self.content.to_lowercase().contains(&term.to_lowercase())
    }
    
    // Get word count of segment
    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }
}

// Custom error type for segment parser
#[derive(Debug)]
pub enum SegmentParserError {
    IoError(std::io::Error),
    ParseError(String),
    ValidationError(String),
}

impl fmt::Display for SegmentParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SegmentParserError::IoError(e) => write!(f, "I/O error: {}", e),
            SegmentParserError::ParseError(e) => write!(f, "Parse error: {}", e),
            SegmentParserError::ValidationError(e) => write!(f, "Validation error: {}", e),
        }
    }
}

impl Error for SegmentParserError {}

impl From<std::io::Error> for SegmentParserError {
    fn from(err: std::io::Error) -> Self {
        SegmentParserError::IoError(err)
    }
}

// Result type for segment parser operations
pub type SegmentResult<T> = Result<T, SegmentParserError>;

// Memory segment parser
pub struct MemorySegmentParser {
    segment_size: usize,
    segment_overlap: usize,
    smart_segmentation: bool,
    heading_regex: Regex,
}

impl MemorySegmentParser {
    // Create a new parser
    pub fn new(segment_size: usize, segment_overlap: usize, smart_segmentation: bool) -> Self {
        Self {
            segment_size,
            segment_overlap,
            smart_segmentation,
            heading_regex: Regex::new(r"^#+\s+(.+)$").unwrap(),
        }
    }
    
    // Parse a file into segments
    pub fn parse_file(&self, file_path: &Path) -> SegmentResult<Vec<MemorySegment>> {
        // Read file content
        let content = fs::read_to_string(file_path)?;
        
        // Determine memory type based on file name
        let file_name = file_path.file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("");
        
        let memory_type = self.determine_memory_type(file_name);
        let tier = self.determine_memory_tier(file_name, &content);
        
        // Extract domain info from path
        let domains = self.extract_domains(file_path);
        
        // Get base ID from file name
        let base_id = file_path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        // Segment content
        if self.smart_segmentation {
            self.smart_segment(&content, file_path.to_string_lossy().to_string(), domains, memory_type, tier, &base_id)
        } else {
            self.fixed_segment(&content, file_path.to_string_lossy().to_string(), domains, memory_type, tier, &base_id)
        }
    }
    
    // Determine memory type from file name
    fn determine_memory_type(&self, file_name: &str) -> MemoryType {
        let lowercase = file_name.to_lowercase();
        
        if lowercase.contains("memory.md") {
            MemoryType::Core
        } else if lowercase.contains("continuouslearning") || lowercase.contains("learning") {
            MemoryType::Learning
        } else if lowercase.contains("session") || lowercase.contains("conversation") {
            MemoryType::Session
        } else if lowercase.contains("cache") {
            MemoryType::Cache
        } else {
            MemoryType::Context
        }
    }
    
    // Determine memory tier based on file and content
    fn determine_memory_tier(&self, file_name: &str, content: &str) -> MemoryTier {
        let lowercase_name = file_name.to_lowercase();
        
        // Check file name first
        if lowercase_name.contains("critical") {
            return MemoryTier::Critical;
        }
        
        if lowercase_name.contains("memory.md") {
            return MemoryTier::Critical;
        }
        
        if lowercase_name.contains("continuouslearning") {
            return MemoryTier::Important;
        }
        
        // Check content for indicators
        if content.to_lowercase().contains("critical") || content.contains("CRITICAL") {
            return MemoryTier::Critical;
        }
        
        if content.to_lowercase().contains("important") || content.contains("IMPORTANT") {
            return MemoryTier::Important;
        }
        
        // Default tier
        MemoryTier::Standard
    }
    
    // Extract domains from file path
    fn extract_domains(&self, file_path: &Path) -> Vec<String> {
        let mut domains = Vec::new();
        
        // Convert path to string representation
        let path_str = file_path.to_string_lossy();
        
        // Check for common domain indicators in path
        if path_str.contains("Agents") || path_str.contains("AGENTS") {
            domains.push("agent".to_string());
            
            // Try to extract agent name from path
            if let Some(agent_name) = path_str.split('/').find(|part| 
                part.contains("Agents") || part.contains("AGENTS")
            ).and_then(|_| 
                path_str.split('/').skip_while(|part| 
                    !part.contains("Agents") && !part.contains("AGENTS")
                ).nth(1)
            ) {
                domains.push(agent_name.to_lowercase());
            }
        }
        
        if path_str.contains("Sessions") {
            domains.push("session".to_string());
            
            // Try to extract session name
            if let Some(session_name) = path_str.split('/').find(|part| 
                part.contains("Sessions")
            ).and_then(|_| 
                path_str.split('/').skip_while(|part| 
                    !part.contains("Sessions")
                ).nth(1)
            ) {
                domains.push(session_name.to_lowercase());
            }
        }
        
        // Add file name without extension as domain
        if let Some(file_stem) = file_path.file_stem().and_then(|s| s.to_str()) {
            domains.push(file_stem.to_lowercase());
        }
        
        domains
    }
    
    // Segment content with fixed-size segments
    fn fixed_segment(
        &self,
        content: &str,
        source_file: String,
        domains: Vec<String>,
        memory_type: MemoryType,
        tier: MemoryTier,
        base_id: &str,
    ) -> SegmentResult<Vec<MemorySegment>> {
        let mut segments = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        // Calculate how many lines to use in each segment
        let segment_lines = self.segment_size;
        let overlap_lines = self.segment_overlap;
        
        // Process lines in chunks
        let mut start_line = 0;
        while start_line < lines.len() {
            // Determine end line for this segment
            let end_line = std::cmp::min(start_line + segment_lines, lines.len());
            
            // Create segment content
            let segment_content = lines[start_line..end_line].join("\n");
            
            // Skip empty segments
            if !segment_content.trim().is_empty() {
                // Create segment ID
                let segment_id = format!("{}_seg_{}", base_id, start_line / (segment_lines - overlap_lines));
                
                // Create segment
                let segment = MemorySegment::new(
                    &segment_id,
                    &segment_content,
                    &source_file,
                    domains.clone(),
                    memory_type,
                    tier,
                );
                
                segments.push(segment);
            }
            
            // Move to next chunk with overlap
            start_line = if overlap_lines >= segment_lines {
                start_line + 1 // Ensure at least 1 line of progress
            } else {
                start_line + segment_lines - overlap_lines
            };
        }
        
        Ok(segments)
    }
    
    // Segment content based on headings and semantic boundaries
    fn smart_segment(
        &self,
        content: &str,
        source_file: String,
        domains: Vec<String>,
        memory_type: MemoryType,
        tier: MemoryTier,
        base_id: &str,
    ) -> SegmentResult<Vec<MemorySegment>> {
        let mut segments = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        // First find all headings
        let mut heading_indices: Vec<(usize, String)> = Vec::new();
        
        for (i, line) in lines.iter().enumerate() {
            if let Some(captures) = self.heading_regex.captures(line) {
                if let Some(heading) = captures.get(1) {
                    heading_indices.push((i, heading.as_str().to_string()));
                }
            }
        }
        
        // If no headings found, fall back to fixed segmentation
        if heading_indices.is_empty() {
            return self.fixed_segment(content, source_file, domains, memory_type, tier, base_id);
        }
        
        // Process content between headings
        for i in 0..heading_indices.len() {
            let (start_idx, heading) = &heading_indices[i];
            
            // Determine end index (either next heading or end of content)
            let end_idx = if i < heading_indices.len() - 1 {
                heading_indices[i+1].0
            } else {
                lines.len()
            };
            
            // Create segment content
            let segment_content = lines[*start_idx..end_idx].join("\n");
            
            // Skip empty segments
            if !segment_content.trim().is_empty() {
                // Create segment ID
                let sanitized_heading = heading
                    .chars()
                    .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { '_' })
                    .collect::<String>()
                    .trim()
                    .replace(' ', "_");
                
                let segment_id = format!("{}_{}", base_id, sanitized_heading);
                
                // Create segment
                let segment = MemorySegment::new(
                    &segment_id,
                    &segment_content,
                    &source_file,
                    domains.clone(),
                    memory_type,
                    tier,
                );
                
                segments.push(segment);
            }
        }
        
        // If heading-based segmentation produced too few segments, apply further segmentation
        if segments.len() == 1 && segments[0].content.len() > self.segment_size * 2 {
            // Extract the single large segment
            let large_segment = segments.pop().unwrap();
            
            // Apply fixed segmentation to this large content
            let sub_segments = self.fixed_segment(
                &large_segment.content,
                source_file,
                domains,
                memory_type,
                tier,
                &large_segment.id,
            )?;
            
            segments.extend(sub_segments);
        }
        
        Ok(segments)
    }
    
    // Update parser configuration
    pub fn set_segment_size(&mut self, size: usize) {
        self.segment_size = size;
    }
    
    pub fn set_segment_overlap(&mut self, overlap: usize) {
        self.segment_overlap = overlap;
    }
    
    pub fn set_smart_segmentation(&mut self, enable: bool) {
        self.smart_segmentation = enable;
    }
}