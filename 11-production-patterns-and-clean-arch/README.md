# Module 11: Production Patterns & Clean Architecture

In enterprise Laravel projects, you use Clean Architecture / Hexagonal Architecture:
- Separation of Controllers, Services, Repositories, DTOs, and Domain Models.
- Structured logging with Monolog / Logstash.
- Graceful shutdown for zero-downtime deployments.
- Docker containers for staging and production.

Here is how to apply those exact production standards in Rust.

---

## 1. Enterprise Project Layout (Hexagonal / Layered)

```text
my_service/
├── Cargo.toml
├── Dockerfile
├── .env.example
├── src/
│   ├── main.rs                   <-- Process entrypoint & graceful shutdown
│   ├── config.rs                 <-- Strongly typed environment config
│   ├── domain/                   <-- Pure business entities & errors (No framework code)
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   └── trade.rs
│   ├── application/              <-- Business logic & Use cases (Services)
│   │   ├── mod.rs
│   │   └── client_service.rs
│   ├── infrastructure/           <-- Database, External APIs, Repositories
│   │   ├── mod.rs
│   │   ├── postgres_client_repo.rs
│   │   └── backoffice_gateway.rs
│   └── transport/                <-- HTTP Axum handlers & DTOs
│       ├── mod.rs
│       ├── routes.rs
│       └── handlers/
│           ├── mod.rs
│           └── client_handler.rs
```

---

## 2. Structured JSON Telemetry: `tracing`

In Laravel, you write `Log::info("User logged in", ['id' => $id])`.  
In Rust, the **`tracing`** ecosystem provides high-performance structured logging and span tracing across async tasks:

```rust
use tracing::{info, warn, error, instrument};

#[instrument(skip(pool), fields(client_code = %code))]
async fn process_deposit(pool: &DbPool, code: &str, amount: f64) -> Result<(), AppError> {
    info!("Starting deposit processing for amount: ৳{amount:.2}");

    if amount <= 0.0 {
        warn!("Rejected invalid deposit amount: {amount}");
        return Err(AppError::BadRequest("Amount must be positive".into()));
    }

    // Database operation...
    info!("Deposit completed successfully");
    Ok(())
}
```

When configured with `tracing-subscriber::fmt().json().init()`, this outputs structured JSON logs ready for Datadog, Grafana Loki, or ELK Stack:

```json
{"timestamp":"2026-11-21T14:49:00Z","level":"INFO","fields":{"client_code":"CL-1002","message":"Starting deposit processing for amount: ৳5000.00"}}
```

---

## 3. Graceful Shutdown (Zero-Downtime Deployments)

When Docker or Kubernetes stops a container, it sends `SIGTERM`. If your server shuts down immediately, active user requests are abruptly terminated with `502 Bad Gateway`.

Axum supports graceful shutdown with one function:

```rust
use tokio::signal;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Graceful shutdown signal received. Finishing in-flight requests...");
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(|| async { "OK" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    println!("Server shutdown complete.");
}
```

---

## 4. Multi-Stage Docker Build (From 1GB to 15MB Image)

A typical PHP-FPM + Nginx Docker image is **300MB – 600MB**.  
A production Rust container is **less than 20MB** because Rust compiles to a standalone binary with zero runtime dependencies.

### Production `Dockerfile`:
```dockerfile
# -----------------------------------------------------------------------------
# Stage 1: Build the Rust binary using official compiler
# -----------------------------------------------------------------------------
FROM rust:1.80-bullseye AS builder

WORKDIR /app

# Cache dependency compilation
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy real source code and build final binary
COPY src ./src
RUN touch src/main.rs && cargo build --release

# -----------------------------------------------------------------------------
# Stage 2: Minimal runtime image (Scratch or Distroless)
# -----------------------------------------------------------------------------
FROM gcr.io/distroless/cc-debian12

WORKDIR /app
COPY --from=builder /app/target/release/my_service /app/my_service

EXPOSE 8080
USER nonroot:nonroot

CMD ["/app/my_service"]
```

### Result:
- Container size: **~18MB**
- Attack surface: **Zero shell, zero package manager, zero interpreter**
- Boot time: **< 10ms**
