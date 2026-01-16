# Contributing to Sorcerer

Thank you for your interest in contributing to Sorcerer! This document provides guidelines for contributing.

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Assume good intentions

## Getting Started

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Write tests for your changes
5. Ensure all tests pass
6. Submit a pull request

## Development Guidelines

### Code Style

#### Rust

- Use `cargo fmt` for formatting
- Run `cargo clippy --all -- -D warnings` before committing
- Follow Rust API guidelines
- Document public APIs with `///` doc comments
- Use meaningful variable and function names

#### Python

- Follow PEP 8 style guide
- Use `black` for formatting: `black python/`
- Use `isort` for import sorting: `isort python/`
- Type hint all functions
- Document public APIs with docstrings

### Testing

#### Rust Tests

```bash
# Run all tests
cd rust && cargo test --all

# Run specific test
cargo test --package sorcerer-core --lib agent::tests::test_task_creation

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --all --release
```

#### Python Tests

```bash
# Run all tests
cd python && pytest tests/ -v

# Run specific test file
pytest tests/test_agent.py -v

# Run with coverage
pytest tests/ --cov=sorcerer --cov-report=html

# Run with debug output
pytest tests/ -v -s
```

### Commit Messages

Follow conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Test changes
- `chore`: Maintenance tasks

Examples:
```
feat(core): add agent orchestration system

Implement basic task decomposition and
agent coordination for multi-step reasoning.

Closes #123
```

```
fix(index): resolve vector search timeout

Increase timeout for pgvector queries
to handle large document sets.
```

## Project Structure

```
sorcerer/
├── rust/                   # Rust workspace
│   ├── sorcerer-core/      # Core abstractions
│   ├── sorcerer-search/     # Search API
│   └── sorcerer-crawler/    # Crawler system
├── python/                 # Python package
│   └── sorcerer/
│       ├── agent/           # Agent system
│       ├── config/          # Configuration
│       └── utils/          # Utilities
├── config/                 # Configuration files
├── database/               # Database schema and migrations
├── docker/                 # Docker configuration
└── docs/                   # Documentation
```

## Making Changes

### Adding New Features

1. Create an issue first to discuss the feature
2. Design the feature with documentation
3. Implement with tests
4. Update relevant documentation
5. Submit pull request

### Fixing Bugs

1. Create an issue describing the bug
2. Add a test that reproduces the bug
3. Fix the bug
4. Ensure the test passes
5. Submit pull request

### Documentation Updates

- Update API docs when adding/changing APIs
- Update ARCHITECTURE.md for structural changes
- Update SETUP.md for new dependencies
- Keep examples up to date

## Pull Request Process

1. Update documentation
2. Ensure all tests pass
3. Update CHANGELOG.md
4. Submit PR with description:
   - What changes were made
   - Why the changes are needed
   - Link to related issues
   - Screenshots for UI changes

## Review Process

- All PRs require at least one approval
- CI must pass before merging
- Address review feedback promptly
- Be patient with reviews

## Release Process

1. Update version numbers
2. Update CHANGELOG.md
3. Tag release: `git tag -a v0.1.0 -m "Release v0.1.0"`
4. Push tags: `git push origin v0.1.0`
5. Create GitHub release

## Questions?

- Open an issue
- Join community discussions
- Ask in pull request comments

Thank you for contributing to Sorcerer! 🚀
