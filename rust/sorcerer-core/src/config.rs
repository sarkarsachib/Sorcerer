//! Configuration loading and management

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Main Sorcerer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SorcererConfig {
    /// System-level configuration
    pub system: SystemConfig,
    /// Database configuration
    pub database: DatabaseConfig,
    /// Indexing configuration
    pub indexing: IndexingConfig,
    /// Crawler configuration
    pub crawler: CrawlerConfig,
    /// Agent configuration
    pub agents: AgentsConfig,
    /// API configuration
    pub api: ApiConfig,
}

impl SorcererConfig {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: SorcererConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Load configuration with defaults
    pub fn load_default() -> Self {
        Self {
            system: SystemConfig::default(),
            database: DatabaseConfig::default(),
            indexing: IndexingConfig::default(),
            crawler: CrawlerConfig::default(),
            agents: AgentsConfig::default(),
            api: ApiConfig::default(),
        }
    }
}

/// System-level configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// System name
    pub name: String,
    /// System version
    pub version: String,
    /// Logging level
    pub log_level: String,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            name: "Sorcerer".to_string(),
            version: "0.1.0".to_string(),
            log_level: "info".to_string(),
        }
    }
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database host
    pub host: String,
    /// Database port
    pub port: u16,
    /// Database name
    pub name: String,
    /// Connection pool size
    pub pool_size: u32,
    /// Connection timeout in seconds
    pub timeout_secs: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            name: "sorcerer_index".to_string(),
            pool_size: 20,
            timeout_secs: 30,
        }
    }
}

/// Indexing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingConfig {
    /// Vector dimensions for embeddings
    pub vector_dimensions: usize,
    /// Batch size for bulk operations
    pub batch_size: usize,
    /// Auto-commit interval in seconds
    pub auto_commit_interval_secs: u32,
}

impl Default for IndexingConfig {
    fn default() -> Self {
        Self {
            vector_dimensions: 1536,
            batch_size: 100,
            auto_commit_interval_secs: 300,
        }
    }
}

/// Crawler configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlerConfig {
    /// User agent string
    pub user_agent: String,
    /// Maximum concurrent crawls
    pub max_concurrent_crawls: usize,
    /// Timeout in seconds
    pub timeout_secs: u32,
}

impl Default for CrawlerConfig {
    fn default() -> Self {
        Self {
            user_agent: "Sorcerer/0.1.0 (+http://sorcerer.ai)".to_string(),
            max_concurrent_crawls: 10,
            timeout_secs: 30,
        }
    }
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsConfig {
    /// Maximum sub-agents per task
    pub max_sub_agents: usize,
    /// Memory TTL in hours
    pub memory_ttl_hours: u32,
    /// Default timeout in seconds
    pub default_timeout_secs: u32,
}

impl Default for AgentsConfig {
    fn default() -> Self {
        Self {
            max_sub_agents: 5,
            memory_ttl_hours: 24,
            default_timeout_secs: 300,
        }
    }
}

/// API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// API host
    pub host: String,
    /// HTTP API port
    pub port: u16,
    /// gRPC API port
    pub grpc_port: u16,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            grpc_port: 50051,
        }
    }
}
