# File Processing Pipeline Service

A high‑performance Rust service built with **Actix‑web**, **SQLx**, and **Tokio** that provides a simple API for uploading file metadata, listing files, retrieving a single file, and triggering a processing step.

## Features

- Clean architecture with separation of models, repositories, services and handlers
- Async PostgreSQL access with connection pooling
- Comprehensive error handling & input validation
- Structured logging via `tracing`
- Docker multi‑stage build and `docker‑compose` for local development
- CI pipeline with linting, testing and building

## Getting Started

### Prerequisites

- Docker & Docker Compose
- Rust toolchain (for local dev)
- PostgreSQL (if not using Docker)

### Running with Docker