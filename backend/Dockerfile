# =============================================================================
# Base Stage
# =============================================================================
FROM rust:1.85 AS base
WORKDIR /workspace/backend

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install cargo tools with --locked to use compatible versions
RUN cargo install cargo-watch --locked
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres --locked

# Copy wait-for-it.sh
COPY wait-for-it.sh /usr/local/bin/wait-for-it.sh
RUN chmod +x /usr/local/bin/wait-for-it.sh

# =============================================================================
# Development Stage
# =============================================================================
FROM base AS dev

ENV RUST_LOG=debug
ENV RUST_BACKTRACE=1

# Development entry point (overridden in compose.yaml)
CMD ["cargo", "watch", "-x", "run"]

# =============================================================================
# Builder Stage (for production)
# =============================================================================
FROM base AS builder

COPY backend/Cargo.toml backend/Cargo.lock* ./
COPY backend/src ./src

RUN cargo build --release

# =============================================================================
# Production Stage
# =============================================================================
FROM debian:bookworm-slim AS prod

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /workspace/backend/target/release/todo-backend /app/todo-backend
COPY wait-for-it.sh /usr/local/bin/wait-for-it.sh
RUN chmod +x /usr/local/bin/wait-for-it.sh

EXPOSE 3000

CMD ["./todo-backend"]
