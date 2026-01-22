//! Centralized error handling for Sorcerer

use thiserror::Error;

/// Comprehensive error types for the Sorcerer system
#[derive(Error, Debug)]
pub enum SorcererError {
    /// Configuration-related errors
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Database operation errors
    #[error("Database error: {0}")]
    DatabaseError(String),

    /// Index operation errors
    #[error("Index error: {0}")]
    IndexError(String),

    /// Crawler operation errors
    #[error("Crawler error: {0}")]
    CrawlerError(String),

    /// Agent execution errors
    #[error("Agent error: {0}")]
    AgentError(String),

    /// Query execution errors
    #[error("Query error: {0}")]
    QueryError(String),

    /// I/O operation errors
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON serialization/deserialization errors
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// TOML parsing errors
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),
}

/// Type alias for Result with SorcererError
pub type Result<T> = std::result::Result<T, SorcererError>;
