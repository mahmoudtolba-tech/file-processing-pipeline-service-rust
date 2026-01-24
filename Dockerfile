FROM rust:1.63-slim AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM postgres:12
ENV PG_USER=user
ENV PG_PASSWORD=password
ENV PG_DATABASE=database

FROM alpine:latest
WORKDIR /app
COPY --from=build /app/target/release/file_pipeline .
EXPOSE 8080
CMD ["./file_pipeline"]

#### docker-compose.yml