-- Seed data for Sorcerer development and testing
-- This file contains sample data for development

-- Insert sample documents
INSERT INTO documents (id, content, source, trust_score, metadata) VALUES
    ('550e8400-e29b-41d4-a716-4466554400001'::uuid,
     'Test document about AI safety',
     'https://example.com/ai-safety',
     0.85,
     '{"category": "research", "author": "test"}'::jsonb),
    ('550e8400-e29b-41d4-a716-4466554400002'::uuid,
     'Web scraping best practices',
     'https://example.com/scraping',
     0.72,
     '{"category": "guide", "author": "test"}'::jsonb),
    ('550e8400-e29b-41d4-a716-4466554400003'::uuid,
     'Search engine optimization overview',
     'https://example.com/seo',
     0.65,
     '{"category": "guide", "author": "test"}'::jsonb)
ON CONFLICT (id) DO NOTHING;

-- Insert sample entities
INSERT INTO entities (document_id, name, entity_type, confidence) VALUES
    ('550e8400-e29b-41d4-a716-4466554400001'::uuid, 'AI Safety Institute', 'organization', 0.92),
    ('550e8400-e29b-41d4-a716-4466554400001'::uuid, 'machine learning', 'concept', 0.88),
    ('550e8400-e29b-41d4-a716-4466554400002'::uuid, 'BeautifulSoup', 'tool', 0.95),
    ('550e8400-e29b-41d4-a716-4466554400002'::uuid, 'web scraping', 'technique', 0.90),
    ('550e8400-e29b-41d4-a716-4466554400003'::uuid, 'Google', 'organization', 0.98),
    ('550e8400-e29b-41d4-a716-4466554400003'::uuid, 'ranking algorithm', 'concept', 0.85)
ON CONFLICT (document_id, name, entity_type) DO NOTHING;

-- Insert sample relationships
INSERT INTO relationships (entity_a_id, entity_b_id, relationship_type, confidence)
SELECT
    e1.id, e2.id, 'related_to', 0.80
FROM entities e1
JOIN entities e2 ON e1.entity_type = 'concept' AND e2.entity_type = 'organization'
WHERE e1.document_id = '550e8400-e29b-41d4-a716-4466554400001'::uuid
  AND e2.document_id = '550e8400-e29b-41d4-a716-4466554400001'::uuid
ON CONFLICT (entity_a_id, entity_b_id, relationship_type) DO NOTHING;

-- Insert sample inverted index entries
INSERT INTO inverted_index (document_id, keyword, position, frequency) VALUES
    ('550e8400-e29b-41d4-a716-4466554400001'::uuid, 'AI', 1, 5),
    ('550e8400-e29b-41d4-a716-4466554400001'::uuid, 'safety', 2, 3),
    ('550e8400-e29b-41d4-a716-4466554400001'::uuid, 'machine', 3, 2),
    ('550e8400-e29b-41d4-a716-4466554400002'::uuid, 'scraping', 1, 4),
    ('550e8400-e29b-41d4-a716-4466554400002'::uuid, 'web', 2, 3),
    ('550e8400-e29b-41d4-a716-4466554400003'::uuid, 'search', 1, 6),
    ('550e8400-e29b-41d4-a716-4466554400003'::uuid, 'engine', 2, 4)
ON CONFLICT (document_id, keyword, position) DO NOTHING;

-- Insert sample embeddings (1536-dim vectors)
INSERT INTO embeddings (id, vector, created_at)
SELECT
    id,
    (ARRAY[0.1, 0.2, 0.3] || ARRAY_FILL(0.0::float, ARRAY[1533]))::vector,
    NOW()
FROM documents
ON CONFLICT (id) DO NOTHING;

-- Display seed data summary
SELECT
    'documents' as table_name,
    COUNT(*) as record_count
FROM documents
UNION ALL
SELECT 'entities', COUNT(*) FROM entities
UNION ALL
SELECT 'relationships', COUNT(*) FROM relationships
UNION ALL
SELECT 'inverted_index', COUNT(*) FROM inverted_index
UNION ALL
SELECT 'embeddings', COUNT(*) FROM embeddings;
