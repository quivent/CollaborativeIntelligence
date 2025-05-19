use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};

use dashmap::DashMap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::memory_segment_parser::{MemorySegment, MemoryType};
use crate::memory_loader::MemoryLoader;

/// Compression strategy to apply to memory segments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompressionStrategy {
    /// Deduplicates semantically similar content
    SemanticDeduplication,
    /// Reduces detail level based on relevance/recency
    VariableResolution,
    /// Compresses old memories more aggressively
    TemporalCompression,
    /// Summarizes detailed procedural knowledge
    ProceduralSummarization,
    /// Keeps only essential identity information
    IdentityCompression,
}

/// Results of a compression operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionResult {
    /// Original size in bytes
    pub original_size: usize,
    /// Compressed size in bytes
    pub compressed_size: usize,
    /// Compression ratio (original/compressed)
    pub compression_ratio: f64,
    /// Strategy used for compression
    pub strategy: CompressionStrategy,
    /// Quality score (0.0-1.0) representing information retention
    pub quality_score: f64,
}

/// Configuration for memory compression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    /// Minimum age (in days) for a segment to be considered for compression
    pub min_segment_age_days: u32,
    /// Maximum compression ratio to apply (to prevent over-compression)
    pub max_compression_ratio: f64,
    /// Whether to enable semantic deduplication
    pub enable_semantic_deduplication: bool,
    /// Whether to enable variable resolution compression
    pub enable_variable_resolution: bool,
    /// Whether to enable temporal compression
    pub enable_temporal_compression: bool,
    /// Whether to enable procedural summarization
    pub enable_procedural_summarization: bool,
    /// Whether to enable identity compression
    pub enable_identity_compression: bool,
    /// Minimum activation score for a segment to avoid compression
    pub min_activation_score: f64,
    /// Reference embeddings for semantic similarity matching
    pub reference_embeddings: HashMap<String, Vec<f32>>,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            min_segment_age_days: 30,
            max_compression_ratio: 5.0,
            enable_semantic_deduplication: true,
            enable_variable_resolution: true,
            enable_temporal_compression: true,
            enable_procedural_summarization: true,
            enable_identity_compression: false, // Default to false for identity - more sensitive
            min_activation_score: 0.3,
            reference_embeddings: HashMap::new(),
        }
    }
}

/// Memory compression system for the Collaborative Intelligence platform
pub struct MemoryCompressor {
    /// Configuration for compression behavior
    config: CompressionConfig,
    /// Cache of already compressed segments (segment_id -> compressed_segment)
    compressed_segments: DashMap<String, Arc<MemorySegment>>,
    /// Compression results for metrics and analysis
    compression_results: DashMap<String, CompressionResult>,
    /// Reference to memory loader for retrieving segments
    memory_loader: Arc<Mutex<MemoryLoader>>,
    /// Semantic similarity calculator (using cosine similarity on embeddings)
    similarity_calculator: SemanticSimilarityCalculator,
}

impl MemoryCompressor {
    /// Create a new memory compressor with the given configuration
    pub fn new(config: CompressionConfig, memory_loader: Arc<Mutex<MemoryLoader>>) -> Self {
        Self {
            config,
            compressed_segments: DashMap::new(),
            compression_results: DashMap::new(),
            memory_loader,
            similarity_calculator: SemanticSimilarityCalculator::new(config.reference_embeddings.clone()),
        }
    }

    /// Create a new memory compressor with default configuration
    pub fn default(memory_loader: Arc<Mutex<MemoryLoader>>) -> Self {
        Self::new(CompressionConfig::default(), memory_loader)
    }

