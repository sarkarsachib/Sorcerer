//! VOID SEARCH API - Raw, uncensored search layer

use sorcerer_core::{QueryResult, SearchQuery, SorcererError};
use uuid::Uuid;

/// VOID SEARCH API - Provides raw access to search data
pub struct SearchApi {
    // Will be implemented in Phase 2
}

impl SearchApi {
    /// Create a new Search API instance
    pub fn new() -> Self {
        Self {}
    }

    /// Execute a search query
    pub async fn search(&self, _query: SearchQuery) -> Result<QueryResult, SorcererError> {
        // Placeholder - will be implemented in Phase 2
        Ok(QueryResult::new(Uuid::new_v4().to_string()))
    }
}

impl Default for SearchApi {
    fn default() -> Self {
        Self::new()
    }
}
