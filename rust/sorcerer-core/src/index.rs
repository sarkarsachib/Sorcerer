//! Index abstractions for multi-modal search storage

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Trait for index storage backends
#[async_trait]
pub trait IndexStore: Send + Sync {
    /// Insert a document into the index
    async fn insert(&mut self, doc: Document) -> Result<String, IndexError>;

    /// Search the index for matching documents
    async fn search(&self, query: SearchQuery) -> Result<Vec<SearchResult>, IndexError>;

    /// Get a document by ID
    async fn get(&self, id: &str) -> Result<Option<Document>, IndexError>;

    /// Delete a document by ID
    async fn delete(&mut self, id: &str) -> Result<bool, IndexError>;
}

/// A document stored in the index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Unique document identifier
    pub id: String,
    /// The document's content
    pub content: String,
    /// Document metadata
    pub metadata: Metadata,
    /// Optional vector embeddings for semantic search
    pub embeddings: Option<Vec<f32>>,
    /// Extracted entities from the document
    pub entities: Vec<Entity>,
}

impl Document {
    /// Create a new document with a generated ID
    pub fn new(content: impl Into<String>, source: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            content: content.into(),
            metadata: Metadata {
                source: source.into(),
                created_at: now,
                updated_at: now,
                trust_score: 0.5,
                tags: Vec::new(),
            },
            embeddings: None,
            entities: Vec::new(),
        }
    }
}

/// Document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// Source of the document (URL, file path, etc.)
    pub source: String,
    /// When the document was created
    pub created_at: DateTime<Utc>,
    /// When the document was last updated
    pub updated_at: DateTime<Utc>,
    /// Trust score (0.0 - 1.0) from ARCANA RANK ENGINE
    pub trust_score: f32,
    /// Tags for categorization
    pub tags: Vec<String>,
}

/// An extracted entity from a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Entity name
    pub name: String,
    /// Type of entity (person, organization, location, etc.)
    pub entity_type: String,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
}

/// Search query for the index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// The search text
    pub text: String,
    /// Maximum number of results
    pub limit: usize,
    /// Minimum trust score
    pub min_trust_score: Option<f32>,
    /// Optional vector for semantic search
    pub vector: Option<Vec<f32>>,
}

impl SearchQuery {
    /// Create a new search query
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            limit: 10,
            min_trust_score: None,
            vector: None,
        }
    }
}

/// A search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Document ID
    pub id: String,
    /// Document title (extracted from content)
    pub title: String,
    /// Document content snippet
    pub content: String,
    /// Source of the document
    pub source: String,
    /// Relevance confidence (0.0 - 1.0)
    pub confidence: f32,
    /// Trust score (0.0 - 1.0)
    pub trust_score: f32,
    /// Additional metadata
    pub metadata: serde_json::Value,
}

/// Errors that can occur during index operations
#[derive(Debug)]
pub enum IndexError {
    /// Storage operation failed
    StorageError(String),
    /// Query execution failed
    QueryError(String),
    /// Document not found
    NotFound,
}

impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IndexError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            IndexError::QueryError(msg) => write!(f, "Query error: {}", msg),
            IndexError::NotFound => write!(f, "Document not found"),
        }
    }
}

impl std::error::Error for IndexError {}
