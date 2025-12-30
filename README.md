# File Processing Pipeline

This is a file processing pipeline service written in Rust.

## Overview

The service provides a REST API for processing files. It uses a PostgreSQL database to store file metadata and a Redis cache for performance optimization.

## Features

* Process files
* Health check endpoint

## Architecture

The service follows a clean architecture design with separate layers for the application, domain, and infrastructure.

+---------------+
|  Application  |
+---------------+
           |
           |
           v
+---------------+
|    Domain     |
+---------------+
           |
           |
           v
+---------------+
| Infrastructure|
+---------------+

## Prerequisites

* Rust 1.62 or later
* PostgreSQL 13 or later
* Redis 6 or later
* Docker

## Setup

1. Clone the repository
2. Create a `.env` file with the required environment variables
3. Run `docker-compose up` to start the services
4. Run `cargo build` to build the application
5. Run `cargo run` to start the application

## API Endpoints

### Process File

* **POST /process_file**: Process a file
	+ Request Body: `file_path` (string)
	+ Response: `200 OK` with a JSON response

### Health Check

* **GET /health_check**: Check the health of the service
	+ Response: `200 OK` with a JSON response

## Development Workflow

1. Clone the repository
2. Create a new branch for your feature or bug fix
3. Make changes to the code
4. Run `cargo test` to run the tests
5. Run `docker-compose up` to start the services
6. Run `cargo run` to start the application
7. Test your changes
8. Commit your changes
9. Push your changes to the remote repository
10. Create a pull request

## Testing

1. Run `cargo test` to run the tests

## Troubleshooting

* Check the logs for errors
* Check the database for issues
* Check the Redis cache for issues

## Author

**Mahmoud Tolba**

## License

This project is licensed under the MIT License - see the LICENSE file for details