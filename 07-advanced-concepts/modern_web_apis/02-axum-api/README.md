# Axum REST API Example

Modern, type-safe REST API using Axum framework with database integration.

## Features

- Type-safe routing and extractors
- JSON request/response handling
- Database integration (SQLx)
- Error handling with custom types
- Middleware (CORS, logging)
- API documentation

## API Endpoints

```
GET    /api/health         # Health check
GET    /api/users          # List all users
POST   /api/users          # Create user
GET    /api/users/:id      # Get user by ID
PUT    /api/users/:id      # Update user
DELETE /api/users/:id      # Delete user
```

## Running

An ordinary build check does not require PostgreSQL because the handlers use typed runtime SQLx queries:

```bash
cargo check
```

Running the server does require PostgreSQL:

```bash
# Start PostgreSQL
docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=password postgres:15

# Set environment
export DATABASE_URL=postgres://postgres:password@localhost/apidb

# Run
cargo run
```

## Testing

```bash
# Health check
curl http://localhost:3000/api/health

# Create user
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"alice@example.com"}'

# List users
curl http://localhost:3000/api/users

# Get user
curl http://localhost:3000/api/users/1

# Update user
curl -X PUT http://localhost:3000/api/users/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice Smith","email":"alice.smith@example.com"}'

# Delete user
curl -X DELETE http://localhost:3000/api/users/1
```

## Architecture

- **Routes** - HTTP endpoint definitions
- **Handlers** - Request processing logic
- **Models** - Data structures
- **State** - Shared application state (DB pool)
- **Errors** - Custom error types
- **Middleware** - Cross-cutting concerns

## Key Learnings

- Axum's type-safe extractors
- Tower middleware composition
- Database connection pooling
- REST API best practices
- Error handling patterns
