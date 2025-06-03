// Agent Cache System library
// 
// This library provides functionality for tracking agent activations,
// maintaining an audit trail, and updating terminal titles.

pub mod error;
pub mod cache_manager;
pub mod terminal;
pub mod models;
pub mod security;

pub use cache_manager::AgentCacheManager;
pub use models::{AgentCacheEntry, AgentStatus};
pub use error::AgentCacheError;