//! Rust Data Structures Learning Project
//!
//! This library provides a comprehensive learning resource for Rust's standard
//! data structures, including exercises, examples, and best practices.

pub mod binary_heap;
pub mod btreemap;
pub mod hashmap;
pub mod hashset;
pub mod string;
pub mod vecdeque;
pub mod vector;
pub mod array;
pub mod matrix;

// We don't need to re-export VectorExercises here since it's already
// public through the vector module

#[derive(Debug)]
pub enum ExerciseError {
    InvalidInput(String),
    OperationFailed(String),
}

impl std::fmt::Display for ExerciseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExerciseError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ExerciseError::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
        }
    }
}

impl std::error::Error for ExerciseError {}

/// Result type alias for exercise functions
pub type Result<T> = std::result::Result<T, ExerciseError>;
