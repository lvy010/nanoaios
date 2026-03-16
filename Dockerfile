# ---- Build stage ----
FROM rust:1.85 AS builder

WORKDIR /app

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create dummy main to build dependencies
RUN mkdir src && \
    echo 'fn main() {}' > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source
COPY src ./src

# Touch source files to invalidate the dummy build
RUN touch src/main.rs && \
    cargo build --release

# ---- Runtime stage ----
FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r nanoaios && useradd -r -g nanoaios -d /app -s /sbin/nologin nanoaios

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/nanoaios /app/nanoaios
RUN ln -s /app/nanoaios /usr/local/bin/nanoaios

# Set ownership
RUN chown -R nanoaios:nanoaios /app

USER nanoaios

EXPOSE 4242

ENTRYPOINT ["/app/nanoaios"]
CMD ["start"]