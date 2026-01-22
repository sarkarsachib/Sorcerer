-- Enable extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "vector";

-- Core document storage
CREATE TABLE documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content TEXT NOT NULL,
    source VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    trust_score FLOAT DEFAULT 0.5,
    metadata JSONB DEFAULT '{}'::jsonb
);

-- Vector embeddings (pgvector)
CREATE TABLE embeddings (
    id UUID PRIMARY KEY REFERENCES documents(id) ON DELETE CASCADE,
    vector vector(1536),
    created_at TIMESTAMP DEFAULT NOW()
);

-- Named entities
CREATE TABLE entities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID REFERENCES documents(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    confidence FLOAT,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Entity relationships (graph index)
CREATE TABLE relationships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_a_id UUID REFERENCES entities(id) ON DELETE CASCADE,
    entity_b_id UUID REFERENCES entities(id) ON DELETE CASCADE,
    relationship_type VARCHAR(100) NOT NULL,
    confidence FLOAT,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Inverted index (keyword search)
CREATE TABLE inverted_index (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID REFERENCES documents(id) ON DELETE CASCADE,
    keyword VARCHAR(255) NOT NULL,
    position INTEGER NOT NULL,
    frequency INTEGER DEFAULT 1
);

-- Unique constraints for idempotent seed data
ALTER TABLE entities
ADD CONSTRAINT uc_entity_unique UNIQUE (document_id, name, entity_type);

ALTER TABLE relationships
ADD CONSTRAINT uc_relationship_unique UNIQUE (entity_a_id, entity_b_id, relationship_type);

ALTER TABLE inverted_index
ADD CONSTRAINT uc_inverted_index_unique UNIQUE (document_id, keyword, position);

-- Create indexes for performance
CREATE INDEX idx_documents_source ON documents(source);
CREATE INDEX idx_documents_created_at ON documents(created_at);
CREATE INDEX idx_documents_trust_score ON documents(trust_score);
CREATE INDEX idx_inverted_keyword ON inverted_index(keyword);
CREATE INDEX idx_entities_type ON entities(entity_type);
CREATE INDEX idx_entities_document_id ON entities(document_id);
CREATE INDEX idx_relationships_entity_a ON relationships(entity_a_id);
CREATE INDEX idx_relationships_entity_b ON relationships(entity_b_id);
CREATE INDEX idx_embeddings_vector ON embeddings USING ivfflat (vector vector_cosine_ops);
