# Stage 1: Build the application
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Build the Docker image
FROM postgres:latest
RUN apt-get update && apt-get install -y libpq-dev
COPY --from=builder /app/target/release/file_processing_pipeline /app/
WORKDIR /app
CMD ["./file_processing_pipeline"]
EXPOSE 8080