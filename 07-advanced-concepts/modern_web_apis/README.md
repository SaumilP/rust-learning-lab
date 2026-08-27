# Section 14: Modern Web APIs

## Overview

Build production-grade REST and GraphQL APIs with Rust web frameworks. Learn type-safe routing, middleware, authentication, and API design patterns.

## Why Rust for Web APIs?

✅ **Performance** - Handle 100K+ requests/second <br />
✅ **Type Safety** - Compile-time API validation <br />
✅ **Memory Efficient** - Low memory footprint <br />
✅ **Concurrent** - Async I/O with Tokio <br />
✅ **Reliable** - No runtime crashes

## Web Frameworks

### Actix-Web (High Performance)
- Actor-based framework
- Fastest Rust web framework
- Mature ecosystem

### Axum (Ergonomic)
- Built on Tokio and Hyper
- Type-safe extractors
- Excellent Tower middleware

### Rocket (Developer Friendly)
- Macro-based routing
- Easy to learn
- Great documentation

### Warp (Functional)
- Filter-based routing
- Composable
- Type-driven

## What You'll Learn

1. **REST APIs** - CRUD operations, status codes
2. **JSON Handling** - serde serialization
3. **Routing** - Path parameters, query strings
4. **Middleware** - Logging, auth, CORS
5. **Error Handling** - Custom error types
6. **Database Integration** - SQLx, Diesel
7. **Authentication** - JWT, OAuth2
8. **GraphQL** - async-graphql
9. **API Documentation** - OpenAPI/Swagger
10. **Testing** - Integration and unit tests

## Section Contents

### 01-actix-rest-api/
Complete REST API with Actix-Web

### 02-axum-api/
Modern API with Axum framework

### 03-jwt-authentication/
JWT-based authentication

### 04-graphql-server/
GraphQL API with async-graphql

### 05-api-versioning/
API versioning strategies

### 06-rate-limiting/
Request rate limiting

## Prerequisites

- Rust installed
- Understanding of async/await
- Basic HTTP knowledge
- Familiarity with JSON

## Quick Start

```bash
# Create new API project
cargo new my-api
cd my-api

# Add dependencies
cargo add axum tokio serde
cargo add serde_json tower-http

# Run
cargo run
```

## Project Structure

```
14-modern-web-apis/
├── README.md
├── 01-actix-rest-api/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── routes/
│   │   ├── models/
│   │   ├── handlers/
│   │   └── middleware/
│   └── README.md
├── 02-axum-api/
├── 03-jwt-authentication/
├── 04-graphql-server/
├── 05-api-versioning/
└── 06-rate-limiting/
```

## Key Concepts

### REST API with Axum

```rust
use axum::{
    routing::{get, post},
    Router, Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
}

async fn get_user(Path(id): Path<u64>) -> Json<User> {
    Json(User { id, name: "Alice".to_string() })
}

async fn create_user(Json(user): Json<User>) -> Json<User> {
    // Create user in database
    Json(user)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

### Error Handling

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("User not found: {0}")]
    NotFound(u64),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Unauthorized")]
    Unauthorized,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

### Middleware

```rust
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};

let app = Router::new()
    .route("/api/users", get(list_users))
    .layer(CorsLayer::permissive())
    .layer(TraceLayer::new_for_http());
```

### JWT Authentication

```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn create_jwt(user_id: &str) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap()
}

fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default()
    ).map(|data| data.claims)
}
```

### GraphQL with async-graphql

```rust
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

struct Query;

#[Object]
impl Query {
    async fn hello(&self, name: String) -> String {
        format!("Hello, {}!", name)
    }

    async fn user(&self, id: i32) -> User {
        User { id, name: "Alice".to_string() }
    }
}

let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
```

### Database Integration

```rust
use sqlx::PgPool;
use axum::extract::State;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

async fn list_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&state.db)
        .await
        .unwrap();

    Json(users)
}

let app = Router::new()
    .route("/users", get(list_users))
    .with_state(AppState { db: pool });
