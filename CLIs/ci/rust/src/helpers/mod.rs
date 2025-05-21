//! Helper utilities for CI CLI
//!
//! This module provides common helper functions and utilities used throughout the CI CLI.

pub mod terminal;
pub mod path;
pub mod execution;
pub mod external;

// Re-export commonly used helpers
pub use terminal::TerminalHelpers;
pub use path::PathHelpers;
pub use execution::ExecutionHelpers;
pub use external::{
    is_external_ci_available, 
    detect_external_ci, 
    execute_external_ci,
    show_deprecation_notice
};