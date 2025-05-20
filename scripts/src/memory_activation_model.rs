use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use super::memory_segment_parser::{MemorySegment, MemoryTier, MemoryType};

// Segment activation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentActivation {
    pub segment_id: String,
    pub activation_level: f64,
    pub last_accessed: u64,
    pub access_count: u32,
}

impl SegmentActivation {
    pub fn new(segment_id: &str) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        Self {
            segment_id: segment_id.to_string(),
            activation_level: 0.0,
            last_accessed: now,
            access_count: 0,
        }
    }
    
    // Update activation after access
    pub fn update_activation(&mut self, importance: f64) {
        // Get current time
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Update access count
        self.access_count += 1;
        
        // Calculate time decay (older activations decay)
        let time_since_last_access = now.saturating_sub(self.last_accessed) as f64;
        let decay_factor = (-0.1 * time_since_last_access / 3600.0).exp(); // Exponential decay, half-life ~7 hours
        
        // Calculate new activation level
        let base_activation = 0.5; // Base activation from being accessed
        let count_factor = (self.access_count as f64).sqrt() * 0.2; // Diminishing returns for repeated access
        
        // Compute new activation level
        self.activation_level = (self.activation_level * decay_factor) + base_activation + count_factor + importance;
        
        // Cap activation between 0 and 10
        self.activation_level = self.activation_level.max(0.0).min(10.0);
        
        // Update last accessed time
        self.last_accessed = now;
    }
    
    // Decay activation over time (called periodically)
    pub fn decay_activation(&mut self) {
        // Get current time
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Calculate time since last access
        let time_since_last_access = now.saturating_sub(self.last_accessed) as f64;
        
        // Don't decay if accessed very recently
        if time_since_last_access < 60.0 {
            return;
        }
        
        // Calculate decay factor
        let decay_factor = (-0.1 * time_since_last_access / 3600.0).exp();
        
        // Apply decay
        self.activation_level *= decay_factor;
    }
}

// Memory activation model for tracking segment importance
pub struct MemoryActivationModel {
    // Track activation levels for all segments by agent
    activations: Arc<Mutex<HashMap<String, HashMap<String, SegmentActivation>>>>,
    
    // Memory tier importance multipliers
    tier_multipliers: HashMap<MemoryTier, f64>,
    
    // Memory type importance multipliers
    type_multipliers: HashMap<MemoryType, f64>,
    
    // Cache of most recently activated segments by agent
    recent_segments: Arc<Mutex<HashMap<String, Vec<String>>>>,
    
    // Maximum number of recent segments to track
    max_recent_segments: usize,
}

impl MemoryActivationModel {
    // Create a new activation model
    pub fn new() -> Self {
        let mut model = Self {
            activations: Arc::new(Mutex::new(HashMap::new())),
            tier_multipliers: HashMap::new(),
            type_multipliers: HashMap::new(),
            recent_segments: Arc::new(Mutex::new(HashMap::new())),
            max_recent_segments: 50,
        };
        
        // Set default tier multipliers
        model.tier_multipliers.insert(MemoryTier::Critical, 2.0);
        model.tier_multipliers.insert(MemoryTier::Important, 1.5);
        model.tier_multipliers.insert(MemoryTier::Standard, 1.0);
        model.tier_multipliers.insert(MemoryTier::Peripheral, 0.7);
        model.tier_multipliers.insert(MemoryTier::Archival, 0.4);
        
        // Set default type multipliers
        model.type_multipliers.insert(MemoryType::Core, 2.0);
        model.type_multipliers.insert(MemoryType::Learning, 1.5);
        model.type_multipliers.insert(MemoryType::Context, 1.2);
        model.type_multipliers.insert(MemoryType::Session, 1.0);
        model.type_multipliers.insert(MemoryType::Cache, 0.8);
        model.type_multipliers.insert(MemoryType::Custom, 1.0);
        
        model
    }
    
    // Register a segment access
    pub fn register_access(&self, agent_name: &str, segment: &MemorySegment, importance: Option<f64>) {
        let segment_id = &segment.id;
        
        // Calculate importance multiplier based on tier and type
        let tier_multiplier = self.tier_multipliers.get(&segment.tier).unwrap_or(&1.0);
        let type_multiplier = self.type_multipliers.get(&segment.memory_type).unwrap_or(&1.0);
        
        // Calculate final importance
        let base_importance = importance.unwrap_or(0.5);
        let final_importance = base_importance * tier_multiplier * type_multiplier;
        
        // Update activation
        let mut activations = self.activations.lock().unwrap();
        
        // Get or create agent's activation map
        let agent_activations = activations
            .entry(agent_name.to_string())
            .or_insert_with(HashMap::new);
        
        // Get or create segment activation
        let segment_activation = agent_activations
            .entry(segment_id.clone())
            .or_insert_with(|| SegmentActivation::new(segment_id));
        
        // Update activation
        segment_activation.update_activation(final_importance);
        
        // Update recent segments
        self.update_recent_segments(agent_name, segment_id);
    }
    
