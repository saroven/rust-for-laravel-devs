# Module 10: Web Backend Architecture (Laravel to Axum)

In this module, we map the entire **Laravel web architecture** directly to **Axum** (Rust's premier web framework built by Tokio).

---

## 1. Architectural Architecture Comparison

```
Laravel Web Request Flow:
Client ──► public/index.php ──► Kernel ──► Middleware Pipeline ──► Route ──► Controller (FormRequest) ──► Eloquent ──► Response

Axum Web Request Flow:
Client ──► Axum Binary (Hyper Engine) ──► Tower Middleware ──► Router ──► Handler Function (Extractors) ──► SQLx ──► IntoResponse
```

---

## 2. Request Routing & Handlers

### In Laravel (`routes/api.php`):
```php
<?php
Route::prefix('v1/clients')->group(function () {
    Route::get('/', [ClientController::class, 'index']);
    Route::post('/', [ClientController::class, 'store']);
    Route::get('/{id}', [ClientController::class, 'show']);
});
```

### In Axum:
```rust
use axum::{
    routing::{get, post},
    Router,
};

pub fn client_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/v1/clients", get(list_clients).post(create_client))
        .route("/api/v1/clients/:id", get(get_client))
}
```

---

## 3. Extractors: The Superpower of Axum

In Laravel, you inject `$request: Request` or a `FormRequest` class, then call `$request->validate()` or `$request->input('name')`.

In Axum, handlers declare **typed extractors** in their arguments. Axum automatically deserializes, parses, and validates them before running your handler:

```rust
use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct FilterParams {
    pub search: Option<String>,
    pub page: Option<u32>,
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
async fn create_client(
    State(state): State<Arc<AppState>>,             // Like Laravel Dependency Injection
    Query(filters): Query<FilterParams>,            // Like $request->query('page')
    Json(payload): Json<CreateClientDto>,           // Like FormRequest (auto-parsed JSON)
) -> Result<Json<ClientResponse>, AppError> {
    // If the client sent invalid JSON or missing fields,
    // Axum automatically returns HTTP 400 or 422 before this function is ever called!

    Ok(Json(ClientResponse {
        id: 1,
        client_code: payload.client_code,
        balance: payload.initial_deposit,
    }))
}
```

---

## 4. Custom Extractors: Replacing FormRequests & Auth Guards

In Laravel, you use Middleware or `$request->user()` to access the authenticated user.  
In Axum, you can create a custom `AuthUser` extractor:

```rust
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

pub struct AuthUser {
    pub user_id: u64,
    pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        match auth_header {
            Some(token) if token == "Bearer secret-token-123" => Ok(AuthUser {
                user_id: 42,
                role: "client".to_string(),
            }),
            _ => Err((StatusCode::UNAUTHORIZED, "Invalid or missing Bearer token")),
        }
    }
}

// Now you can simply declare AuthUser in any handler!
async fn get_my_portfolio(user: AuthUser) -> String {
    format!("Hello User {}, loading your portfolio...", user.user_id)
}
```

---

## 5. Database Access: SQLx vs Eloquent

### Laravel Eloquent:
```php
<?php
// Eloquent Active Record
$clients = Client::where('balance', '>', 1000)
    ->orderBy('created_at', 'desc')
    ->get();
```

### Rust with SQLx:
SQLx provides **compile-time checked SQL**. If you misspell a column name in your SQL query, `cargo build` will fail with a compilation error!

```rust
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow, serde::Serialize)]
pub struct ClientRow {
    pub id: i64,
    pub client_code: String,
    pub boid: String,
    pub balance: f64,
}

pub async fn find_wealthy_clients(pool: &PgPool) -> Result<Vec<ClientRow>, sqlx::Error> {
    let clients = sqlx::query_as!(
        ClientRow,
        r#"
        SELECT id, client_code, boid, balance 
        FROM clients 
        WHERE balance > $1 
        ORDER BY id DESC
        "#,
        1000.0
    )
    .fetch_all(pool)
    .await?;

    Ok(clients)
}
```

---

## 6. Centralized HTTP Error Handling

In Laravel, you configure `app/Exceptions/Handler.php`.  
In Axum, you implement `IntoResponse` for your application error enum:

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
    DatabaseError(sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::DatabaseError(err) => {
                eprintln!("Database error: {:?}", err);
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

Now, any handler can simply return `Result<Json<T>, AppError>` and errors are automatically transformed into clean JSON HTTP responses!
