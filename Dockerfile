# =========================================================================
# Stage 1: Build the release binary
# =========================================================================
FROM rust:1-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/worpen

# Copy workspace manifests
COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/crates ./crates

# Build release binary
RUN cargo build --release --bin worpen && \
    strip target/release/worpen

# =========================================================================
# Stage 2: Minimal hardened runtime image
# =========================================================================
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Hardened unprivileged system user
RUN groupadd -g 10001 worpen && \
    useradd -u 10001 -g worpen -s /usr/sbin/nologin -d /app worpen

# Persistent state & secret isolation mount directories
RUN mkdir -p /app /data /secrets && \
    chown -R worpen:worpen /app /data /secrets

COPY --from=builder --chown=worpen:worpen /usr/src/worpen/target/release/worpen /usr/local/bin/worpen

USER worpen
WORKDIR /app

ENV HOST=0.0.0.0 \
    PORT=3000 \
    DATA_DIR=/data \
    DATABASE_URL=sqlite:/data/worpen.db?mode=rwc

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD ["worpen", "doctor"] || exit 1

ENTRYPOINT ["worpen"]
CMD ["serve"]
