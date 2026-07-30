# ---------- Build stage ----------
FROM rust:1.78 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

# ---------- Runtime stage ----------
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/file_pipeline_service /usr/local/bin/file_pipeline_service
COPY --from=builder /app/migrations ./migrations
EXPOSE 8080
ENV RUST_LOG=info
CMD ["file_pipeline_service"]