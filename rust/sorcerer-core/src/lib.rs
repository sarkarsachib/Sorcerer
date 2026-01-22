//! Sorcerer Core - Core abstractions for the agentic search OS
//!
//! This library provides the foundational types and traits for Sorcerer's
//! multi-layered architecture including agents, indices, queries, and results.

pub mod agent;
pub mod config;
pub mod error;
pub mod index;
pub mod query;
pub mod result;

pub use agent::{
    Agent, AgentError, AgentResult, AgentType, ExecutionStatus, Memory, Task,
};

pub use config::{
    AgentsConfig, ApiConfig, CrawlerConfig, DatabaseConfig, IndexingConfig,
    SorcererConfig, SystemConfig,
};

pub use error::{SorcererError, Result};

pub use index::{
    Document, Entity, IndexStore, IndexError, Metadata,
    SearchQuery as IndexSearchQuery, SearchResult as IndexSearchResult,
};

pub use query::{QueryAction, QueryConstraints, QueryMode, SearchQuery};

pub use result::{QueryResult, SearchResult};
