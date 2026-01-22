# Development Setup

## Prerequisites

Before setting up Sorcerer, ensure you have the following installed:

- **Rust** 1.70+ ([Install Rust](https://www.rust-lang.org/tools/install))
- **Python** 3.10+ ([Install Python](https://www.python.org/downloads/))
- **Docker** & **Docker Compose** ([Install Docker](https://docs.docker.com/get-docker/))
- **Git** ([Install Git](https://git-scm.com/downloads))

## Quick Start

### 1. Clone Repository

```bash
git clone https://github.com/yourusername/sorcerer.git
cd sorcerer
```

### 2. Start Services

Start PostgreSQL and Redis using Docker Compose:

```bash
docker-compose -f docker/docker-compose.yml up -d
```

### 3. Wait for Services

Wait for PostgreSQL to be ready (typically 10-15 seconds):

```bash
# Check PostgreSQL is ready
docker-compose -f docker/docker-compose.yml exec postgres pg_isready

# Should output: sorcerer:5432 - accepting connections
```

### 4. Build Rust Workspace

```bash
cd rust
cargo build --all
```

### 5. Run Tests

```bash
cargo test --all
```

### 6. Setup Python Environment

Create a Python virtual environment and install dependencies:

```bash
cd ..

python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

cd python
pip install -r requirements.txt
```

### 7. Run Python Tests

```bash
pytest tests/ -v
```

## Verifying Setup

### Check PostgreSQL

```bash
# Connect to PostgreSQL
docker-compose -f docker/docker-compose.yml exec postgres psql -U sorcerer -d sorcerer_index

# Check version
SELECT version();

# Check tables
\dt

# Exit
\q
```

Expected output:

```text
             version
 -----------------------------------------------
 PostgreSQL 16.x on x86_64-pc-linux-gnu

           List of relations
 Schema |      Name      | Type  |  Owner
 --------+----------------+-------+----------
  public | documents      | table  | sorcerer
  public | embeddings     | table  | sorcerer
  public | entities       | table  | sorcerer
  public | inverted_index | table  | sorcerer
  public | relationships  | table  | sorcerer
```

### Check Redis

```bash
# Connect to Redis
docker-compose -f docker/docker-compose.yml exec redis redis-cli

# Test connection
PING

# Exit
EXIT
```

Expected output: `PONG`

### Check Rust Compilation

```bash
cd rust
cargo build --all --release
```

Expected output: `Finished release [optimized] target(s) in X.XXs`

### Check Python Setup

```bash
cd python
python -c "import sorcerer; print(sorcerer.__version__)"
```

Expected output: `0.1.0`

## Development Workflow

### Running Development Servers

```bash
# Start PostgreSQL and Redis
docker-compose -f docker/docker-compose.yml up -d

# Watch for file changes and rebuild Rust
cd rust
cargo watch -x test -x run

# Run Python tests in watch mode
cd python
pytest-watch tests/
```

### Database Migrations

```bash
# Connect to PostgreSQL
docker-compose -f docker/docker-compose.yml exec postgres psql -U sorcerer -d sorcerer_index

# Apply schema
\i /docker-entrypoint-initdb.d/01-schema.sql
```

### Environment Configuration

Copy the example environment file and configure:

```bash
cp .env.example .env
# Edit .env with your settings
```

Key environment variables:
- `DATABASE_URL`: PostgreSQL connection string
- `REDIS_URL`: Redis connection string
- `LOG_LEVEL`: Logging level (debug, info, warning, error)
- `RUST_LOG`: Rust-specific logging filter

### Debugging

#### Rust Debugging

```bash
# Enable debug logging
export RUST_LOG=sorcerer_core=debug,sorcerer_search=debug

# Run with backtrace
export RUST_BACKTRACE=1
cargo run
```

#### Python Debugging

```bash
# Enable debug logging
export LOG_LEVEL=debug

# Run with debugger
python -m pdb -m sorcerer
```

### IDE Setup

#### VS Code

Install extensions:
- rust-analyzer (Rust)
- Python (Microsoft)
- Python Test Explorer
- TOML Language Server

#### PyCharm

Install plugins:
- Rust Plugin
- TOML Support

## Troubleshooting

### PostgreSQL Won't Start

```bash
# Check logs
docker-compose -f docker/docker-compose.yml logs postgres

# Reset database (WARNING: destroys data)
docker-compose -f docker/docker-compose.yml down -v
docker-compose -f docker/docker-compose.yml up -d
```

### Rust Build Fails

```bash
# Clean build artifacts
cd rust
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build --all
```

### Python Import Errors

```bash
# Ensure virtual environment is activated
source venv/bin/activate  # or venv\Scripts\activate on Windows

# Reinstall dependencies
pip install -r requirements.txt --force-reinstall
```

### Port Conflicts

If ports 5432 or 6379 are already in use, modify `docker/docker-compose.yml`:

```yaml
services:
  postgres:
    ports:
      - "5433:5432"  # Use 5433 instead

  redis:
    ports:
      - "6380:6379"  # Use 6380 instead
```

## Production Deployment

See `docker/docker-compose.prod.yml` for production configuration.

Key production differences:
- Larger connection pools
- Higher batch sizes
- Environment variable configuration
- Health checks and restart policies
- Volume persistence

## Next Steps

After setup:

1. Read [ARCHITECTURE.md](ARCHITECTURE.md) for system design
2. Review [API.md](API.md) for API specification
3. Check [AGENTS.md](AGENTS.md) for agent system details
4. Start contributing! See [CONTRIBUTING.md](CONTRIBUTING.md)

## Getting Help

- Open an issue on GitHub
- Join our community Discord
- Check documentation in `/docs`
- Review code comments and examples
