//! Result structures for search operations

use serde::{Deserialize, Serialize};

/// A single search result from VOID SEARCH API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Unique result identifier
    pub id: String,
    /// Result title
    pub title: String,
    /// Result content snippet
    pub content: String,
    /// Source of the result
    pub source: String,
    /// Relevance confidence (0.0 - 1.0)
    pub confidence: f32,
    /// Trust score from ARCANA RANK ENGINE (0.0 - 1.0)
    pub trust_score: f32,
    /// Additional metadata as JSON
    pub metadata: serde_json::Value,
}

/// Complete query result with execution stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    /// Query identifier
    pub query_id: String,
    /// Array of search results
    pub results: Vec<SearchResult>,
    /// Total number of matching results
    pub total_count: usize,
    /// Time taken to execute in milliseconds
    pub execution_time_ms: u64,
}

impl QueryResult {
    /// Create a new query result
    pub fn new(query_id: impl Into<String>) -> Self {
        Self {
            query_id: query_id.into(),
            results: Vec::new(),
            total_count: 0,
            execution_time_ms: 0,
        }
    }

    /// Add a search result
    pub fn add_result(&mut self, result: SearchResult) {
        self.results.push(result);
        self.total_count += 1;
    }

    /// Set execution time
    pub fn with_execution_time(mut self, time_ms: u64) -> Self {
        self.execution_time_ms = time_ms;
        self
    }
}