    /// Compress a single memory segment using the appropriate strategy
    pub fn compress_segment(&self, segment: &MemorySegment) -> Result<Arc<MemorySegment>, Box<dyn Error>> {
        // Skip if already compressed
        if let Some(cached) = self.compressed_segments.get(&segment.id) {
            return Ok(cached.clone());
        }

        // Choose appropriate compression strategy based on segment type
        let strategy = self.select_compression_strategy(segment);
        
        // Apply the selected compression strategy
        let (compressed_segment, result) = match strategy {
            CompressionStrategy::SemanticDeduplication => self.apply_semantic_deduplication(segment)?,
            CompressionStrategy::VariableResolution => self.apply_variable_resolution(segment)?,
            CompressionStrategy::TemporalCompression => self.apply_temporal_compression(segment)?,
            CompressionStrategy::ProceduralSummarization => self.apply_procedural_summarization(segment)?,
            CompressionStrategy::IdentityCompression => self.apply_identity_compression(segment)?,
        };

        // Cache the compressed segment
        let arc_segment = Arc::new(compressed_segment);
        self.compressed_segments.insert(segment.id.clone(), arc_segment.clone());
        self.compression_results.insert(segment.id.clone(), result);

        Ok(arc_segment)
    }

    /// Batch compress agent memory segments
    pub fn compress_agent_memory(&self, agent_name: &str) -> Result<Vec<CompressionResult>, Box<dyn Error>> {
        let memory_loader = self.memory_loader.lock().map_err(|e| format!("Failed to lock memory loader: {}", e))?;
        let segments = memory_loader.load_agent_memory(agent_name, None)?;
        drop(memory_loader); // Release lock

        // Filter segments eligible for compression
        let eligible_segments: Vec<_> = segments.into_iter()
            .filter(|s| self.is_eligible_for_compression(s))
            .collect();

        // Compress segments in parallel
        let results: Vec<_> = eligible_segments.par_iter()
            .filter_map(|segment| {
                match self.compress_segment(segment) {
                    Ok(_) => {
                        // Get the compression result from the cache
                        self.compression_results.get(&segment.id).map(|r| r.clone())
                    },
                    Err(e) => {
                        eprintln!("Failed to compress segment {}: {}", segment.id, e);
                        None
                    }
                }
            })
            .collect();

        Ok(results)
    }

    /// Select the best compression strategy for a given segment
    fn select_compression_strategy(&self, segment: &MemorySegment) -> CompressionStrategy {
        match segment.memory_type {
            MemoryType::Semantic => {
                if self.config.enable_semantic_deduplication {
                    CompressionStrategy::SemanticDeduplication
                } else {
                    CompressionStrategy::VariableResolution
                }
            },
            MemoryType::Episodic => {
                if self.config.enable_temporal_compression {
                    CompressionStrategy::TemporalCompression
                } else {
                    CompressionStrategy::VariableResolution
                }
            },
            MemoryType::Procedural => {
                if self.config.enable_procedural_summarization {
                    CompressionStrategy::ProceduralSummarization
                } else {
                    CompressionStrategy::VariableResolution
                }
            },
            MemoryType::Identity => {
                if self.config.enable_identity_compression {
                    CompressionStrategy::IdentityCompression
                } else {
                    // Identity is sensitive - use variable resolution as fallback
                    CompressionStrategy::VariableResolution
                }
            },
            MemoryType::Relational => {
                // Relational memory is best compressed with semantic deduplication
                CompressionStrategy::SemanticDeduplication
            },
        }
    }

    /// Check if a segment is eligible for compression
    fn is_eligible_for_compression(&self, segment: &MemorySegment) -> bool {
        // Skip compression for critical identity information
        if segment.memory_type == MemoryType::Identity && !self.config.enable_identity_compression {
            return false;
        }

        // Skip compression for recently used segments
        if segment.last_accessed_timestamp.is_some() {
            let days_since_access = segment.days_since_last_access();
            if days_since_access < self.config.min_segment_age_days {
                return false;
            }
        }

        // Skip compression for high-activation segments
        if segment.activation_score > self.config.min_activation_score {
            return false;
        }

        // Skip compression for already compressed segments
        // We check the compression marker in segment metadata
        if segment.metadata.get("compressed").map_or(false, |v| v == "true") {
            return false;
        }

        true
    }

