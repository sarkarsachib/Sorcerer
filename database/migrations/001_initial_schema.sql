-- Initial schema migration for Sorcerer SPELLBOOK INDEX
-- This creates the multi-modal indexing system

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "vector";

-- Core document storage
CREATE TABLE IF NOT EXISTS documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content TEXT NOT NULL,
    source VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    trust_score FLOAT DEFAULT 0.5,
    metadata JSONB DEFAULT '{}'::jsonb
);

-- Vector embeddings (pgvector)
CREATE TABLE IF NOT EXISTS embeddings (
    id UUID PRIMARY KEY REFERENCES documents(id) ON DELETE CASCADE,
    vector vector(1536),
    created_at TIMESTAMP DEFAULT NOW()
);

-- Named entities
CREATE TABLE IF NOT EXISTS entities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID REFERENCES documents(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    confidence FLOAT,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Entity relationships (graph index)
CREATE TABLE IF NOT EXISTS relationships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_a_id UUID REFERENCES entities(id) ON DELETE CASCADE,
    entity_b_id UUID REFERENCES entities(id) ON DELETE CASCADE,
    relationship_type VARCHAR(100) NOT NULL,
    confidence FLOAT,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Inverted index (keyword search)
CREATE TABLE IF NOT EXISTS inverted_index (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID REFERENCES documents(id) ON DELETE CASCADE,
    keyword VARCHAR(255) NOT NULL,
    position INTEGER NOT NULL,
    frequency INTEGER DEFAULT 1
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_documents_source ON documents(source);
CREATE INDEX IF NOT EXISTS idx_documents_created_at ON documents(created_at);
CREATE INDEX IF NOT EXISTS idx_documents_trust_score ON documents(trust_score);
CREATE INDEX IF NOT EXISTS idx_inverted_keyword ON inverted_index(keyword);
CREATE INDEX IF NOT EXISTS idx_entities_type ON entities(entity_type);
CREATE INDEX IF NOT EXISTS idx_entities_document_id ON entities(document_id);
CREATE INDEX IF NOT EXISTS idx_relationships_entity_a ON relationships(entity_a_id);
CREATE INDEX IF NOT EXISTS idx_relationships_entity_b ON relationships(entity_b_id);
CREATE INDEX IF NOT EXISTS idx_embeddings_vector ON embeddings USING ivfflat (vector vector_cosine_ops);
