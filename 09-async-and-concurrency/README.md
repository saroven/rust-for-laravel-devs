# Module 09: Async & Concurrency (Tokio vs PHP-FPM)

Concurrency in PHP and Rust couldn't be more different:
- **PHP's Model**: Synchronous, single-threaded execution per request inside a PHP-FPM worker. Concurrency is handled by spawning dozens of OS processes.
- **Rust's Model**: Asynchronous, non-blocking I/O powered by the **Tokio** runtime. A single process handles tens of thousands of concurrent connections using lightweight tasks.

---

## 1. The Core Mental Shift: Blocking vs Non-Blocking

In PHP:
```php
<?php
// When PHP executes this, the entire OS thread FREEZES and waits for the database.
// No other code can run on this worker until MySQL responds.
$user = DB::table('users')->where('id', 1)->first();
```

In Rust with Tokio:
```rust
// When Rust executes `.await`, the thread does NOT freeze!
// Tokio pauses this task and immediately switches the CPU core to handle other users!
// When the database replies, Tokio resumes this task where it left off.
let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", 1)
    .fetch_one(&pool)
    .await?;
```

---

## 2. What is Tokio?

Rust's standard library does not include an async runtime; it only provides the syntax (`async` and `.await`) and the `Future` trait.  
**Tokio** is the industry standard async runtime for Rust (used by AWS, Cloudflare, Discord, Microsoft).

### Adding Tokio to `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
```

### The `#[tokio::main]` Attribute:
```rust
#[tokio::main]
async fn main() {
    println!("Hello from the async world!");
}
```

---

## 3. Spawning Async Tasks: `tokio::spawn`

In PHP, you cannot spawn a background thread inside a web request without a queue like Redis/RabbitMQ and a separate worker daemon.

In Rust, you can spawn a background task instantly in micro-seconds:

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn send_welcome_email(user_id: u64) {
    sleep(Duration::from_millis(500)).await;
    println!("📧 Welcome email sent to user {user_id}");
}

#[tokio::main]
async fn main() {
    println!("Creating user...");

    // Spawns a lightweight green task in the background (like a Go goroutine)
    tokio::spawn(async {
        send_welcome_email(101).await;
    });

    println!("User created! Continuing immediate HTTP response...");
    sleep(Duration::from_millis(600)).await; // Allow background task to finish for demo
}
```

### OS Threads vs Tokio Tasks:
| Property | OS Thread (`std::thread`) | Tokio Task (`tokio::spawn`) |
|---|---|---|
| **Memory Stack** | ~2 MB | ~2 KB (1,000x smaller!) |
| **Context Switch** | Slow (CPU kernel mode switch) | Extremely fast (user-space work-stealing) |
| **Max Concurrency**| Thousands | Millions |

---

## 4. Concurrent Execution: `tokio::join!` (Parallel Requests)

In PHP, making 3 HTTP API calls sequentially takes `time(A) + time(B) + time(C)` (unless using Guzzle async promises or cURL multi).

In Rust, you execute them concurrently with `tokio::join!`:

```rust
async fn fetch_stock_rate() -> f64 { 285.50 }
async fn fetch_forex_rate() -> f64 { 121.20 }
async fn fetch_client_kyc() -> bool { true }

#[tokio::main]
async fn main() {
    // All three futures execute concurrently on the thread pool!
    let (stock, forex, kyc) = tokio::join!(
        fetch_stock_rate(),
        fetch_forex_rate(),
        fetch_client_kyc()
    );

    println!("Stock: ৳{stock}, USD: ৳{forex}, KYC Verified: {kyc}");
}
```

---

## 5. In-Process Message Queues: `tokio::sync::mpsc`

In Laravel, you use `dispatch(new ProcessAuditLogJob($data))` backed by Redis.  
In Rust, you can use built-in **MPSC (Multi-Producer, Single-Consumer) Channels** for high-speed in-process message queues:

```rust
use tokio::sync::mpsc;

enum AuditEvent {
    ClientLogin { id: u64, ip: String },
    TradeExecuted { trade_id: u64, amount: f64 },
}

#[tokio::main]
async fn main() {
    // Channel with a buffer size of 100 messages
    let (tx, mut rx) = mpsc::channel::<AuditEvent>(100);

    // Spawn the Background Consumer (Like a Laravel Queue Worker)
    tokio::spawn(async move {
        println!("Background audit worker started...");
        while let Some(event) = rx.recv().await {
            match event {
                AuditEvent::ClientLogin { id, ip } => {
                    println!("[AUDIT] User {id} logged in from {ip}");
                }
                AuditEvent::TradeExecuted { trade_id, amount } => {
                    println!("[AUDIT] Trade {trade_id} executed for ৳{amount}");
                }
            }
        }
    });

    // Web Handlers can clone `tx` and dispatch events without blocking
    let tx1 = tx.clone();
    tx1.send(AuditEvent::ClientLogin { id: 1, ip: "192.168.1.1".into() }).await.unwrap();

    let tx2 = tx.clone();
    tx2.send(AuditEvent::TradeExecuted { trade_id: 8821, amount: 50_000.0 }).await.unwrap();

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
}
```

---

## 6. The `Send` and `Sync` Traits: Fearless Concurrency

Why doesn't Rust suffer from race conditions, deadlocks, and threading crashes?  
The compiler enforces two marker traits:

1. **`Send`**: Indicates that ownership of a type can be safely transferred to another thread.
2. **`Sync`**: Indicates that multiple threads can safely read a reference (`&T`) simultaneously.

If you try to send a non-thread-safe type (like `Rc<T>`) across a `tokio::spawn` boundary, **the compiler will refuse to compile your code!**  
You don't need to guess if your concurrency code is safe; if it compiles, it is guaranteed safe from data races.

---

## 🛠️ Exercises: Module 09

1. **Parallel HTTP Ping**:
   Write a program using `tokio::join!` that simulates pinging three services (`auth`, `database`, `payment`) with simulated delays using `tokio::time::sleep`.
2. **Producer-Consumer Queue**:
   Create an MPSC channel that passes integers. Spawn 3 producer tasks that each send 5 numbers, and 1 consumer task that calculates and prints the running sum of all received numbers.
