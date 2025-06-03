use thiserror::Error;
use std::io;

/// Custom error types for the Agent Cache System
#[derive(Error, Debug)]
pub enum AgentCacheError {
    /// I/O errors (file operations, etc.)
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    
    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    /// Errors parsing or creating cache entries
    #[error("Cache entry error: {0}")]
    EntryError(String),
    
    /// Errors related to file system paths
    #[error("Path error: {0}")]
    PathError(String),
    
    /// Agent not found
    #[error("Agent not found: {0}")]
    AgentNotFound(String),
    
    /// General errors
    #[error("Error: {0}")]
    GeneralError(String),
    
    /// System time error
    #[error("System time error: {0}")]
    TimeError(#[from] std::time::SystemTimeError),
    
    /// Security error
    #[error("Security error: {0}")]
    SecurityError(#[from] crate::security::SecurityError),
}

/// Result type for Agent Cache operations
pub type Result<T> = std::result::Result<T, AgentCacheError>;