use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

// Import from memory_segment_parser.rs
use crate::memory_segment_parser::{MemorySegment, MemoryType, MemoryTier};

// Activation threshold for spreading activation
const BASE_ACTIVATION_THRESHOLD: f64 = 0.2;
const DECAY_RATE: f64 = 0.1;
const RECENCY_WEIGHT: f64 = 0.5;
const FREQUENCY_WEIGHT: f64 = 0.3;
const RELEVANCE_WEIGHT: f64 = 1.0;

// Storage for activation values of segments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentActivation {
    pub segment_id: String,
    pub activation_value: f64,
    pub last_activation: SystemTime,
    pub activation_history: Vec<(SystemTime, f64)>,
    pub related_segments: HashMap<String, f64>,
    pub context_relevance: HashMap<String, f64>,
}

// Memory activation model based on spreading activation in neuroscience
pub struct MemoryActivationModel {
    // All segments keyed by ID
    segments: HashMap<String, Arc<MemorySegment>>,
    
    // Segment activations
    activations: HashMap<String, SegmentActivation>,
    
    // Active context keywords
    active_context: Vec<String>,
    
    // Domain-based segment indices
    domain_index: HashMap<String, Vec<String>>,
    
    // Type-based segment indices
    type_index: HashMap<MemoryType, Vec<String>>,
    
    // Tier-based segment indices
    tier_index: HashMap<MemoryTier, Vec<String>>,
    
    // Statistics for optimization
    stats: ActivationStatistics,
}

// Statistics for memory access optimization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivationStatistics {
    pub total_activations: u64,
    pub context_switches: u64,
    pub activation_by_type: HashMap<MemoryType, u64>,
    pub activation_by_tier: HashMap<MemoryTier, u64>,
    pub activation_by_domain: HashMap<String, u64>,
    pub segment_access_frequency: HashMap<String, u64>,
    pub context_transitions: HashMap<String, HashMap<String, u64>>,
}

impl MemoryActivationModel {
    // Create a new activation model
    pub fn new() -> Self {
        MemoryActivationModel {
            segments: HashMap::new(),
            activations: HashMap::new(),
            active_context: Vec::new(),
            domain_index: HashMap::new(),
            type_index: HashMap::new(),
            tier_index: HashMap::new(),
            stats: ActivationStatistics::default(),
        }
    }
    
    // Register a segment with the activation model
    pub fn register_segment(&mut self, segment: MemorySegment) {
        let segment_id = segment.id.clone();
        
        // Create initial activation entry
        if !self.activations.contains_key(&segment_id) {
            self.activations.insert(segment_id.clone(), SegmentActivation {
                segment_id: segment_id.clone(),
                activation_value: 0.0,
                last_activation: SystemTime::now(),
                activation_history: Vec::new(),
                related_segments: HashMap::new(),
                context_relevance: HashMap::new(),
            });
        }
        
        // Index segment by domain
        for domain in &segment.domains {
            self.domain_index
                .entry(domain.clone())
                .or_insert_with(Vec::new)
                .push(segment_id.clone());
        }
        
        // Index segment by type
        self.type_index
            .entry(segment.memory_type)
            .or_insert_with(Vec::new)
            .push(segment_id.clone());
        
        // Index segment by tier
        self.tier_index
            .entry(segment.tier)
            .or_insert_with(Vec::new)
            .push(segment_id.clone());
        
        // Store the segment
        let arc_segment = Arc::new(segment);
        self.segments.insert(segment_id, arc_segment);
    }
    
