// Main library module for the memory system

pub mod memory_segment_parser;
pub mod memory_activation_model;
pub mod memory_loader;
pub mod memory_updater;

pub use memory_segment_parser::{
    MemorySegment, 
    MemoryType, 
    MemoryTier, 
    MemorySegmentParser,
};

pub use memory_activation_model::{
    MemoryActivationModel,
    SegmentActivation,
};

pub use memory_loader::{
    MemoryLoader,
    MemoryLoaderConfig,
    LoadStatistics,
    LoaderState,
};

pub use memory_updater::{
    MemoryUpdater,
    MemoryUpdaterConfig,
    UpdateStatistics,
    MemoryUpdate,
    UpdateType,
    UpdateBatch,
};

// Error type for memory system operations
#[derive(thiserror::Error, Debug)]
pub enum MemorySystemError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Segment not found: {0}")]
    SegmentNotFound(String),
    
    #[error("Agent not found: {0}")]
    AgentNotFound(String),
    
    #[error("Invalid segment format: {0}")]
    InvalidSegmentFormat(String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Update error: {0}")]
    UpdateError(String),
    
    #[error("Other error: {0}")]
    Other(String),
}

impl From<String> for MemorySystemError {
    fn from(error: String) -> Self {
        MemorySystemError::Other(error)
    }
}

impl From<&str> for MemorySystemError {
    fn from(error: &str) -> Self {
        MemorySystemError::Other(error.to_string())
    }
}

// Public API for the memory system
pub struct MemorySystem {
    loader: MemoryLoader,
    updater: MemoryUpdater,
}

impl MemorySystem {
    // Create a new memory system
    pub fn new() -> Result<Self, MemorySystemError> {
        // Use default configurations
        let loader_config = MemoryLoaderConfig::default();
        let mut loader = MemoryLoader::new(loader_config).map_err(|e| MemorySystemError::Other(e.to_string()))?;
        
        // Share activation model between loader and updater
        let activation_model = loader.get_activation_model();
        
        let updater_config = MemoryUpdaterConfig::default();
        let updater = MemoryUpdater::new(updater_config, activation_model).map_err(|e| MemorySystemError::Other(e.to_string()))?;
        
        Ok(MemorySystem {
            loader,
            updater,
        })
    }
    
    // Create a new memory system with custom configuration
    pub fn with_config(loader_config: MemoryLoaderConfig, updater_config: MemoryUpdaterConfig) -> Result<Self, MemorySystemError> {
        let mut loader = MemoryLoader::new(loader_config).map_err(|e| MemorySystemError::Other(e.to_string()))?;
        
        // Share activation model between loader and updater
        let activation_model = loader.get_activation_model();
        
        let updater = MemoryUpdater::new(updater_config, activation_model).map_err(|e| MemorySystemError::Other(e.to_string()))?;
        
        Ok(MemorySystem {
            loader,
            updater,
        })
    }
    
    // Load agent memory with context
    pub fn load_agent_memory(&mut self, agent_name: &str, context: Option<Vec<String>>) -> Result<Vec<MemorySegment>, MemorySystemError> {
        self.loader.load_agent_memory(agent_name, context).map_err(|e| MemorySystemError::Other(e.to_string()))
    }
    
    // Create and queue a memory update
    pub fn update_memory(&mut self, agent_name: &str, update_type: UpdateType, segment: Option<MemorySegment>, importance: f64) -> Result<(), MemorySystemError> {
        let update = self.updater.create_update(
            update_type,
            segment,
            importance,
            "api",
            None
        );
        
        self.updater.queue_update(agent_name, update).map_err(|e| MemorySystemError::Other(e.to_string()))
    }
    
    // Process all pending updates for an agent
    pub fn process_updates(&mut self, agent_name: &str) -> Result<bool, MemorySystemError> {
        self.updater.process_pending_updates(agent_name).map_err(|e| MemorySystemError::Other(e.to_string()))
    }
    
    // Get loader statistics
    pub fn get_loader_statistics(&self) -> &LoadStatistics {
        &self.loader.get_loader_state().load_statistics
    }
    
    // Get updater statistics
    pub fn get_updater_statistics(&self) -> &UpdateStatistics {
        self.updater.get_statistics()
    }
    
    // Reset all statistics
    pub fn reset_statistics(&mut self) {
        self.loader.reset_statistics();
        self.updater.reset_statistics();
    }
}

// Initialize logging
pub fn init_logging() {
    env_logger::init();
}