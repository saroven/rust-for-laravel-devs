# Database Integration & Migrations with SQLx

In Laravel, the database experience is powered by **PDO**, **Eloquent ORM**, and **Artisan migrations** (`php artisan migrate`).

In Rust, the industry standard for production web services is **`sqlx`**.  
SQLx gives you:
1. **Asynchronous non-blocking queries** with Tokio.
2. **Connection Pooling** built-in.
3. **Compile-Time Checked SQL**: SQL queries are verified against your live database schema when you build!
4. **Pure async migrations**.

---

## 1. Setting Up Dependencies in `Cargo.toml`

```toml
[dependencies]
# SQLx with PostgreSQL (or MySQL) and runtime features
sqlx = { version = "0.7", features = ["runtime-tokio", "tls-rustls", "postgres", "chrono", "macros"] }
tokio = { version = "1.0", features = ["full"] }
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
dotenvy = "0.15" # For loading .env file (like vlucas/phpdotenv in Laravel)
```

---

## 2. Creating a Database Connection Pool

In Laravel, the framework opens connections under the hood per request.  
In Rust, you initialize a **Connection Pool** once at server startup and share it across all requests:

```rust
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

pub async fn establish_connection_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(20)                 // Max connections in pool
        .min_connections(5)                  // Keep 5 idle connections ready
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}
```

---

## 3. Database Migrations (Like Artisan Migrations)

SQLx has its own migration system:

### 1. Install SQLx CLI:
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### 2. Create a new migration (like `php artisan make:migration`):
```bash
sqlx migrate add create_clients_table
```
This creates a file in `migrations/<timestamp>_create_clients_table.sql`:

```sql
-- Up Migration
CREATE TABLE IF NOT EXISTS clients (
    id BIGSERIAL PRIMARY KEY,
    client_code VARCHAR(50) NOT NULL UNIQUE,
    boid VARCHAR(16) NOT NULL,
    balance NUMERIC(15, 2) NOT NULL DEFAULT 0.00,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_clients_code ON clients(client_code);
```

### 3. Run Migrations Automatically on Server Boot:
In Laravel, you usually run migrations via CLI before deploying. In Rust, you can also embed and run migrations programmatically inside your `main.rs`:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = establish_connection_pool(&db_url).await?;

    // Runs all pending migrations in migrations/ folder!
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    println!("Database migrations ran successfully!");
    Ok(())
}
```

---

## 4. Querying Data: `FromRow` Mapping (Like Eloquent Hydration)

```rust
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, serde::Serialize)]
pub struct Client {
    pub id: i64,
    pub client_code: String,
    pub boid: String,
    pub balance: f64,
    pub created_at: DateTime<Utc>,
}

// 1. Fetch single row (Like Client::where('id', $id)->first())
pub async fn find_client_by_id(pool: &PgPool, id: i64) -> Result<Option<Client>, sqlx::Error> {
    let client = sqlx::query_as!(
        Client,
        "SELECT id, client_code, boid, balance, created_at FROM clients WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(client)
}

// 2. Fetch list (Like Client::where('balance', '>', 5000)->get())
pub async fn find_active_clients(pool: &PgPool) -> Result<Vec<Client>, sqlx::Error> {
    let clients = sqlx::query_as!(
        Client,
        "SELECT id, client_code, boid, balance, created_at FROM clients WHERE balance > $1 ORDER BY id DESC",
        0.0
    )
    .fetch_all(pool)
    .await?;

    Ok(clients)
}
```

---

## 5. Database Transactions: `DB::transaction()` Equivalent

In Laravel:
```php
<?php
DB::transaction(function () use ($senderId, $receiverId, $amount) {
    // Deduct sender
    // Credit receiver
});
```

In Rust with SQLx:
```rust
pub async fn transfer_funds(
    pool: &PgPool,
    from_client_id: i64,
    to_client_id: i64,
    amount: f64,
) -> Result<(), sqlx::Error> {
    // 1. Begin transaction
    let mut tx = pool.begin().await?;

    // 2. Deduct from sender
    sqlx::query!(
        "UPDATE clients SET balance = balance - $1 WHERE id = $2",
        amount,
        from_client_id
    )
    .execute(&mut *tx)
    .await?;

    // 3. Credit receiver
    sqlx::query!(
        "UPDATE clients SET balance = balance + $1 WHERE id = $2",
        amount,
        to_client_id
    )
    .execute(&mut *tx)
    .await?;

    // 4. Commit transaction (If this is not called or an error occurs above,
    // the transaction automatically ROLLS BACK when `tx` drops out of scope!)
    tx.commit().await?;

    Ok(())
}
```

> **The Safe Transaction Guarantee:**  
> In Rust, if any query fails with `?`, the function returns early. When `tx` goes out of scope, SQLx **automatically rolls back the transaction**. You can never leave a dangling, uncommitted transaction!
