//! Enhanced Agent Creation System
//! 
//! This module provides the new template-based agent creation functionality
//! that integrates with the existing CIA CLI agent management system.

pub mod template_engine;
pub mod creation;

// Re-export main functions for easy access
pub use creation::{create_agent_interactive, validate_agent};
pub use template_engine::{AgentTemplate, TemplateProcessor};