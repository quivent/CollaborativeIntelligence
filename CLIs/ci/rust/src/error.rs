//! Error handling for CI CLI
//!
//! This module defines the error types used throughout the CI CLI.

use thiserror::Error;

/// CI CLI errors
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration-related errors
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    /// Integration-related errors
    #[error("Integration error: {0}")]
    IntegrationError(String),
    
    /// Project-related errors
    #[error("Project error: {0}")]
    ProjectError(String),
    
    /// Agent-related errors
    #[error("Agent error: {0}")]
    AgentError(String),
    
    /// Memory-related errors
    #[error("Memory error: {0}")]
    MemoryError(String),
    
    /// System-related errors
    #[error("System error: {0}")]
    SystemError(String),
    
    /// IO errors
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// Result type for CI CLI operations
pub type Result<T> = std::result::Result<T, Error>;