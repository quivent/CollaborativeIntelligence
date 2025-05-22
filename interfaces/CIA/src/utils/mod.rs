//! Utility functions for CIA CLI
//!
//! This module provides utility functions that are not part of the main CLI structure
//! but assist with various operations.

mod terminal;
mod path;

// Re-export only what's actually used
// Note: This is commented out because these modules aren't currently
// used but might be needed in the future
// pub use terminal::*;
// pub use path::*;