```

## API Design Patterns

### RESTful Resources

```
GET    /api/users          # List all users
POST   /api/users          # Create user
GET    /api/users/:id      # Get user
PUT    /api/users/:id      # Update user
DELETE /api/users/:id      # Delete user
PATCH  /api/users/:id      # Partial update
```

### Pagination

```rust
#[derive(Deserialize)]
struct Pagination {
    page: u32,
    per_page: u32,
}

async fn list_users(Query(pagination): Query<Pagination>) -> Json<Vec<User>> {
    let offset = (pagination.page - 1) * pagination.per_page;
    // Query with LIMIT and OFFSET
    Json(users)
}
```

### Filtering

```rust
#[derive(Deserialize)]
struct UserFilter {
    name: Option<String>,
    min_age: Option<u32>,
    email_domain: Option<String>,
}

async fn search_users(Query(filter): Query<UserFilter>) -> Json<Vec<User>> {
    // Build dynamic query based on filters
    Json(users)
}
```

### API Versioning

```rust
// URL versioning
.route("/api/v1/users", get(v1::list_users))
.route("/api/v2/users", get(v2::list_users))

// Header versioning
async fn versioned_handler(
    TypedHeader(version): TypedHeader<ApiVersion>
) -> Response {
    match version.0.as_str() {
        "v1" => v1::handle().into_response(),
        "v2" => v2::handle().into_response(),
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}
```

## Performance Optimization

### Connection Pooling

```rust
let pool = PgPoolOptions::new()
    .max_connections(100)
    .connect(&db_url)
    .await?;
```

### Caching

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

#[derive(Clone)]
struct Cache {
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl Cache {
    async fn get(&self, key: &str) -> Option<String> {
        self.data.read().await.get(key).cloned()
    }

    async fn set(&self, key: String, value: String) {
        self.data.write().await.insert(key, value);
    }
}
```

### Rate Limiting

```rust
use tower::limit::RateLimitLayer;
use std::time::Duration;

let app = Router::new()
    .route("/api/users", get(list_users))
    .layer(RateLimitLayer::new(100, Duration::from_secs(60)));
```

## Testing

### Integration Tests

```rust
#[tokio::test]
async fn test_create_user() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"Alice"}"#))
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}
```

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_service() {
        let service = UserService::new(mock_db());
        let user = service.create("Alice").await.unwrap();
        assert_eq!(user.name, "Alice");
    }
}
```

## Security Best Practices

1. **Input Validation** - Validate all user input
2. **SQL Injection** - Use parameterized queries
3. **XSS Prevention** - Sanitize output
4. **CSRF Protection** - Use tokens
5. **HTTPS Only** - Force TLS
6. **Rate Limiting** - Prevent abuse
7. **Authentication** - JWT or session-based
8. **Authorization** - Role-based access control

## API Documentation

### OpenAPI/Swagger

```rust
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(
    paths(list_users, create_user),
    components(schemas(User))
)]
struct ApiDoc;

// Serve at /swagger-ui
```

## Deployment

```dockerfile
# Dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/api /usr/local/bin/api
CMD ["api"]
```

```bash
# Docker
docker build -t my-api .
docker run -p 3000:3000 my-api

# Kubernetes
kubectl apply -f deployment.yaml
```

## Monitoring

```rust
use prometheus::{Encoder, TextEncoder, Registry, Counter};

let counter = Counter::new("requests_total", "Total requests").unwrap();
registry.register(Box::new(counter.clone())).unwrap();

// Increment on each request
counter.inc();

// Metrics endpoint
async fn metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    encoder.encode_to_string(&metric_families).unwrap()
}
```

## Resources

- [Axum Documentation](https://docs.rs/axum/)
- [Actix-Web Guide](https://actix.rs/docs/)
- [Rocket Guide](https://rocket.rs/guide/)
- [async-graphql](https://async-graphql.github.io/async-graphql/)
- [REST API Best Practices](https://restfulapi.net/)

## Next Steps

After completing this section:
- Build a complete production API
- Add real-time WebSocket support
- Implement microservices architecture
- Deploy to cloud (AWS, GCP, Azure)

---

**Estimated Time**: 18-24 hours
**Difficulty**: ★★★★☆ (Advanced)
**Prerequisites**: Rust basics, HTTP/REST, async/await
