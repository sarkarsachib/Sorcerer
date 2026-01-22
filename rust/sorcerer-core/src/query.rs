//! Query types and constraints for the search system

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Different query modes supported by Sorcerer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryMode {
    /// Traditional keyword-based search
    Keyword,
    /// Vector similarity search (semantic)
    Semantic,
    /// Graph traversal through entity relationships
    Graph,
    /// Time-based temporal queries
    TimeBased,
    /// Local filesystem search
    FileSystem,
    /// External API discovery
    API,
}

/// A search query with constraints and actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// The query mode
    pub mode: QueryMode,
    /// The search text
    pub text: String,
    /// Query constraints
    pub constraints: QueryConstraints,
    /// Actions to perform on results
    pub actions: Vec<QueryAction>,
}

impl SearchQuery {
    /// Create a new search query
    pub fn new(text: impl Into<String>, mode: QueryMode) -> Self {
        Self {
            mode,
            text: text.into(),
            constraints: QueryConstraints::default(),
            actions: Vec::new(),
        }
    }
}

/// Constraints to filter and limit query results
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryConstraints {
    /// Only return results updated after this time
    pub updated_after: Option<DateTime<Utc>>,
    /// Minimum trust score (0.0 - 1.0), defaults to 0.0
    pub min_trust_score: Option<f32>,
    /// Maximum number of results to return
    pub max_results: usize,
}

/// Actions that can be performed on query results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryAction {
    /// Verify the accuracy of results
    Verify,
    /// Compare results against each other
    Compare,
    /// Generate a summary of results
    Summarize,
    /// Execute an action based on results
    Execute,
}
