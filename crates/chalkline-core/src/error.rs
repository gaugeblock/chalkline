//! Error types for `chalkline-core`.

#![forbid(unsafe_code)]

use thiserror::Error;

/// Result alias for chalkline operations.
pub type Result<T> = std::result::Result<T, ChalklineError>;

/// Master error enumeration for `chalkline-core`.
#[derive(Debug, Error)]
pub enum ChalklineError {
    /// I/O failure.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization/deserialization error.
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    /// Errors originating from the storypole engine.
    #[error("Storypole store error: {0}")]
    Storypole(String),

    /// Errors originating from the sawhorse runtime.
    #[error("Sawhorse inference error: {0}")]
    Sawhorse(String),

    /// Errors originating from the miter AST engine.
    #[error("Miter AST engine error: {0}")]
    Miter(String),

    /// Specification parsing error.
    #[error("Spec parsing error: {0}")]
    SpecParse(String),

    /// Contract or topological validation error.
    #[error("Validation error: {0}")]
    Validation(String),
}
