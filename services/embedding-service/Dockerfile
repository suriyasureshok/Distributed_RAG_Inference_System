FROM rust:1.75

WORKDIR /app

COPY . .

RUN cargo build --release

RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app

USER appuser

CMD ["./target/release/embedding-service"]