    // Update recent segments list
    fn update_recent_segments(&self, agent_name: &str, segment_id: &str) {
        let mut recent = self.recent_segments.lock().unwrap();
        
        // Get or create agent's recent segments list
        let agent_recent = recent
            .entry(agent_name.to_string())
            .or_insert_with(Vec::new);
        
        // Remove segment if it already exists in the list
        if let Some(index) = agent_recent.iter().position(|id| id == segment_id) {
            agent_recent.remove(index);
        }
        
        // Add segment to the beginning of the list
        agent_recent.insert(0, segment_id.to_string());
        
        // Trim list if necessary
        if agent_recent.len() > self.max_recent_segments {
            agent_recent.truncate(self.max_recent_segments);
        }
    }
    
    // Get most active segments for an agent
    pub fn get_most_active_segments(&self, agent_name: &str, count: usize) -> Vec<SegmentActivation> {
        let activations = self.activations.lock().unwrap();
        
        // Get agent's activation map
        if let Some(agent_activations) = activations.get(agent_name) {
            // Convert to vector for sorting
            let mut activations_vec: Vec<SegmentActivation> = agent_activations
                .values()
                .cloned()
                .collect();
            
            // Sort by activation level (descending)
            activations_vec.sort_by(|a, b| {
                b.activation_level.partial_cmp(&a.activation_level).unwrap()
            });
            
            // Return top N
            activations_vec.truncate(count);
            activations_vec
        } else {
            Vec::new()
        }
    }
    
    // Get recently accessed segments for an agent
    pub fn get_recent_segments(&self, agent_name: &str, count: usize) -> Vec<String> {
        let recent = self.recent_segments.lock().unwrap();
        
        // Get agent's recent segments list
        if let Some(agent_recent) = recent.get(agent_name) {
            // Clone and truncate to requested count
            let mut result = agent_recent.clone();
            result.truncate(count);
            result
        } else {
            Vec::new()
        }
    }
    
    // Decay all activations (call periodically)
    pub fn decay_all_activations(&self) {
        let mut activations = self.activations.lock().unwrap();
        
        // Iterate through all agents and segments
        for agent_activations in activations.values_mut() {
            for activation in agent_activations.values_mut() {
                activation.decay_activation();
            }
        }
    }
    
    // Set tier importance multiplier
    pub fn set_tier_multiplier(&mut self, tier: MemoryTier, multiplier: f64) {
        self.tier_multipliers.insert(tier, multiplier);
    }
    
    // Set type importance multiplier
    pub fn set_type_multiplier(&mut self, memory_type: MemoryType, multiplier: f64) {
        self.type_multipliers.insert(memory_type, multiplier);
    }
    
    // Set maximum number of recent segments to track
    pub fn set_max_recent_segments(&mut self, count: usize) {
        self.max_recent_segments = count;
    }
    
    // Get activation level for a specific segment
    pub fn get_segment_activation(&self, agent_name: &str, segment_id: &str) -> Option<f64> {
        let activations = self.activations.lock().unwrap();
        
        activations
            .get(agent_name)
            .and_then(|agent_activations| agent_activations.get(segment_id))
            .map(|activation| activation.activation_level)
    }
    
    // Get all activations for an agent
    pub fn get_agent_activations(&self, agent_name: &str) -> HashMap<String, f64> {
        let activations = self.activations.lock().unwrap();
        
        if let Some(agent_activations) = activations.get(agent_name) {
            agent_activations
                .iter()
                .map(|(id, activation)| (id.clone(), activation.activation_level))
                .collect()
        } else {
            HashMap::new()
        }
    }
    
    // Clear all activations for an agent
    pub fn clear_agent_activations(&self, agent_name: &str) {
        let mut activations = self.activations.lock().unwrap();
        activations.remove(agent_name);
        
        let mut recent = self.recent_segments.lock().unwrap();
        recent.remove(agent_name);
    }
    
    // Get all agent names with activation data
    pub fn get_all_agent_names(&self) -> HashSet<String> {
        let activations = self.activations.lock().unwrap();
        activations.keys().cloned().collect()
    }
}