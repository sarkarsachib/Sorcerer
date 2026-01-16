# Database Migrations

This directory contains SQL migrations for the Sorcerer SPELLBOOK INDEX database.

## Migration Files

Migrations are numbered sequentially (e.g., `001_initial_schema.sql`, `002_add_column.sql`).

## Running Migrations

### Manual Application

```bash
# Connect to database
docker-compose -f ../docker/docker-compose.yml exec postgres psql -U sorcerer -d sorcerer_index

# Apply migrations in order
\i migrations/001_initial_schema.sql
```

### Automated (Future)

Migration automation will be added in Phase 2.

## Naming Convention

- `XXX_description.sql` - Where XXX is the sequential number and description is what the migration does
- Always start with 001 and increment by 1
- Use snake_case for descriptions
- Keep migrations atomic (single logical change per file)

## Best Practices

- Always create migrations that are reversible
- Use `IF NOT EXISTS` for creating tables and indexes
- Test migrations on development database first
- Never modify existing migration files - create new ones instead
- Document breaking changes in migration comments