    /// Apply semantic deduplication compression strategy
    fn apply_semantic_deduplication(&self, segment: &MemorySegment) -> Result<(MemorySegment, CompressionResult), Box<dyn Error>> {
        let original_content = &segment.content;
        let original_size = original_content.len();

        // Create paragraph embeddings
        let paragraphs: Vec<&str> = original_content.split("\n\n").collect();
        let mut unique_paragraphs = Vec::new();
        let mut seen_embeddings = HashSet::new();

        // Deduplicate semantically similar paragraphs
        for paragraph in paragraphs {
            if paragraph.trim().is_empty() {
                continue;
            }

            let paragraph_hash = self.similarity_calculator.calculate_semantic_hash(paragraph);
            
            // If we haven't seen a semantically similar paragraph, keep it
            if !seen_embeddings.contains(&paragraph_hash) {
                unique_paragraphs.push(paragraph);
                seen_embeddings.insert(paragraph_hash);
            }
        }

        // Recreate content with just unique paragraphs
        let compressed_content = unique_paragraphs.join("\n\n");
        let compressed_size = compressed_content.len();
        let compression_ratio = if compressed_size > 0 {
            original_size as f64 / compressed_size as f64
        } else {
            1.0
        };

        // Calculate quality score based on information retention
        // Simple heuristic: deduplication usually retains most information
        let quality_score = 0.95;

        // Create compressed segment
        let mut compressed_segment = segment.clone();
        compressed_segment.content = compressed_content;
        compressed_segment.metadata.insert("compressed".to_string(), "true".to_string());
        compressed_segment.metadata.insert("compression_strategy".to_string(), "semantic_deduplication".to_string());
        compressed_segment.metadata.insert("original_size".to_string(), original_size.to_string());

        let result = CompressionResult {
            original_size,
            compressed_size,
            compression_ratio,
            strategy: CompressionStrategy::SemanticDeduplication,
            quality_score,
        };

        Ok((compressed_segment, result))
    }

    /// Apply variable resolution compression strategy
    fn apply_variable_resolution(&self, segment: &MemorySegment) -> Result<(MemorySegment, CompressionResult), Box<dyn Error>> {
        let original_content = &segment.content;
        let original_size = original_content.len();

        // Split into sections
        let sections: Vec<&str> = original_content.split("\n## ").collect();
        let mut compressed_sections = Vec::new();

        // Process each section with varying levels of detail
        for (i, section) in sections.iter().enumerate() {
            if i == 0 && !section.starts_with("## ") {
                // First section without heading - keep as is
                compressed_sections.push(section.to_string());
                continue;
            }

            // Determine importance based on heading keywords
            let importance = if section.contains("critical") || 
                             section.contains("essential") || 
                             section.contains("important") {
                1.0 // Full detail
            } else if section.contains("background") || 
                      section.contains("context") || 
                      section.contains("example") {
                0.5 // Medium detail
            } else {
                0.3 // Low detail
            };

            // Apply appropriate resolution based on importance
            let compressed_section = if importance > 0.8 {
                // Keep full section
                section.to_string()
            } else if importance > 0.4 {
                // Keep heading and first sentence of each paragraph
                let heading = if i > 0 { format!("## {}", section.split("\n").next().unwrap_or("")) } 
                             else { section.split("\n").next().unwrap_or("").to_string() };
                
                let paragraphs: Vec<&str> = section.split("\n\n").collect();
                let compressed_paragraphs: Vec<String> = paragraphs.iter()
                    .map(|p| {
                        if p.contains(". ") {
                            p.split(". ").next().unwrap_or(p).to_string() + "."
                        } else {
                            p.to_string()
                        }
                    })
                    .collect();
                
                format!("{}\n\n{}", heading, compressed_paragraphs.join("\n\n"))
            } else {
                // Keep only heading and first paragraph
                let heading = if i > 0 { format!("## {}", section.split("\n").next().unwrap_or("")) } 
                             else { section.split("\n").next().unwrap_or("").to_string() };
                
                let first_paragraph = section.split("\n\n").nth(1).unwrap_or("");
                if first_paragraph.is_empty() {
                    heading
                } else {
                    format!("{}\n\n{}", heading, first_paragraph)
                }
            };

            compressed_sections.push(compressed_section);
        }

        // Join sections back together
        let mut compressed_content = compressed_sections[0].clone();
        for section in &compressed_sections[1..] {
            compressed_content.push_str("\n## ");
            compressed_content.push_str(section);
        }

        let compressed_size = compressed_content.len();
        let compression_ratio = if compressed_size > 0 {
            original_size as f64 / compressed_size as f64
        } else {
            1.0
        };

        // Calculate quality score based on information retention
        let quality_score = 0.85; // Variable resolution loses some detail

        // Create compressed segment
        let mut compressed_segment = segment.clone();
        compressed_segment.content = compressed_content;
        compressed_segment.metadata.insert("compressed".to_string(), "true".to_string());
        compressed_segment.metadata.insert("compression_strategy".to_string(), "variable_resolution".to_string());
        compressed_segment.metadata.insert("original_size".to_string(), original_size.to_string());

        let result = CompressionResult {
            original_size,
            compressed_size,
            compression_ratio,
            strategy: CompressionStrategy::VariableResolution,
            quality_score,
        };

        Ok((compressed_segment, result))
    }

