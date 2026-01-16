# VOID SEARCH API Specification

The VOID SEARCH API provides raw, uncensored access to search data through multiple query modes.

## Overview

The API is designed to be:
- **Censorship-free**: No content filtering or bias
- **Multi-modal**: Supports different search paradigms
- **Flexible**: Constraint-based filtering
- **Extensible**: Action-based result processing

## Query Modes

### Keyword Mode

Traditional full-text search using the inverted index.

```json
{
  "mode": "keyword",
  "text": "machine learning algorithms",
  "constraints": {
    "max_results": 50,
    "min_trust_score": 0.3
  }
}
```

### Semantic Mode

Vector similarity search using embeddings.

```json
{
  "mode": "semantic",
  "text": "artificial intelligence",
  "constraints": {
    "max_results": 20,
    "min_trust_score": 0.5
  }
}
```

### Graph Mode

Entity relationship traversal for connected information.

```json
{
  "mode": "graph",
  "text": "Elon Musk",
  "constraints": {
    "max_results": 30
  }
}
```

### TimeBased Mode

Temporal queries with time-based filtering.

```json
{
  "mode": "timebased",
  "text": "AI news",
  "constraints": {
    "max_results": 25,
    "updated_after": "2024-01-01T00:00:00Z"
  }
}
```

### FileSystem Mode

Local file system search (desktop/CLI usage).

```json
{
  "mode": "filesystem",
  "text": "project documentation",
  "constraints": {
    "max_results": 100
  }
}
```

### API Mode

External API discovery and integration.

```json
{
  "mode": "api",
  "text": "weather API",
  "constraints": {
    "max_results": 10
  }
}
```

## Query Constraints

```typescript
interface QueryConstraints {
  updated_after?: DateTime;  // Only results updated after this time
  min_trust_score: float;    // Minimum trust score (0.0 - 1.0)
  max_results: number;         // Maximum number of results
}
```

## Query Actions

Actions that can be applied to search results:

```typescript
enum QueryAction {
  Verify,    // Fact-check results against multiple sources
  Compare,    // Compare results and identify contradictions
  Summarize,  // Generate a summary of all results
  Execute     // Trigger an action based on results
}
```

### Example: Verify Action

```json
{
  "mode": "semantic",
  "text": "climate change statistics",
  "constraints": {
    "max_results": 10,
    "min_trust_score": 0.6
  },
  "actions": ["verify"]
}
```

This would:
1. Execute the search query
2. Have Verifier agents cross-check each result
3. Return results with verification confidence

## Result Format

```typescript
interface SearchResult {
  id: string;                // Unique result ID
  title: string;              // Result title
  content: string;            // Content snippet
  source: string;            // Source URL or identifier
  confidence: number;         // Relevance confidence (0.0 - 1.0)
  trust_score: number;        // Trust score from ARCANA (0.0 - 1.0)
  metadata: {                // Additional metadata
    [key: string]: any
  };
}
```

## Query Result

```typescript
interface QueryResult {
  query_id: string;          // Query identifier
  results: SearchResult[];    // Array of results
  total_count: number;       // Total matching results
  execution_time_ms: number;  // Execution time in milliseconds
}
```

## Error Responses

```typescript
interface ErrorResponse {
  error: string;             // Error message
  error_code: string;        // Machine-readable error code
  details?: {               // Additional error details
    [key: string]: any
  };
}
```

## Implementation Status

- ✅ Core types and structures defined
- ⏳ HTTP/gRPC endpoints (Phase 3)
- ⏳ Query execution engine (Phase 3)
- ⏳ Result streaming (Phase 3)
- ⏳ Agent integration (Phase 5)

## Future Enhancements

- Query history and saved searches
- Batch query execution
- Result caching with TTL
- Advanced query operators (AND, OR, NOT)
- Custom ranking functions
- Federated search across multiple indices
