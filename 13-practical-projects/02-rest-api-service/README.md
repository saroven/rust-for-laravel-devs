# Project 02: High-Performance JSON REST API in Axum

In this project, you will build a complete, production-structured JSON REST API for managing client accounts.

It features:
- **Axum 0.7** router and handlers
- **Serde** JSON serialization / deserialization
- **Thread-safe in-memory state** using `Arc<RwLock<HashMap<...>>>` (simulating a database)
- **Validation and proper HTTP status codes** (`200 OK`, `201 Created`, `400 Bad Request`, `404 Not Found`)

---

## 1. Project Configuration (`Cargo.toml`)

```toml
[package]
name = "qcrm_api"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web framework
axum = "0.7"
# Asynchronous runtime
tokio = { version = "1.0", features = ["full"] }
# JSON serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
# Tracing & logging
tracing = "0.1"
tracing-subscriber = "0.3"
```

---

## 2. API Implementation (`src/main.rs`)

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

// ----------------------------------------------------------------------------
// 1. Models & DTOs (Data Transfer Objects)
// ----------------------------------------------------------------------------

#[derive(Debug, Serialize, Clone)]
pub struct Account {
    pub id: u64,
    pub client_code: String,
    pub boid: String,
    pub balance: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub client_code: String,
    pub boid: String,
    pub initial_deposit: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

// ----------------------------------------------------------------------------
// 2. Application State (Thread-Safe Shared Memory)
// ----------------------------------------------------------------------------

type Db = Arc<RwLock<HashMap<u64, Account>>>;

struct AppState {
    db: Db,
}

// ----------------------------------------------------------------------------
// 3. Handlers (Controllers)
// ----------------------------------------------------------------------------

/// GET /api/v1/accounts - List all accounts
async fn list_accounts(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let db = state.db.read().unwrap();
    let accounts: Vec<Account> = db.values().cloned().collect();

    (
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            message: "Accounts retrieved successfully".to_string(),
            data: Some(accounts),
        }),
    )
}

/// GET /api/v1/accounts/:id - Retrieve account by ID
async fn get_account(
    Path(id): Path<u64>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    let db = state.db.read().unwrap();

    if let Some(account) = db.get(&id) {
        Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message: "Account found".to_string(),
                data: Some(account.clone()),
            }),
        ))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                message: format!("Account with ID {id} not found"),
                data: None,
            }),
        ))
    }
}

/// POST /api/v1/accounts - Create a new account
async fn create_account(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAccountRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    // Basic validation
    if payload.client_code.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "client_code is required".to_string(),
                data: None,
            }),
        ));
    }

    let mut db = state.db.write().unwrap();
    let next_id = (db.len() as u64) + 1;

    let account = Account {
        id: next_id,
        client_code: payload.client_code.trim().to_uppercase(),
        boid: payload.boid.trim().to_string(),
        balance: payload.initial_deposit.unwrap_or(0.0),
    };

    db.insert(next_id, account.clone());

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            success: true,
            message: "Account created successfully".to_string(),
            data: Some(account),
        }),
    ))
}

// ----------------------------------------------------------------------------
// 4. Server Entrypoint
// ----------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Initialize state
    let state = Arc::new(AppState {
        db: Arc::new(RwLock::new(HashMap::new())),
    });

    // Seed initial demo data
    {
        let mut db = state.db.write().unwrap();
        db.insert(
            1,
            Account {
                id: 1,
                client_code: "CL-1001".to_string(),
                boid: "1201900012345678".to_string(),
                balance: 50_000.0,
            },
        );
    }

    // Build router
    let app = Router::new()
        .route("/api/v1/accounts", get(list_accounts).post(create_account))
        .route("/api/v1/accounts/:id", get(get_account))
        .with_state(state);

    let addr = "0.0.0.0:8000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Rust API Server listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
```

---

## 3. Testing the Endpoints with cURL

### 1. Start the Server:
```bash
cargo run
```

### 2. List Accounts:
```bash
curl -s http://localhost:8000/api/v1/accounts | jq
```

### 3. Create a New Account:
```bash
curl -s -X POST http://localhost:8000/api/v1/accounts \
  -H "Content-Type: application/json" \
  -d '{"client_code":"CL-2005","boid":"1201900099887766","initial_deposit":75000.0}' | jq
```

### 4. Fetch the Account by ID:
```bash
curl -s http://localhost:8000/api/v1/accounts/2 | jq
```

### 5. Test 404 Not Found:
```bash
curl -s http://localhost:8000/api/v1/accounts/999 | jq
```
Response:
```json
{
  "success": false,
  "message": "Account with ID 999 not found",
  "data": null
}
```
