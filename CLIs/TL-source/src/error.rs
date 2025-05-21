// TL CLI Error Definition Module

use std::path::PathBuf;
use thiserror::Error;

/// Comprehensive error type for the TL CLI
#[derive(Error, Debug)]
pub enum Error {
    // Configuration errors
    #[error("Failed to load configuration: {0}")]
    ConfigError(String),
    
    #[error("Invalid configuration value: {0}")]
    ConfigValueError(String),
    
    // File system errors
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    // Command execution errors
    #[error("Command failed: {0}")]
    CommandFailed(String),
    
    #[error("Command execution error: {0}")]
    CommandError(String),
    
    // Agent errors
    #[error("Agent not found: {0}")]
    AgentNotFound(String),
    
    #[error("Agent already active: {0}")]
    AgentAlreadyActive(String),
    
    #[error("Invalid agent resources: {0}")]
    InvalidAgentResources(String),
    
    // Memory errors
    #[error("Memory loading error: {0}")]
    MemoryLoadError(String),
    
    #[error("Memory segment not found: {0}")]
    MemorySegmentNotFound(String),
    
    // Knowledge errors
    #[error("Knowledge registry error: {0}")]
    KnowledgeRegistryError(String),
    
    #[error("Knowledge item not found: {0}")]
    KnowledgeItemNotFound(String),
    
    // System errors
    #[error("Repository error: {0}")]
    RepositoryError(String),
    
    #[error("Git operation failed: {0}")]
    GitError(String),
    
    // Parsing errors
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),
    
    // Generic errors
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Operation timed out: {0}")]
    TimeoutError(String),
    
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

/// Result type alias for TL CLI operations
pub type Result<T> = std::result::Result<T, Error>;

/// Extension methods for Result type
pub trait ResultExt<T> {
    /// Convert any error to a TL CLI error with a custom message
    fn with_context<F, S>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> S,
        S: AsRef<str>;
    
    /// Convert any error to a TL CLI error with a custom message and exit the process
    fn context<S: AsRef<str>>(self, msg: S) -> Result<T>;
}

impl<T, E: std::error::Error + 'static> ResultExt<T> for std::result::Result<T, E> {
    fn with_context<F, S>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> S,
        S: AsRef<str>,
    {
        self.map_err(|e| Error::UnexpectedError(format!("{}: {}", f().as_ref(), e)))
    }
    
    fn context<S: AsRef<str>>(self, msg: S) -> Result<T> {
        self.map_err(|e| Error::UnexpectedError(format!("{}: {}", msg.as_ref(), e)))
    }
}

/// Helpers for creating errors
pub mod helpers {
    use super::Error;
    use std::path::Path;
    
    /// Create a file not found error
    pub fn file_not_found<P: AsRef<Path>>(path: P) -> Error {
        Error::FileNotFound(path.as_ref().to_path_buf())
    }
    
    /// Create a permission denied error
    pub fn permission_denied<P: AsRef<Path>>(path: P) -> Error {
        Error::PermissionDenied(path.as_ref().to_path_buf())
    }
    
    /// Create a command failed error
    pub fn command_failed<S: AsRef<str>>(command: S) -> Error {
        Error::CommandFailed(command.as_ref().to_string())
    }
    
    /// Create an agent not found error
    pub fn agent_not_found<S: AsRef<str>>(agent: S) -> Error {
        Error::AgentNotFound(agent.as_ref().to_string())
    }
}

/// Pretty error printing for terminal output
pub mod display {
    use colored::*;
    use std::fmt::Display;
    
    /// Print an error message to stderr with color and formatting
    pub fn print_error<T: Display>(error: T) {
        eprintln!("{} {}", "Error:".red().bold(), error.to_string().bright_red());
    }
    
    /// Print an error message with context
    pub fn print_error_with_context<T: Display, C: Display>(error: T, context: C) {
        eprintln!(
            "{} {}\n{} {}",
            "Error:".red().bold(),
            error.to_string().bright_red(),
            "Context:".yellow().bold(),
            context
        );
    }
    
    /// Print a validation error with specific details
    pub fn print_validation_error<T: Display, D: Display>(error: T, details: D) {
        eprintln!(
            "{} {}\n{} {}",
            "Validation Error:".red().bold(),
            error.to_string().bright_red(),
            "Details:".yellow().bold(),
            details
        );
    }
}