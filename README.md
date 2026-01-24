# File Pipeline Service
This project is a file processing pipeline service, designed with modern development practices and clean architecture principles.

## Architecture
The project uses a microservices architecture, with separate modules for models, services, controllers, and repositories.

## Features
- File processing pipeline
- Database integration using PostgreSQL
- Dockerization for easy deployment

## Prerequisites
- Rust 1.63 or higher
- Docker
- PostgreSQL

## Setup
1. Clone the repository: `git clone https://github.com/mahmoudtolba/file-pipeline.git`
2. Create a PostgreSQL database: `createdb -U user file_pipeline`
3. Run the application: `cargo run`
4. Build and deploy the Docker image: `docker build -t file_pipeline .`

## API Endpoints
- `POST /file-pipeline`: Process a file

## Environment Variables
- `PG_USER`: PostgreSQL username
- `PG_PASSWORD`: PostgreSQL password
- `PG_DATABASE`: PostgreSQL database name
- `PG_HOST`: PostgreSQL host

## Author
**Mahmoud Tolba**

## License
This project is licensed under the MIT License - see the LICENSE file for details

#### .gitignore