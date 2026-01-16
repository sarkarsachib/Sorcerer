# Sorcerer: Agentic Search OS

A general-purpose agentic search engine + browser hybrid that performs search, reasoning, crawling, and autonomous action execution.

## Features

- 🔍 **VOID SEARCH API**: Raw, uncensored access to search data
- 🧠 **SINGULARITY CORE**: Multi-step agent reasoning and task decomposition
- 🕷️ **GRIMOIRE CRAWLER**: Decentralized, adaptive web crawling
- 📚 **SPELLBOOK INDEX**: Multi-modal indexing (vector, keyword, graph, temporal)
- 🎯 **ARCANA RANK ENGINE**: Trust-based ranking (no SEO gaming)
- 🌐 **SORCERER BROWSER**: Merged search + browser with inline agent actions

## Quick Start

### Prerequisites
- Rust 1.70+
- Python 3.10+
- Docker & Docker Compose

### Development Setup

```bash
git clone https://github.com/yourusername/sorcerer.git
cd sorcerer

# Start PostgreSQL and Redis
docker-compose -f docker/docker-compose.yml up -d

# Build Rust modules
cd rust && cargo build --all && cd ..

# Setup Python environment
python -m venv venv
source venv/bin/activate
cd python && pip install -r requirements.txt && cd ..

# Run tests
cd rust && cargo test --all && cd ..
cd python && pytest tests/ && cd ..
```

## Architecture

Sorcerer is built on a multi-layered architecture:

```
┌─────────────────────────────┐
│     SORCERER UI/BROWSER     │
└──────────────┬──────────────┘
               │
┌──────────────▼──────────────┐
│  SINGULARITY CORE (Agents)  │
└──────────────┬──────────────┘
               │
┌──────────────▼──────────────┐
│  VOID SEARCH API LAYER      │
└──────────────┬──────────────┘
               │
┌──────────────▼──────────────┐
│ INDEX + CRAWL + OBSERVE     │
└─────────────────────────────┘
```

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for detailed design.

## Development

See [docs/SETUP.md](docs/SETUP.md) for complete setup instructions.

## License

MIT

## Contributing

See [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md)
