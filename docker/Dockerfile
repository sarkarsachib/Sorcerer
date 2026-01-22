# Multi-stage Dockerfile for Sorcerer
FROM rust:1.75-slim as rust-builder

WORKDIR /app

# Copy Rust workspace
COPY rust/ ./rust/
COPY Cargo.toml ./
COPY pyproject.toml ./

# Build Rust workspace
WORKDIR /app/rust
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Rust binaries
COPY --from=rust-builder /app/rust/target/release/sorcerer-search /usr/local/bin/
COPY --from=rust-builder /app/rust/target/release/sorcerer-crawler /usr/local/bin/

# Copy Python code
COPY python/ ./python/

# Install Python dependencies
WORKDIR /app/python
RUN pip3 install --no-cache-dir -r requirements.txt

# Set Python path for module resolution
ENV PYTHONPATH=/app/python:$PYTHONPATH

# Ensure Python is in PATH
ENV PATH=/usr/local/bin:$PATH

# Copy config
COPY config/ /etc/sorcerer/

WORKDIR /app

CMD ["python3", "-m", "sorcerer"]
