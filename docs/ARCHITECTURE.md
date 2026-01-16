# Sorcerer Architecture

## High-Level Design

Sorcerer is a multi-layered agentic search engine consisting of:

1. **SPELLBOOK INDEX** - Multi-modal indexing system (vector, keyword, graph, temporal)
2. **VOID SEARCH API** - Raw, uncensored search layer
3. **GRIMOIRE CRAWLER** - Decentralized crawling network
4. **SINGULARITY CORE** - Agent reasoning engine
5. **ARCANA RANK ENGINE** - Trust-based ranking
6. **SORCERER BROWSER** - Agentic browser interface

## System Layers

```
┌─────────────────────────────┐
│     SORCERER UI/BROWSER     │  (Phase 6)
└──────────────┬──────────────┘
               │
┌──────────────▼──────────────┐
│  SINGULARITY CORE (Agents)  │  (Phase 5)
│  - Task Decomposition         │
│  - Agent Orchestration        │
│  - Multi-step Reasoning       │
└──────────────┬──────────────┘
               │
┌──────────────▼──────────────┐
│  VOID SEARCH API LAYER      │  (Phase 3)
│  - Multi-mode Query          │
│  - Result Aggregation        │
│  - Trust Filtering           │
└──────────────┬──────────────┘
               │
┌──────────────▼──────────────┐
│ INDEX + CRAWL + OBSERVE     │  (Phase 2 & 4)
│  - Vector Search             │
│  - Keyword Search            │
│  - Graph Traversal          │
│  - Web Crawling             │
└─────────────────────────────┘
```

## Core Components

### SPELLBOOK INDEX

Multi-modal indexing system supporting:
- **Vector embeddings** (pgvector with OpenAI-style 1536 dimensions)
- **Inverted index** for full-text keyword search
- **Entity graph** for relationship-based queries
- **Temporal indexing** for time-based searches

Database tables:
- `documents` - Core document storage
- `embeddings` - Vector embeddings with pgvector
- `entities` - Named entity extraction
- `relationships` - Entity relationship graph
- `inverted_index` - Keyword search index

### VOID SEARCH API

Raw, uncensored search API providing:
- Multiple query modes (keyword, semantic, graph, time-based, filesystem, API)
- Flexible constraint filtering
- Action-based post-processing (verify, compare, summarize, execute)
- Real-time result streaming

### GRIMOIRE CRAWLER

Decentralized web crawling with:
- Concurrent crawling with rate limiting
- Adaptive politeness policies
- Content extraction and normalization
- Entity extraction and linking

### SINGULARITY CORE

Agent reasoning engine featuring:
- Specialized agents (Scout, Analyst, Verifier, Executor, Memory)
- Task decomposition and parallel execution
- Long-term context management
- Multi-step reasoning chains

## Phase 1: Foundation

Phase 1 focuses on:
- Core type abstractions (Agent, Index, Query, Result)
- PostgreSQL + pgvector index infrastructure
- Basic search API structure
- Simple crawler framework
- Agent orchestration primitives

See SETUP.md for development environment setup.

## Technology Stack

### Rust Core
- **Runtime**: Tokio async runtime
- **Database**: SQLx for compile-time checked queries
- **Serialization**: Serde for JSON/TOML
- **Error Handling**: thiserror for structured errors
- **Logging**: tracing for structured logging

### Python Layer
- **Agent System**: Async agent orchestration
- **Configuration**: TOML-based config loader
- **Utilities**: Loguru for logging, pytest for testing
- **Integration**: asyncio for async operations

### Infrastructure
- **Database**: PostgreSQL 16 + pgvector extension
- **Caching**: Redis for session and queue management
- **Containerization**: Docker & Docker Compose
- **CI/CD**: GitHub Actions for automated testing

## Data Flow

### Query Flow
1. User submits query through UI/API
2. Query parsed and mode determined
3. Appropriate index queried (vector/keyword/graph)
4. Results filtered by trust score and constraints
5. Agents process results (verify, compare, summarize)
6. Final results returned to user

### Indexing Flow
1. Crawler fetches new content
2. Content normalized and processed
3. Entities extracted and linked
4. Embeddings generated
5. Multi-modal indexes updated
6. Trust scores calculated (ARCANA RANK)

## Configuration

Sorcerer uses TOML-based configuration with environment-specific overrides:
- `config/sorcerer.toml` - Default configuration
- `config/dev.toml` - Development overrides
- `config/prod.toml` - Production overrides

Environment variables can override config values (see .env.example).
