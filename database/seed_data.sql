-- Seed data for Sorcerer development and testing
-- This file contains sample data for development

-- Insert sample documents
INSERT INTO documents (id, content, source, trust_score, metadata) VALUES
    ('11111111-1111-1111-1111-111111111111',
     'Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.',
     'https://www.rust-lang.org',
     0.95,
     '{"category": "programming", "language": "rust"}'::jsonb),
    ('22222222-2222-2222-2222-222222222222',
     'Python is an interpreted, high-level, general-purpose programming language.',
     'https://www.python.org',
     0.98,
     '{"category": "programming", "language": "python"}'::jsonb),
    ('33333333-3333-3333-3333-333333333333',
     'Machine learning is a subset of artificial intelligence that enables systems to learn from data.',
     'https://example.com/ml-intro',
     0.75,
     '{"category": "ai", "topic": "machine-learning"}'::jsonb)
ON CONFLICT (id) DO NOTHING;

-- Insert sample entities
INSERT INTO entities (document_id, name, entity_type, confidence) VALUES
    ('11111111-1111-1111-1111-111111111111', 'Rust', 'programming_language', 0.99),
    ('11111111-1111-1111-1111-111111111111', 'Mozilla', 'organization', 0.85),
    ('22222222-2222-2222-2222-222222222222', 'Python', 'programming_language', 0.99),
    ('22222222-2222-2222-2222-222222222222', 'Guido van Rossum', 'person', 0.90),
    ('33333333-3333-3333-3333-333333333333', 'Machine Learning', 'technology', 0.95),
    ('33333333-3333-3333-3333-333333333333', 'Artificial Intelligence', 'technology', 0.92)
ON CONFLICT DO NOTHING;

-- Insert sample relationships
INSERT INTO relationships (entity_a_id, entity_b_id, relationship_type, confidence) VALUES
    ((SELECT id FROM entities WHERE name = 'Rust' LIMIT 1),
     (SELECT id FROM entities WHERE name = 'Mozilla' LIMIT 1),
     'created_by',
     0.85),
    ((SELECT id FROM entities WHERE name = 'Python' LIMIT 1),
     (SELECT id FROM entities WHERE name = 'Guido van Rossum' LIMIT 1),
     'created_by',
     0.90),
    ((SELECT id FROM entities WHERE name = 'Machine Learning' LIMIT 1),
     (SELECT id FROM entities WHERE name = 'Artificial Intelligence' LIMIT 1),
     'subset_of',
     0.95)
ON CONFLICT DO NOTHING;

-- Insert sample inverted index entries
INSERT INTO inverted_index (document_id, keyword, position, frequency) VALUES
    ('11111111-1111-1111-1111-111111111111', 'rust', 0, 3),
    ('11111111-1111-1111-1111-111111111111', 'programming', 1, 2),
    ('11111111-1111-1111-1111-111111111111', 'language', 2, 1),
    ('22222222-2222-2222-2222-222222222222', 'python', 0, 2),
    ('22222222-2222-2222-2222-222222222222', 'interpreted', 1, 1),
    ('22222222-2222-2222-2222-222222222222', 'general-purpose', 2, 1),
    ('33333333-3333-3333-3333-333333333333', 'machine', 0, 1),
    ('33333333-3333-3333-3333-333333333333', 'learning', 1, 2),
    ('33333333-3333-3333-3333-333333333333', 'artificial', 2, 1),
    ('33333333-3333-3333-3333-333333333333', 'intelligence', 3, 1)
ON CONFLICT DO NOTHING;

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
SELECT 'inverted_index', COUNT(*) FROM inverted_index;
