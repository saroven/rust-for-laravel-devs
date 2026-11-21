# Module 10: Web Backend Architecture (Laravel 11/12 to Axum 0.7+)

In modern **Laravel 11 & 12**, the framework was radically streamlined:
- `app/Http/Kernel.php` was **removed**.
- Middleware and routing are now configured directly in **`bootstrap/app.php`**.
- Native health routes (`/up`).

In Rust, the premier modern web framework is **Axum 0.7+** (built and maintained by the official Tokio team).

---

## 1. Architectural Request Flow Comparison

```
Modern Laravel 11/12:
HTTP Request ──► public/index.php ──► bootstrap/app.php (withRouting, withMiddleware) ──► Controller (FormRequest) ──► Eloquent ──► Response

Modern Axum 0.7+:
HTTP Request ──► Axum Binary (Hyper Engine) ──► Tower Middleware Pipeline ──► Router ──► Handler (Extractors) ──► SQLx ──► IntoResponse
```

---

## 2. Application Setup & Middleware Registration

### Modern Laravel 11/12 (`bootstrap/app.php`):
```php
<?php

use Illuminate\Foundation\Application;
use Illuminate\Foundation\Configuration\Exceptions;
use Illuminate\Foundation\Configuration\Middleware;

return Application::configure(basePath: dirname(__DIR__))
    ->withRouting(
        api: __DIR__.'/../routes/api.php',
        commands: __DIR__.'/../routes/console.php',
        health: '/up',
    )
    ->withMiddleware(function (Middleware $middleware) {
        $middleware->append(CustomAuthMiddleware::class);
    })
    ->withExceptions(function (Exceptions $exceptions) {
        // Exception mapping...
    })
    ->create();
```

### Modern Axum 0.7+ (`src/main.rs`):
```rust
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // 1. Initialize logging
    tracing_subscriber::fmt::init();

    // 2. Initialize shared state (like Laravel Service Container)
    let state = Arc::new(AppState::new().await);

    // 3. Build router with middleware pipeline and state
    let app = Router::new()
        .route("/up", get(|| async { "OK" })) // Built-in health check like Laravel /up
        .route("/api/v1/clients", get(list_clients).post(create_client))
        .layer(TraceLayer::new_for_http())    // Middleware (like Laravel request logging)
        .with_state(state);

    // 4. Modern Axum 0.7+ listener binding (NOT legacy Server::bind)
    let addr = "0.0.0.0:8000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Server running on http://{addr}");

    axum::serve(listener, app).await.unwrap();
}
```

---

## 3. Extractors: Declarative Request Handling

In Laravel, you inject `$request` or a `FormRequest` class, then call `$request->validate()`.  
In Axum, handlers declare **typed extractors** in their arguments. Axum automatically deserializes, parses, and validates them before running your handler:

```rust
use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Deserialize)]
pub struct CreateClientDto {
    pub client_code: String,
    pub boid: String,
    pub initial_deposit: f64,
}

#[derive(Serialize)]
pub struct ClientResponse {
    pub id: u64,
    pub client_code: String,
    pub balance: f64,
}

// Axum Handler:
// - Path: /api/v1/clients/:id
// - Query: ?page=1&limit=20
// - State: Shared AppState
// - Json: Auto-validated JSON body
async fn create_client(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<Pagination>,
    Json(payload): Json<CreateClientDto>,
) -> Result<Json<ClientResponse>, AppError> {
    // If client sent malformed JSON or wrong data types,
    // Axum automatically rejects with 400 Bad Request BEFORE this code runs!

    Ok(Json(ClientResponse {
        id: 101,
        client_code: payload.client_code,
        balance: payload.initial_deposit,
    }))
}
```

---

## 4. Centralized Error Handling: `IntoResponse`

In Laravel 11/12, you configure exception rendering inside `withExceptions` in `bootstrap/app.php`.  
In Axum, you implement the `IntoResponse` trait for your custom error enum:

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized,
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::Internal(err) => {
                eprintln!("[SERVER ERROR] {err}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        let body = Json(json!({
            "success": false,
            "error": message,
        }));

        (status, body).into_response()
    }
}
```

Any handler can now return `Result<Json<T>, AppError>` and errors are automatically transformed into clean JSON HTTP responses with proper status codes!

---

## 5. Next Step: Database & Migrations with SQLx

Proceed to the companion guide:  
👉 **[SQLx Database, Connection Pools & Migrations](database-and-migrations-sqlx.md)**
