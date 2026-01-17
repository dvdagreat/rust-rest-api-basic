# REST API

A Rust-based REST API for managing users, built with Axum and SeaORM.

## Features

- **User Management**: Full CRUD operations on users
- **Database**: PostgreSQL with SeaORM ORM
- **Async Runtime**: Tokio for high-performance async operations
- **JSON Serialization**: Serde for request/response handling

## Resources

### Users
- `GET /users` - List all users
- `GET /users/{id}` - Get a specific user
- `POST /users` - Create a new user
- `PUT /users/{id}` - Update a user
- `DELETE /users/{id}` - Delete a user

## Setup

1. Create `.env` file from `.env.example` file:
  ```
  cp .env.example .env
  ```
  
  If it exists, then delete `.env` first and then execute above command:
  ```
  rm -rf .env
  ```

2. Run database migrations from database folder

3. Start the server:
  ```bash
  cargo run
  ```
