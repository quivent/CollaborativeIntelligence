pub mod connection;
pub mod error;
pub mod internal;
pub mod projects;
pub mod api;
pub mod ci_integration;
pub mod optimization;
pub mod advanced_features;

pub use connection::Database;
pub use error::{Result, Error};
pub use api::CIDatabaseAPI;
pub use ci_integration::CIDatabaseIntegration;

// Re-export commonly used types
pub use internal::models::{Agent, LearningEntry, Session, KnowledgeEntry};
pub use projects::models::{Project, Operation, Metric};