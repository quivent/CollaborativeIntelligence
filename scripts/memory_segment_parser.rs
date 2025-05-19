use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use lazy_static::lazy_static;

// Memory segment types based on neuroscience model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryType {
    Identity,      // Core agent identity
    Semantic,      // Factual knowledge
    Episodic,      // Experience-based knowledge 
    Procedural,    // Skill-based knowledge
    Relational,    // Connections between concepts
}

// Tier level for loading priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryTier {
    Tier1,  // Critical, always loaded
    Tier2,  // Important, loaded in most contexts
    Tier3,  // Supplementary, loaded when specifically relevant
}

// Domain categorization for contextual loading
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MemoryDomain {
    pub name: String,
    pub keywords: Vec<String>,
}

// Memory segment structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySegment {
    pub id: String,
    pub memory_type: MemoryType,
    pub tier: MemoryTier,
    pub domains: Vec<String>,
    pub content: String,
    pub last_accessed: SystemTime,
    pub access_count: u32,
    pub source_file: String,
    pub line_start: usize,
    pub line_end: usize,
    pub metadata: HashMap<String, String>,
}

// Pattern for extracting memory segments from markdown files
// Using HTML comments as they won't interfere with markdown rendering
lazy_static! {
    static ref SEGMENT_PATTERN: Regex = Regex::new(r"(?s)<!--\s*MEMORY_SEGMENT\s*
        TYPE=(?P<type>[A-Z]+)\s*
        TIER=(?P<tier>\d+)\s*
        DOMAIN=(?P<domain>[A-Za-z,]+)\s*
        (?P<metadata>.*?)
        -->\s*
        (?P<content>.*?)
        <!--\s*END_MEMORY_SEGMENT\s*-->").expect("Invalid regex pattern");
    
    static ref AUTO_SEGMENT_PATTERN: Regex = Regex::new(r"(?s)##\s+(?P<heading>.*?)\s*\n(?P<content>.*?)(?=\n##|\z)").expect("Invalid regex pattern");
}

// Memory segment parser
pub struct MemorySegmentParser {
    domains: HashMap<String, MemoryDomain>,
}

impl MemorySegmentParser {
    // Create a new parser
    pub fn new() -> Self {
        MemorySegmentParser {
            domains: HashMap::new(),
        }
    }
    
    // Register a memory domain
    pub fn register_domain(&mut self, name: &str, keywords: Vec<String>) {
        self.domains.insert(name.to_string(), MemoryDomain {
            name: name.to_string(),
            keywords,
        });
    }
    
    // Parse memory segments from a markdown file
    pub fn parse_file(&self, file_path: &Path) -> Result<Vec<MemorySegment>, Box<dyn Error>> {
        let content = fs::read_to_string(file_path)?;
        let file_name = file_path.file_name()
            .ok_or("Invalid file path")?
            .to_string_lossy()
            .to_string();
        
        let mut segments = self.extract_marked_segments(&content, &file_name)?;
        
        // If no marked segments are found, try to auto-segment based on markdown structure
        if segments.is_empty() {
            segments = self.auto_segment_file(&content, &file_name)?;
        }
        
        Ok(segments)
    }
    
    // Extract explicitly marked segments
    fn extract_marked_segments(&self, content: &str, file_name: &str) -> Result<Vec<MemorySegment>, Box<dyn Error>> {
        let mut segments = Vec::new();
        let mut id_counter = 0;
        
        for cap in SEGMENT_PATTERN.captures_iter(content) {
            let memory_type = match cap.name("type").map(|m| m.as_str()) {
                Some("IDENTITY") => MemoryType::Identity,
                Some("SEMANTIC") => MemoryType::Semantic,
                Some("EPISODIC") => MemoryType::Episodic,
                Some("PROCEDURAL") => MemoryType::Procedural,
                Some("RELATIONAL") => MemoryType::Relational,
                _ => MemoryType::Semantic, // Default to semantic if unspecified
            };
            
            let tier = match cap.name("tier").map(|m| m.as_str()) {
                Some("1") => MemoryTier::Tier1,
                Some("2") => MemoryTier::Tier2,
                Some("3") => MemoryTier::Tier3,
                _ => MemoryTier::Tier3, // Default to lowest tier if unspecified
            };
            
            let domains = cap.name("domain")
                .map(|m| m.as_str().split(',')
                    .map(|s| s.trim().to_string())
                    .collect())
                .unwrap_or_else(|| vec!["general".to_string()]);
            
            let content_str = cap.name("content")
                .map(|m| m.as_str().trim())
                .unwrap_or("");
            
            // Parse metadata
            let metadata_str = cap.name("metadata").map(|m| m.as_str()).unwrap_or("");
            let metadata = self.parse_metadata(metadata_str);
            
            // Calculate line positions
            let line_start = content[..cap.get(0).unwrap().start()]
                .chars()
                .filter(|&c| c == '\n')
                .count() + 1;
            
            let line_end = line_start + content_str
                .chars()
                .filter(|&c| c == '\n')
                .count() + 1;
            
            id_counter += 1;
            
            segments.push(MemorySegment {
                id: format!("{}_{}", file_name, id_counter),
                memory_type,
                tier,
                domains,
                content: content_str.to_string(),
                last_accessed: SystemTime::now(),
                access_count: 0,
                source_file: file_name.to_string(),
                line_start,
                line_end,
                metadata,
            });
        }
        
        Ok(segments)
    }
    
    // Automatically segment a file based on markdown structure
    fn auto_segment_file(&self, content: &str, file_name: &str) -> Result<Vec<MemorySegment>, Box<dyn Error>> {
        let mut segments = Vec::new();
        let mut id_counter = 0;
        
        // First try to find markdown sections based on headers
        for cap in AUTO_SEGMENT_PATTERN.captures_iter(content) {
            let heading = cap.name("heading").map(|m| m.as_str()).unwrap_or("");
            let content_str = cap.name("content").map(|m| m.as_str()).unwrap_or("");
            
            // Skip empty sections
            if content_str.trim().is_empty() {
                continue;
            }
            
            // Auto-classify based on heading content
            let (memory_type, tier) = self.auto_classify_section(heading, content_str);
            
            // Auto-detect domains
            let domains = self.detect_domains(heading, content_str);
            
            // Calculate line positions
            let line_start = content[..cap.get(0).unwrap().start()]
                .chars()
                .filter(|&c| c == '\n')
                .count() + 1;
            
            let line_end = line_start + content_str
                .chars()
                .filter(|&c| c == '\n')
                .count() + 1;
            
            id_counter += 1;
            
            // Create metadata
            let mut metadata = HashMap::new();
            metadata.insert("heading".to_string(), heading.to_string());
            metadata.insert("auto_segmented".to_string(), "true".to_string());
            
            segments.push(MemorySegment {
                id: format!("{}_{}", file_name, id_counter),
                memory_type,
                tier,
                domains,
                content: format!("## {}\n{}", heading, content_str),
                last_accessed: SystemTime::now(),
                access_count: 0,
                source_file: file_name.to_string(),
                line_start,
                line_end,
                metadata,
            });
        }
        
        // If we still don't have segments, treat the whole file as one segment
        if segments.is_empty() && !content.trim().is_empty() {
            // Auto-classify based on file name and content
            let (memory_type, tier) = self.auto_classify_file(file_name, content);
            
            // Auto-detect domains
            let domains = self.detect_domains(file_name, content);
            
            // Create metadata
            let mut metadata = HashMap::new();
            metadata.insert("file_name".to_string(), file_name.to_string());
            metadata.insert("auto_segmented".to_string(), "true".to_string());
            
            segments.push(MemorySegment {
                id: format!("{}_1", file_name),
                memory_type,
                tier,
                domains,
                content: content.to_string(),
                last_accessed: SystemTime::now(),
                access_count: 0,
                source_file: file_name.to_string(),
                line_start: 1,
                line_end: content.chars().filter(|&c| c == '\n').count() + 1,
                metadata,
            });
        }
        
        Ok(segments)
    }
    
    // Parse metadata from string
    fn parse_metadata(&self, metadata_str: &str) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        
        for line in metadata_str.lines() {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim();
                metadata.insert(key.to_string(), value.to_string());
            }
        }
        
        metadata
    }
    
    // Auto-classify a section based on heading and content
    fn auto_classify_section(&self, heading: &str, content: &str) -> (MemoryType, MemoryTier) {
        // Default type and tier
        let mut memory_type = MemoryType::Semantic;
        let mut tier = MemoryTier::Tier3;
        
        let heading_lower = heading.to_lowercase();
        let content_lower = content.to_lowercase();
        
        // Determine memory type based on heading and content patterns
        if heading_lower.contains("identity") || 
           heading_lower.contains("core") ||
           heading_lower.contains("foundational") ||
           heading_lower.contains("purpose") {
            memory_type = MemoryType::Identity;
            tier = MemoryTier::Tier1; // Identity is typically highest priority
        } else if heading_lower.contains("experience") ||
                 heading_lower.contains("session") ||
                 heading_lower.contains("history") ||
                 heading_lower.contains("error") ||
                 heading_lower.contains("failure") {
            memory_type = MemoryType::Episodic;
            tier = MemoryTier::Tier2;
        } else if heading_lower.contains("process") ||
                 heading_lower.contains("procedure") ||
                 heading_lower.contains("skill") ||
                 heading_lower.contains("technique") ||
                 heading_lower.contains("implementation") {
            memory_type = MemoryType::Procedural;
            tier = MemoryTier::Tier2;
        } else if heading_lower.contains("relation") ||
                 heading_lower.contains("connection") ||
                 heading_lower.contains("integration") ||
                 heading_lower.contains("linkage") {
            memory_type = MemoryType::Relational;
            tier = MemoryTier::Tier2;
        }
        
        // Check for importance markers in heading to adjust tier
        if heading_lower.contains("critical") ||
           heading_lower.contains("priority") ||
           heading_lower.contains("essential") ||
           heading_lower.contains("important") ||
           heading_lower.contains("key") {
            // Promote tier based on importance
            tier = MemoryTier::Tier1;
        } else if heading_lower.contains("secondary") ||
                 heading_lower.contains("notable") {
            tier = MemoryTier::Tier2;
        }
        
        (memory_type, tier)
    }
    
    // Auto-classify a file based on name and content
    fn auto_classify_file(&self, file_name: &str, content: &str) -> (MemoryType, MemoryTier) {
        let file_lower = file_name.to_lowercase();
        
        // Special handling for known memory file types
        if file_lower == "memory.md" {
            return (MemoryType::Semantic, MemoryTier::Tier1);
        } else if file_lower == "continuouslearning.md" {
            return (MemoryType::Episodic, MemoryTier::Tier2);
        } else if file_lower.contains("readme") {
            return (MemoryType::Identity, MemoryTier::Tier1);
        }
        
        // Default to semantic content with lowest tier
        (MemoryType::Semantic, MemoryTier::Tier3)
    }
    
    // Auto-detect domains based on content keywords
    fn detect_domains(&self, heading: &str, content: &str) -> Vec<String> {
        let mut domains = vec!["general".to_string()]; // Default domain
        let text = format!("{} {}", heading, content).to_lowercase();
        
        // Check registered domains for keyword matches
        for (name, domain) in &self.domains {
            for keyword in &domain.keywords {
                if text.contains(&keyword.to_lowercase()) {
                    if !domains.contains(&name.to_string()) {
                        domains.push(name.to_string());
                    }
                    break;
                }
            }
        }
        
        domains
    }
    
    // Convert existing files to segmented format
    pub fn convert_to_segmented(&self, file_path: &Path) -> Result<String, Box<dyn Error>> {
        let content = fs::read_to_string(file_path)?;
        let segments = self.auto_segment_file(&content, &file_path.to_string_lossy())?;
        
        // Create new content with explicit segment markers
        let mut new_content = String::new();
        
        for segment in segments {
            // Create segment marker
            new_content.push_str(&format!(
                "<!-- MEMORY_SEGMENT\nTYPE={:?}\nTIER={}\nDOMAIN={}\n",
                segment.memory_type,
                match segment.tier {
                    MemoryTier::Tier1 => "1",
                    MemoryTier::Tier2 => "2",
                    MemoryTier::Tier3 => "3",
                },
                segment.domains.join(",")
            ));
            
            // Add any additional metadata
            for (key, value) in segment.metadata {
                new_content.push_str(&format!("{}={}\n", key, value));
            }
            
            // Close the opening marker and add content
            new_content.push_str("-->\n");
            new_content.push_str(&segment.content);
            new_content.push_str("\n<!-- END_MEMORY_SEGMENT -->\n\n");
        }
        
        Ok(new_content)
    }
}

