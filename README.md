# File Pipeline Service

A high‑performance file processing pipeline built with **Actix‑web**, **SQLx**, and **Tokio**.  
It follows clean‑architecture principles, providing clear separation between models, repositories, services, and HTTP handlers.

## Features

- RESTful API for managing file metadata
- PostgreSQL persistence with connection pooling
- Input validation using `validator`
- Structured logging with `tracing`
- Docker‑compose for local development
- CI pipeline with linting, testing, and building

## Getting Started

### Prerequisites

- Docker & Docker‑Compose
- Rust toolchain (optional for local dev)

### Clone the repository