# File Processing Pipeline
A file processing pipeline service written in Rust.

## Overview
This project is a file processing pipeline service that provides a REST API for creating and retrieving files.

## Architecture
The project uses a clean architecture approach, separating concerns into layers:
- Models: Represent the data structures used in the application.
- Repositories: Encapsulate data access logic.
- Services: Contain business logic.
- Controllers: Handle incoming requests and return responses.

## Prerequisites
- Rust 1.56.0 or later
- PostgreSQL 13 or later
- Docker

## Setup
1. Clone the repository.
2. Create a PostgreSQL database and update the `DATABASE_URL` environment variable in `.env.example`.
3. Run `docker-compose up` to start the application.

## API Endpoints
- `POST /file`: Create a new file.
- `GET /file`: Retrieve all files.

## Environment Variables
- `PORT`: The port number to listen on (default: 8080).
- `DATABASE_URL`: The PostgreSQL database URL (default: postgres://user:password@localhost/database).
- `RUST_LOG`: The log level (default: info).

## Docker Setup
1. Build the Docker image using `docker build -t file-processing-pipeline .`.
2. Run the Docker container using `docker run -p 8080:8080 file-processing-pipeline`.

## Development Workflow
1. Run `cargo build` to build the application.
2. Run `cargo test` to run tests.
3. Use a tool like `curl` to test the API endpoints.

## Troubleshooting
- Check the log output for errors.
- Use a debugger like `lldb` to step through the code.

## Author
**Mahmoud Tolba**

## License
This project is licensed under the MIT License - see the LICENSE file for details