// Utility function for extracting a specific segment type from a file
pub fn extract_segment(file_path: &Path, memory_type: MemoryType, tier: Option<MemoryTier>, domain: Option<&str>) -> Result<Vec<MemorySegment>, Box<dyn Error>> {
    let parser = MemorySegmentParser::new();
    let segments = parser.parse_file(file_path)?;
    
    // Filter segments based on criteria
    let filtered = segments.into_iter().filter(|segment| {
        segment.memory_type == memory_type && 
        (tier.is_none() || segment.tier == tier.unwrap()) &&
        (domain.is_none() || segment.domains.iter().any(|d| d == domain.unwrap()))
    }).collect();
    
    Ok(filtered)
}

// Example function for implementing the API
pub fn main() -> Result<(), Box<dyn Error>> {
    // Example usage
    let test_path = Path::new("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS/Athena/MEMORY.md");
    
    // Create parser with some domains
    let mut parser = MemorySegmentParser::new();
    parser.register_domain("memory", vec!["memory".to_string(), "recall".to_string(), "remember".to_string()]);
    parser.register_domain("learning", vec!["learn".to_string(), "knowledge".to_string(), "education".to_string()]);
    parser.register_domain("system", vec!["system".to_string(), "process".to_string(), "framework".to_string()]);
    
    // Parse file
    let segments = parser.parse_file(test_path)?;
    
    println!("Found {} segments", segments.len());
    for segment in &segments {
        println!("Segment ID: {}", segment.id);
        println!("  Type: {:?}", segment.memory_type);
        println!("  Tier: {:?}", segment.tier);
        println!("  Domains: {:?}", segment.domains);
        println!("  Content length: {} chars", segment.content.len());
        println!("  Line range: {}-{}", segment.line_start, segment.line_end);
        println!();
    }
    
    // Example of converting a file to segmented format
    let segmented_content = parser.convert_to_segmented(test_path)?;
    println!("Converted file to segmented format ({}B)", segmented_content.len());
    
    // Example of extracting specific segments
    let identity_segments = extract_segment(test_path, MemoryType::Identity, Some(MemoryTier::Tier1), None)?;
    println!("Found {} identity segments in Tier 1", identity_segments.len());
    
    Ok(())
}