    /// Apply temporal compression strategy for old episodic memories
    fn apply_temporal_compression(&self, segment: &MemorySegment) -> Result<(MemorySegment, CompressionResult), Box<dyn Error>> {
        let original_content = &segment.content;
        let original_size = original_content.len();

        // Age-based compression factor
        let age_days = segment.days_since_creation();
        let compression_factor = if age_days > 365 {
            0.2 // Very old memory - aggressive compression
        } else if age_days > 180 {
            0.4 // Old memory - significant compression
        } else if age_days > 90 {
            0.6 // Medium-old memory - moderate compression
        } else {
            0.8 // Relatively recent memory - light compression
        };

        // Split content into paragraphs
        let paragraphs: Vec<&str> = original_content.split("\n\n").collect();
        
        // For very old memories, keep only headings and key points
        let compressed_paragraphs: Vec<String> = if compression_factor <= 0.3 {
            paragraphs.iter()
                .filter_map(|p| {
                    if p.starts_with("#") || p.contains("KEY:") || p.contains("IMPORTANT:") {
                        Some(p.to_string())
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            // Select paragraphs to keep based on compression factor
            let paragraphs_to_keep = (paragraphs.len() as f64 * compression_factor).ceil() as usize;
            
            // Prioritize headings, first/last paragraphs, and paragraphs with keywords
            let mut paragraph_scores: Vec<(usize, f64)> = paragraphs.iter().enumerate()
                .map(|(i, p)| {
                    let mut score = 0.0;
                    
                    // Headings get high scores
                    if p.starts_with("#") {
                        score += 10.0;
                    }
                    
                    // First and last paragraphs are important
                    if i == 0 || i == paragraphs.len() - 1 {
                        score += 5.0;
                    }
                    
                    // Paragraphs with key indicators
                    if p.contains("KEY:") || p.contains("IMPORTANT:") || p.contains("NOTE:") {
                        score += 8.0;
                    }
                    
                    // Score based on information density (simple heuristic)
                    let density = p.len() as f64 / (p.split_whitespace().count() as f64 + 1.0);
                    score += density;
                    
                    (i, score)
                })
                .collect();
            
            // Sort by score descending
            paragraph_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            
            // Take top paragraphs
            let indices_to_keep: HashSet<usize> = paragraph_scores.iter()
                .take(paragraphs_to_keep)
                .map(|(i, _)| *i)
                .collect();
            
            // Rebuild content in original order
            paragraphs.iter()
                .enumerate()
                .filter_map(|(i, p)| {
                    if indices_to_keep.contains(&i) {
                        Some(p.to_string())
                    } else {
                        None
                    }
                })
                .collect()
        };

        // Rejoin content
        let compressed_content = compressed_paragraphs.join("\n\n");
        
        // Add a temporal compression notice
        let compressed_with_notice = format!(
            "{}\n\n---\n*Note: This memory has been temporally compressed. Age: {} days. Compression ratio: {:.2}x*",
            compressed_content,
            age_days,
            original_size as f64 / compressed_content.len() as f64
        );

        let compressed_size = compressed_with_notice.len();
        let compression_ratio = if compressed_size > 0 {
            original_size as f64 / compressed_size as f64
        } else {
            1.0
        };

        // Calculate quality score
        let quality_score = if age_days > 365 {
            0.6 // Older memories have lower information quality preservation
        } else if age_days > 180 {
            0.7
        } else if age_days > 90 {
            0.8
        } else {
            0.9
        };

        // Create compressed segment
        let mut compressed_segment = segment.clone();
        compressed_segment.content = compressed_with_notice;
        compressed_segment.metadata.insert("compressed".to_string(), "true".to_string());
        compressed_segment.metadata.insert("compression_strategy".to_string(), "temporal_compression".to_string());
        compressed_segment.metadata.insert("original_size".to_string(), original_size.to_string());
        compressed_segment.metadata.insert("age_days".to_string(), age_days.to_string());

        let result = CompressionResult {
            original_size,
            compressed_size,
            compression_ratio,
            strategy: CompressionStrategy::TemporalCompression,
            quality_score,
        };

        Ok((compressed_segment, result))
    }

    /// Apply procedural summarization for procedural memory
    fn apply_procedural_summarization(&self, segment: &MemorySegment) -> Result<(MemorySegment, CompressionResult), Box<dyn Error>> {
        let original_content = &segment.content;
        let original_size = original_content.len();

        // Parse procedural steps
        let lines: Vec<&str> = original_content.lines().collect();
        let mut compressed_lines = Vec::new();
        let mut in_code_block = false;
        let mut in_list = false;
        let mut current_list_items = Vec::new();

        // Process line by line
        for line in lines {
            // Preserve code blocks entirely
            if line.starts_with("```") {
                in_code_block = !in_code_block;
                compressed_lines.push(line.to_string());
                continue;
            }

            if in_code_block {
                compressed_lines.push(line.to_string());
                continue;
            }

            // Handle list items (numbered or bullet points)
            if line.trim().starts_with("- ") || line.trim().starts_with("* ") || 
               line.trim().matches(char::is_numeric).count() > 0 && line.contains(". ") {
                
                if !in_list {
                    in_list = true;
                    current_list_items.clear();
                }
                
                current_list_items.push(line.to_string());
                continue;
            } else if in_list && line.trim().is_empty() {
                // End of list
                in_list = false;
                
                // If list is long, summarize it
                if current_list_items.len() > 5 {
                    compressed_lines.push(format!("{}... (and {} more steps)", 
                        current_list_items[0], current_list_items.len() - 1));
                    compressed_lines.push(format!("{}", current_list_items.last().unwrap_or(&String::new())));
                } else {
                    // Short list - keep all items
                    compressed_lines.extend(current_list_items.clone());
                }
                
                compressed_lines.push(String::new()); // Add empty line
                continue;
            }

            // Handle headings and key content
            if line.starts_with("#") || line.contains("IMPORTANT:") || line.contains("KEY:") {
                compressed_lines.push(line.to_string());
            } 
            // Summarize paragraphs
            else if !line.trim().is_empty() {
                let words: Vec<&str> = line.split_whitespace().collect();
                if words.len() > 20 {
                    // Take first 10 words as summary
                    let summary = words.iter().take(10).cloned().collect::<Vec<&str>>().join(" ");
                    compressed_lines.push(format!("{}...", summary));
                } else {
                    compressed_lines.push(line.to_string());
                }
            } else {
                compressed_lines.push(line.to_string());
            }
        }

        // Handle any remaining list
        if in_list {
            if current_list_items.len() > 5 {
                compressed_lines.push(format!("{}... (and {} more steps)", 
                    current_list_items[0], current_list_items.len() - 1));
                compressed_lines.push(format!("{}", current_list_items.last().unwrap_or(&String::new())));
            } else {
                compressed_lines.extend(current_list_items);
            }
        }

        // Join lines back
        let compressed_content = compressed_lines.join("\n");
        
        // Add recovery notice
        let compressed_with_notice = format!(
            "{}\n\n---\n*Note: This procedural memory has been summarized. Original steps: {}. See full version if details needed.*",
            compressed_content,
            lines.len()
        );

        let compressed_size = compressed_with_notice.len();
        let compression_ratio = if compressed_size > 0 {
            original_size as f64 / compressed_size as f64
        } else {
            1.0
        };

        // For procedural memory, balance between compression and utility
        let quality_score = 0.75;

        // Create compressed segment
        let mut compressed_segment = segment.clone();
        compressed_segment.content = compressed_with_notice;
        compressed_segment.metadata.insert("compressed".to_string(), "true".to_string());
        compressed_segment.metadata.insert("compression_strategy".to_string(), "procedural_summarization".to_string());
        compressed_segment.metadata.insert("original_size".to_string(), original_size.to_string());
        compressed_segment.metadata.insert("original_steps".to_string(), lines.len().to_string());

        let result = CompressionResult {
            original_size,
            compressed_size,
            compression_ratio,
            strategy: CompressionStrategy::ProceduralSummarization,
            quality_score,
        };

        Ok((compressed_segment, result))
    }

    /// Apply identity compression - keeps only essential identity elements
    fn apply_identity_compression(&self, segment: &MemorySegment) -> Result<(MemorySegment, CompressionResult), Box<dyn Error>> {
        let original_content = &segment.content;
        let original_size = original_content.len();

        // For identity, we are very careful - only compress if explicitly allowed
        // and only with most conservative approach
        
        // Extract key identity elements
        let lines: Vec<&str> = original_content.lines().collect();
        let mut compressed_lines = Vec::new();
        
        // Always include headings and purpose statements
        for line in lines {
            if line.starts_with("#") || 
               line.contains("PURPOSE:") ||
               line.contains("MISSION:") ||
               line.contains("ROLE:") ||
               line.contains("IDENTITY:") {
                compressed_lines.push(line.to_string());
            }
        }
        
        // If we didn't find enough identity information, include more
        if compressed_lines.len() < 5 {
            // Include the first paragraph
            let first_para = original_content.split("\n\n").next().unwrap_or("");
            if !first_para.is_empty() && !compressed_lines.contains(&first_para.to_string()) {
                compressed_lines.insert(1, first_para.to_string());
            }
            
            // Look for key capability descriptions
            for line in lines {
                if line.contains("capable of") || 
                   line.contains("specialized in") || 
                   line.contains("responsible for") {
                    compressed_lines.push(line.to_string());
                }
            }
        }
        
        // Rejoin content
        let compressed_content = compressed_lines.join("\n");
        
        // Add recovery notice that indicates this is a partial identity
        let compressed_with_notice = format!(
            "{}\n\n---\n*Note: This is a compressed identity summary. Full identity data is available in the complete memory file.*",
            compressed_content
        );

        let compressed_size = compressed_with_notice.len();
        let compression_ratio = if compressed_size > 0 {
            original_size as f64 / compressed_size as f64
        } else {
            1.0
        };

        // Identity compression must retain essential character
        let quality_score = 0.8;

        // Create compressed segment
        let mut compressed_segment = segment.clone();
        compressed_segment.content = compressed_with_notice;
        compressed_segment.metadata.insert("compressed".to_string(), "true".to_string());
        compressed_segment.metadata.insert("compression_strategy".to_string(), "identity_compression".to_string());
        compressed_segment.metadata.insert("original_size".to_string(), original_size.to_string());
        compressed_segment.metadata.insert("partial_identity".to_string(), "true".to_string());

        let result = CompressionResult {
            original_size,
            compressed_size,
            compression_ratio,
            strategy: CompressionStrategy::IdentityCompression,
            quality_score,
        };

        Ok((compressed_segment, result))
    }

    /// Run analysis on overall memory compression performance
    pub fn analyze_compression_results(&self) -> HashMap<String, f64> {
        let mut analytics = HashMap::new();
        
        // Overall metrics
        let total_original: usize = self.compression_results.iter()
            .map(|r| r.original_size)
            .sum();
            
        let total_compressed: usize = self.compression_results.iter()
            .map(|r| r.compressed_size)
            .sum();
            
        let overall_ratio = if total_compressed > 0 {
            total_original as f64 / total_compressed as f64
        } else {
            1.0
        };
        
        let avg_quality: f64 = if self.compression_results.len() > 0 {
            self.compression_results.iter()
                .map(|r| r.quality_score)
                .sum::<f64>() / self.compression_results.len() as f64
        } else {
            0.0
        };
        
        // Add metrics to result
        analytics.insert("total_original_bytes".to_string(), total_original as f64);
        analytics.insert("total_compressed_bytes".to_string(), total_compressed as f64);
        analytics.insert("overall_compression_ratio".to_string(), overall_ratio);
        analytics.insert("average_quality_score".to_string(), avg_quality);
        analytics.insert("segments_compressed".to_string(), self.compression_results.len() as f64);
        
        // Strategy-specific metrics
        let mut strategy_counts = HashMap::new();
        let mut strategy_ratios = HashMap::new();
        
        for result in self.compression_results.iter() {
            let strategy_name = format!("{:?}", result.strategy);
            
            // Update count
            *strategy_counts.entry(strategy_name.clone()).or_insert(0.0) += 1.0;
            
            // Update average ratio
            let entry = strategy_ratios.entry(strategy_name).or_insert((0.0, 0.0));
            entry.0 += result.compression_ratio;
            entry.1 += 1.0;
        }
        
        // Calculate averages for each strategy
        for (strategy, (total_ratio, count)) in strategy_ratios {
            let avg_ratio = if count > 0.0 { total_ratio / count } else { 0.0 };
            analytics.insert(format!("avg_ratio_{}", strategy), avg_ratio);
            analytics.insert(format!("count_{}", strategy), count);
        }
        
        analytics
    }
}

/// Semantic similarity calculator for comparing text
struct SemanticSimilarityCalculator {
    /// Reference embeddings for similarity comparisons
    reference_embeddings: HashMap<String, Vec<f32>>,
}

impl SemanticSimilarityCalculator {
    /// Create a new semantic similarity calculator
    fn new(reference_embeddings: HashMap<String, Vec<f32>>) -> Self {
        Self {
            reference_embeddings,
        }
    }
    
    /// Calculate a semantic hash for text deduplication
    fn calculate_semantic_hash(&self, text: &str) -> String {
        // Simple implementation using a hash of key words
        // In a real implementation, this would use embeddings
        
        // Normalize and tokenize the text
        let normalized = text.to_lowercase();
        let words: Vec<&str> = normalized
            .split_whitespace()
            .filter(|w| w.len() > 3) // Filter out short words
            .collect();
            
        // Sort words to ensure order invariance
        let mut sorted_words = words.clone();
        sorted_words.sort();
        
        // Take top N most significant words
        let significant_words: Vec<&str> = sorted_words.iter()
            .filter(|w| !Self::is_stopword(w))
            .take(10)
            .cloned()
            .collect();
            
        // Create a hash of these words
        let semantic_key = significant_words.join(" ");
        let mut hasher = Sha256::new();
        hasher.update(semantic_key.as_bytes());
        let hash = hasher.finalize();
        
        // Return first 16 bytes as hex string
        format!("{:x}", hash)[..16].to_string()
    }
    
    /// Check if a word is a common stopword
    fn is_stopword(word: &str) -> bool {
        static STOPWORDS: [&str; 25] = [
            "the", "and", "that", "this", "with", "for", "was", "were", "from", 
            "have", "has", "had", "not", "but", "what", "when", "where", "who",
            "which", "how", "why", "can", "will", "should", "would"
        ];
        
        STOPWORDS.contains(&word)
    }
    
    /// Calculate cosine similarity between two vectors
    #[allow(dead_code)]
    fn calculate_cosine_similarity(&self, vec1: &[f32], vec2: &[f32]) -> f32 {
        if vec1.len() != vec2.len() || vec1.is_empty() {
            return 0.0;
        }
        
        let dot_product: f32 = vec1.iter().zip(vec2.iter())
            .map(|(a, b)| a * b)
            .sum();
            
        let magnitude1: f32 = vec1.iter()
            .map(|x| x * x)
            .sum::<f32>()
            .sqrt();
            
        let magnitude2: f32 = vec2.iter()
            .map(|x| x * x)
            .sum::<f32>()
            .sqrt();
            
        if magnitude1 == 0.0 || magnitude2 == 0.0 {
            return 0.0;
        }
        
        dot_product / (magnitude1 * magnitude2)
    }
}

/// Extension trait for MemorySegment with utility methods
trait MemorySegmentExt {
    /// Get the number of days since the segment was created
    fn days_since_creation(&self) -> u32;
    
    /// Get the number of days since the segment was last accessed
    fn days_since_last_access(&self) -> u32;
}

impl MemorySegmentExt for MemorySegment {
    fn days_since_creation(&self) -> u32 {
        // For simplicity, return a fixed value based on the segment type
        // In a real implementation, this would calculate from actual timestamps
        match self.memory_type {
            MemoryType::Identity => 1,      // Identity segments are treated as recent
            MemoryType::Semantic => 30,     // Semantic knowledge is moderately aged
            MemoryType::Episodic => 90,     // Episodic memories are older
            MemoryType::Procedural => 60,   // Procedural knowledge is moderately aged
            MemoryType::Relational => 45,   // Relational knowledge is moderately aged
        }
    }
    
    fn days_since_last_access(&self) -> u32 {
        // This would calculate from the last_accessed_timestamp
        // For simplicity, we're using a placeholder implementation
        if let Some(_timestamp) = &self.last_accessed_timestamp {
            // Calculate days from timestamp
            // Placeholder value for now
            30
        } else {
            // If never accessed, consider it old
            120
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper function to create a test memory segment
    fn create_test_segment(segment_id: &str, memory_type: MemoryType, content: &str) -> MemorySegment {
        MemorySegment {
            id: segment_id.to_string(),
            memory_type,
            tier: 1,
            content: content.to_string(),
            metadata: HashMap::new(),
            activation_score: 0.2,
            last_accessed_timestamp: None,
            related_segments: Vec::new(),
        }
    }
    
    #[test]
    fn test_semantic_deduplication() {
        // Create a test memory compressor
        let config = CompressionConfig::default();
        let memory_loader = Arc::new(Mutex::new(MemoryLoader::default()));
        let compressor = MemoryCompressor::new(config, memory_loader);
        
        // Create a test segment with duplicate semantic content
        let content = "
# Knowledge about databases

Relational databases store data in tables with rows and columns.
Each row represents a record, and each column represents a field.

Relational databases store data in tables with rows and columns.
The data is organized into tables that can be linked together.

NoSQL databases are designed for specific data models and have flexible schemas.
They are optimized for specific data access patterns.

NoSQL databases are designed for specific data models with flexible schemas.
These databases are built for performance and scalability.
";
        
        let segment = create_test_segment("test-semantic", MemoryType::Semantic, content);
        
        // Compress the segment
        let (compressed, result) = compressor.apply_semantic_deduplication(&segment).unwrap();
        
        // Verify compression results
        assert!(result.compression_ratio > 1.0, "Should achieve some compression");
        assert!(compressed.content.len() < content.len(), "Compressed content should be smaller");
        assert!(compressed.metadata.get("compressed").unwrap() == "true", "Should be marked as compressed");
    }
    
    #[test]
    fn test_temporal_compression() {
        // Create a test memory compressor
        let config = CompressionConfig::default();
        let memory_loader = Arc::new(Mutex::new(MemoryLoader::default()));
        let compressor = MemoryCompressor::new(config, memory_loader);
        
        // Create a test segment with episodic memory
        let content = "
# Project Meeting: Database Schema Design

## Attendees
- Sarah Johnson
- Michael Chen
- Priya Patel
- Juan Rodriguez

## Topics Discussed
1. We reviewed the current schema design for the user authentication system.
2. Michael suggested adding a separate table for user roles.
3. Priya raised concerns about performance with the current index strategy.
4. Juan presented benchmarks showing the query performance.
5. We decided to implement the role-based access control as suggested.
6. Action items were assigned to team members.

## Action Items
- Sarah: Update the schema documentation
- Michael: Implement the new roles table
- Priya: Optimize the existing indexes
- Juan: Prepare a follow-up benchmark

## Next Steps
We will reconvene next week to review the implementation progress.
";
        
        let segment = create_test_segment("test-episodic", MemoryType::Episodic, content);
        
        // Compress the segment
        let (compressed, result) = compressor.apply_temporal_compression(&segment).unwrap();
        
        // Verify compression results
        assert!(result.compression_ratio > 1.0, "Should achieve some compression");
        assert!(compressed.content.len() < content.len(), "Compressed content should be smaller");
        assert!(compressed.metadata.get("compressed").unwrap() == "true", "Should be marked as compressed");
        assert!(compressed.content.contains("Note: This memory has been temporally compressed"), 
                "Should contain compression notice");
    }
}