    // Set active context for relevance calculation
    pub fn set_context(&mut self, keywords: Vec<String>) {
        if keywords != self.active_context {
            self.stats.context_switches += 1;
            
            // Record transition
            if !self.active_context.is_empty() {
                let old_ctx = self.active_context.join(",");
                let new_ctx = keywords.join(",");
                
                self.stats.context_transitions
                    .entry(old_ctx)
                    .or_insert_with(HashMap::new)
                    .entry(new_ctx)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            
            self.active_context = keywords;
            
            // Update context relevance for all segments
            self.update_context_relevance();
        }
    }
    
    // Update context relevance for all segments
    fn update_context_relevance(&mut self) {
        if self.active_context.is_empty() {
            return;
        }
        
        for segment_id in self.segments.keys() {
            let segment = self.segments.get(segment_id).unwrap();
            let activation = self.activations.get_mut(segment_id).unwrap();
            
            // Clear previous context relevance
            activation.context_relevance.clear();
            
            // Calculate relevance to each context keyword
            for keyword in &self.active_context {
                let relevance = self.calculate_keyword_relevance(segment, keyword);
                if relevance > 0.0 {
                    activation.context_relevance.insert(keyword.clone(), relevance);
                }
            }
        }
    }
    
    // Calculate relevance of a segment to a keyword
    fn calculate_keyword_relevance(&self, segment: &MemorySegment, keyword: &str) -> f64 {
        let content = segment.content.to_lowercase();
        let keyword_lower = keyword.to_lowercase();
        
        // Basic keyword matching - could be extended with more sophisticated NLP
        let occurrences = content.matches(&keyword_lower).count();
        
        // Domains have higher weight for relevance
        let domain_relevance = if segment.domains.iter().any(|d| d.to_lowercase().contains(&keyword_lower)) {
            2.0
        } else {
            0.0
        };
        
        // Check metadata for relevance
        let metadata_relevance = segment.metadata.iter()
            .filter(|(_, v)| v.to_lowercase().contains(&keyword_lower))
            .count() as f64;
        
        // Combine all relevance sources
        let mut relevance = (occurrences as f64 * 0.1) + domain_relevance + (metadata_relevance * 1.5);
        
        // Adjust by tier (higher tiers are more relevant)
        relevance *= match segment.tier {
            MemoryTier::Tier1 => 1.5,
            MemoryTier::Tier2 => 1.0,
            MemoryTier::Tier3 => 0.5,
        };
        
        // Adjust by type (identity and procedural memory more context-sensitive)
        relevance *= match segment.memory_type {
            MemoryType::Identity => 1.5,
            MemoryType::Procedural => 1.4,
            MemoryType::Relational => 1.3,
            MemoryType::Semantic => 1.0,
            MemoryType::Episodic => 0.8,
        };
        
        relevance
    }
    
    // Activate a segment by ID, triggering spreading activation
    pub fn activate_segment(&mut self, segment_id: &str) -> Result<Arc<MemorySegment>, Box<dyn Error>> {
        if let Some(segment) = self.segments.get(segment_id).cloned() {
            // Update segment access time and count
            if let Some(seg) = self.segments.get_mut(segment_id) {
                // Can't update inside Arc, so we need to create new instance
                let mut updated = (**seg).clone();
                updated.last_accessed = SystemTime::now();
                updated.access_count += 1;
                
                // Replace with updated version
                self.segments.insert(segment_id.to_string(), Arc::new(updated));
            }
            
            // Update activation statistics
            self.stats.total_activations += 1;
            
            if let Some(seg) = self.segments.get(segment_id) {
                // Update type statistics
                self.stats.activation_by_type
                    .entry(seg.memory_type)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                
                // Update tier statistics
                self.stats.activation_by_tier
                    .entry(seg.tier)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                
                // Update domain statistics
                for domain in &seg.domains {
                    self.stats.activation_by_domain
                        .entry(domain.clone())
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
                
                // Update segment frequency
                self.stats.segment_access_frequency
                    .entry(segment_id.to_string())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            
            // Perform spreading activation
            self.spread_activation(segment_id);
            
            Ok(segment)
        } else {
            Err(format!("Segment not found: {}", segment_id).into())
        }
    }
    
    // Spread activation to related segments
    fn spread_activation(&mut self, seed_segment_id: &str) {
        // Queue for breadth-first spreading
        let mut activation_queue = VecDeque::new();
        let mut visited = HashSet::new();
        
        // Start with seed segment at full activation
        activation_queue.push_back((seed_segment_id.to_string(), 1.0));
        visited.insert(seed_segment_id.to_string());
        
        while let Some((current_id, activation_strength)) = activation_queue.pop_front() {
            // Skip if below threshold
            if activation_strength < BASE_ACTIVATION_THRESHOLD {
                continue;
            }
            
            // Get related segments
            let related_segments = self.find_related_segments(&current_id);
            
            // Update activation for current segment
            if let Some(activation) = self.activations.get_mut(&current_id) {
                activation.activation_value = activation_strength;
                activation.last_activation = SystemTime::now();
                activation.activation_history.push((SystemTime::now(), activation_strength));
                
                // Limit history length
                if activation.activation_history.len() > 20 {
                    activation.activation_history.remove(0);
                }
            }
            
            // Spread activation to related segments
            for (related_id, relatedness) in related_segments {
                if visited.contains(&related_id) {
                    continue;
                }
                
                // Decay activation by distance and relatedness
                let spread_activation = activation_strength * relatedness * (1.0 - DECAY_RATE);
                
                if spread_activation >= BASE_ACTIVATION_THRESHOLD {
                    activation_queue.push_back((related_id.clone(), spread_activation));
                    visited.insert(related_id);
                }
                
                // Record relationship strength in both directions
                if let Some(activation) = self.activations.get_mut(&current_id) {
                    activation.related_segments.insert(related_id.clone(), relatedness);
                }
                
                if let Some(activation) = self.activations.get_mut(&related_id) {
                    activation.related_segments.insert(current_id.clone(), relatedness);
                }
            }
        }
    }
    
    // Find related segments and their relatedness value
    fn find_related_segments(&self, segment_id: &str) -> HashMap<String, f64> {
        let mut related = HashMap::new();
        
        if let Some(segment) = self.segments.get(segment_id) {
            // Find segments of the same type
            if let Some(type_segments) = self.type_index.get(&segment.memory_type) {
                for related_id in type_segments {
                    if related_id != segment_id {
                        related.insert(related_id.clone(), 0.4); // Base type relatedness
                    }
                }
            }
            
            // Find segments in the same domains (stronger relationship)
            for domain in &segment.domains {
                if let Some(domain_segments) = self.domain_index.get(domain) {
                    for related_id in domain_segments {
                        if related_id != segment_id {
                            // Increase relatedness for domain match
                            related.entry(related_id.clone())
                                .and_modify(|r| *r += 0.3)
                                .or_insert(0.6); // Base domain relatedness
                        }
                    }
                }
            }
            
            // Text-based similarity between segments
            for (other_id, other_segment) in &self.segments {
                if other_id == segment_id {
                    continue;
                }
                
                // Calculate text similarity (simple implementation)
                // For production, use more sophisticated text similarity algorithms
                let similarity = self.calculate_text_similarity(segment, other_segment);
                if similarity > 0.3 {
                    related.entry(other_id.clone())
                        .and_modify(|r| *r += similarity * 0.5)
                        .or_insert(similarity * 0.5);
                }
            }
        }
        
        related
    }
    
    // Simple text similarity calculation
    fn calculate_text_similarity(&self, segment1: &MemorySegment, segment2: &MemorySegment) -> f64 {
        // Extract words from both segments
        let words1: HashSet<String> = segment1.content
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        let words2: HashSet<String> = segment2.content
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
    
    // Get top N segments based on context relevance and activation
    pub fn get_relevant_segments(&self, context: Option<Vec<String>>, count: usize) -> Vec<Arc<MemorySegment>> {
        let mut segment_scores: Vec<(String, f64)> = Vec::new();
        
        // Use provided context or current active context
        let context_keywords = if let Some(ctx) = context {
            ctx
        } else {
            self.active_context.clone()
        };
        
        // Score all segments
        for (segment_id, segment) in &self.segments {
            let activation = self.activations.get(segment_id);
            if activation.is_none() {
                continue;
            }
            let activation = activation.unwrap();
            
            // Base score from current activation
            let mut score = activation.activation_value;
            
            // Add recency factor
            if let Ok(duration) = SystemTime::now().duration_since(segment.last_accessed) {
                let seconds = duration.as_secs() as f64;
                let recency_factor = (-seconds / 3600.0).exp(); // Exponential decay over hours
                score += recency_factor * RECENCY_WEIGHT;
            }
            
            // Add frequency factor 
            score += (segment.access_count as f64).ln() * FREQUENCY_WEIGHT;
            
            // Add context relevance
            if !context_keywords.is_empty() {
                let total_relevance: f64 = activation.context_relevance.values().sum();
                score += total_relevance * RELEVANCE_WEIGHT;
            }
            
            // Adjust by tier
            score *= match segment.tier {
                MemoryTier::Tier1 => 2.0,  // Highest priority
                MemoryTier::Tier2 => 1.5,
                MemoryTier::Tier3 => 1.0,
            };
            
            segment_scores.push((segment_id.clone(), score));
        }
        
        // Sort by score
        segment_scores.sort_by(|a, b| {
            b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal)
        });
        
        // Return top N segments
        segment_scores.iter()
            .take(count)
            .filter_map(|(id, _)| self.segments.get(id).cloned())
            .collect()
    }
    
    // Generate optimized loading plan based on activation patterns
    pub fn generate_loading_plan(&self) -> HashMap<MemoryTier, Vec<String>> {
        let mut plan: HashMap<MemoryTier, Vec<String>> = HashMap::new();
        
        // Initialize plan with empty vectors for each tier
        plan.insert(MemoryTier::Tier1, Vec::new());
        plan.insert(MemoryTier::Tier2, Vec::new());
        plan.insert(MemoryTier::Tier3, Vec::new());
        
        // First add frequently accessed segments
        let mut segments_by_frequency: Vec<_> = self.stats.segment_access_frequency.iter().collect();
        segments_by_frequency.sort_by(|a, b| b.1.cmp(a.1));
        
        for (segment_id, _) in segments_by_frequency {
            if let Some(segment) = self.segments.get(segment_id) {
                plan.get_mut(&segment.tier)
                    .unwrap()
                    .push(segment_id.clone());
            }
        }
        
        // Then add segments by type priority
        for memory_type in [MemoryType::Identity, MemoryType::Procedural, MemoryType::Relational, MemoryType::Semantic, MemoryType::Episodic].iter() {
            if let Some(segments) = self.type_index.get(memory_type) {
                for segment_id in segments {
                    if let Some(segment) = self.segments.get(segment_id) {
                        let tier_list = plan.get_mut(&segment.tier).unwrap();
                        if !tier_list.contains(segment_id) {
                            tier_list.push(segment_id.clone());
                        }
                    }
                }
            }
        }
        
        plan
    }
    
    // Get statistics
    pub fn get_statistics(&self) -> &ActivationStatistics {
        &self.stats
    }
    
    // Merge activation models
    pub fn merge(&mut self, other: &MemoryActivationModel) {
        // Merge segments
        for (id, segment) in &other.segments {
            if !self.segments.contains_key(id) {
                self.segments.insert(id.clone(), segment.clone());
            }
        }
        
        // Merge activations
        for (id, activation) in &other.activations {
            if !self.activations.contains_key(id) {
                self.activations.insert(id.clone(), activation.clone());
            }
        }
        
        // Merge indices
        for (domain, segments) in &other.domain_index {
            let domain_list = self.domain_index.entry(domain.clone()).or_insert_with(Vec::new);
            for segment_id in segments {
                if !domain_list.contains(segment_id) {
                    domain_list.push(segment_id.clone());
                }
            }
        }
        
        // Merge type index
        for (memory_type, segments) in &other.type_index {
            let type_list = self.type_index.entry(*memory_type).or_insert_with(Vec::new);
            for segment_id in segments {
                if !type_list.contains(segment_id) {
                    type_list.push(segment_id.clone());
                }
            }
        }
        
        // Merge tier index
        for (tier, segments) in &other.tier_index {
            let tier_list = self.tier_index.entry(*tier).or_insert_with(Vec::new);
            for segment_id in segments {
                if !tier_list.contains(segment_id) {
                    tier_list.push(segment_id.clone());
                }
            }
        }
        
        // Merge statistics
        self.stats.total_activations += other.stats.total_activations;
        self.stats.context_switches += other.stats.context_switches;
        
        for (type_key, count) in &other.stats.activation_by_type {
            self.stats.activation_by_type
                .entry(*type_key)
                .and_modify(|c| *c += count)
                .or_insert(*count);
        }
        
        for (tier_key, count) in &other.stats.activation_by_tier {
            self.stats.activation_by_tier
                .entry(*tier_key)
                .and_modify(|c| *c += count)
                .or_insert(*count);
        }
        
        for (domain_key, count) in &other.stats.activation_by_domain {
            self.stats.activation_by_domain
                .entry(domain_key.clone())
                .and_modify(|c| *c += count)
                .or_insert(*count);
        }
        
        for (segment_id, count) in &other.stats.segment_access_frequency {
            self.stats.segment_access_frequency
                .entry(segment_id.clone())
                .and_modify(|c| *c += count)
                .or_insert(*count);
        }
    }
}

// Example usage function
pub fn main() -> Result<(), Box<dyn Error>> {
    // This is just a placeholder for the main function
    // The actual implementation would use the MemoryActivationModel with real data
    println!("Memory activation model ready");
    
    Ok(())
}