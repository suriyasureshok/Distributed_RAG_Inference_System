FROM rust:1.75 AS builder

WORKDIR /app

# Copy manifests first to maximize Docker layer caching.
COPY Cargo.toml Cargo.lock ./
COPY shared/Cargo.toml shared/Cargo.toml
COPY services/api-gateway/Cargo.toml services/api-gateway/Cargo.toml
COPY services/embedding-service/Cargo.toml services/embedding-service/Cargo.toml
COPY services/rag-service/Cargo.toml services/rag-service/Cargo.toml
COPY services/llm-service/Cargo.toml services/llm-service/Cargo.toml

# Resolve dependencies early to reuse cache when only source changes.
RUN cargo fetch

# Copy real source code and build release binary.
COPY shared shared
COPY services services
RUN cargo build --release -p llm-service

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
	&& apt-get install -y --no-install-recommends ca-certificates libssl3 \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN useradd --uid 1000 --create-home --shell /usr/sbin/nologin appuser

COPY --from=builder --chown=appuser:appuser /app/target/release/llm-service /app/llm-service

USER appuser

CMD ["/app/llm-service"]