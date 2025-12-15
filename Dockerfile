FROM rust:latest AS build

WORKDIR /app

COPY Cargo.toml .

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=build /app/target/release/file_processing_pipeline .

ENV RUST_LOG=info
ENV DATABASE_URL=postgres://user:password@localhost/database
ENV PORT=8080

CMD ["./file_processing_pipeline"]