# ---- Builder Stage ----
FROM rust:1.78-alpine AS builder
WORKDIR /app
# Install required packages
RUN apk add --no-cache libc-dev openssl-dev musl-dev git
# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --locked
# Actual source
COPY . .
RUN cargo test --release
RUN cargo build --release --locked

# ---- Runtime Stage ----
FROM alpine:3.20 AS runtime
WORKDIR /app
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/file_pipeline_service .
ENV RUST_LOG=info
EXPOSE 8080
ENTRYPOINT ["./file_pipeline_service"]