// Collaborative Intelligence Core Library
// Provides core functionality for the CI system

pub mod matrix_quadrants;

// Re-export commonly used types
pub use matrix_quadrants::{
    QuadrantMatrix, 
    MatrixItem, 
    QuadrantPosition, 
    QuadrantType,
    MatrixTemplates
};