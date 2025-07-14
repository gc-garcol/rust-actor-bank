# Multi-stage Dockerfile for Rust Actix application
# Stage 1: Build dependencies (cacheable layer)
FROM rustlang/rust:nightly-slim AS dependencies

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    clang \
    libclang-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies with nightly features (this layer will be cached)
RUN CARGO_UNSTABLE_SPARSE_REGISTRY=true cargo +nightly build --release && rm -rf src target/release/deps/actor_bank*

# Stage 2: Build application
FROM dependencies AS builder

# Copy source code
COPY src ./src

# Build application with nightly features
RUN CARGO_UNSTABLE_SPARSE_REGISTRY=true cargo +nightly build --release

# Stage 3: Runtime
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 appuser

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/actor-bank /app/actor-bank

# Copy any additional files (config, static files, etc.)
# COPY --from=builder /app/config ./config
# COPY --from=builder /app/static ./static

# Change ownership to non-root user
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port (adjust as needed)
EXPOSE 8080

# Run the application
CMD ["./actor-bank"]
