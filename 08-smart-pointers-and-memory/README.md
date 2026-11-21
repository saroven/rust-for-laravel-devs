# Module 08: Smart Pointers, Shared State & Modern Singletons

In PHP, all objects are implicitly reference-counted pointers. When you pass an object around (`$order->setCustomer($customer)`), PHP passes an internal pointer handle.

In Rust, variables have single ownership by default. When you need **shared ownership**, **interior mutability**, or **global singletons**, you use **Smart Pointers**.

---

## 1. Overview of Rust Smart Pointers

| Smart Pointer | Name | Thread-Safe? | Use Case | PHP Analogy |
|---|---|---|---|---|
| `Box<T>` | Box | Yes | Heap allocation with single owner | Standard heap object |
| `Rc<T>` | Reference Counted | No (Single-thread) | Multiple owners reading same data | PHP's internal ref-counting (single process) |
| `Arc<T>` | Atomic Reference Counted | **Yes (Multi-thread)** | Multiple threads sharing same data | Shared memory / Thread-safe object |
| `RefCell<T>` | Reference Cell | No | **Interior Mutability**: mutate data behind immutable ref | Mutating object properties in PHP |
| `Mutex<T>` | Mutual Exclusion Lock | **Yes** | Thread-safe mutable access (one writer at a time) | Mutex / Semaphore |
| `RwLock<T>` | Read-Write Lock | **Yes** | Many readers OR one writer across threads | Read/Write DB lock |
| **`std::sync::LazyLock<T>`** | Lazy Lock (Modern Rust 1.80+) | **Yes** | **Global thread-safe singletons** | **Laravel App Singleton** |

---

## 2. Modern Global Singletons: `std::sync::LazyLock` (Rust 1.80+)

In older Rust tutorials, you will see third-party crates like `lazy_static` or `once_cell` used to create global singletons.

**In modern Rust (1.80+), this is built directly into the standard library with `std::sync::LazyLock`!**

### In Laravel:
```php
<?php
// Registered once in AppServiceProvider:
$this->app->singleton('system_config', fn() => new SystemConfig());
```

### In Modern Rust:
```rust
use std::sync::LazyLock;
use std::collections::HashMap;

// Initialized exactly once on first access, thread-safe, and zero runtime cost after init!
static SUPPORTED_CURRENCIES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("BDT", "Bangladeshi Taka");
    m.insert("USD", "US Dollar");
    m.insert("EUR", "Euro");
    m
});

fn main() {
    // Access directly anywhere in your application across any thread:
    if let Some(name) = SUPPORTED_CURRENCIES.get("BDT") {
        println!("Currency: {name}");
    }
}
```

---

## 3. `Arc<T>`: Shared Ownership Across Threads

In modern async web backends (like Axum), tasks run across multiple CPU cores.  
`Arc<T>` (Atomic Reference Counting) allows multiple tasks to safely share read access to data without cloning the underlying memory:

```rust
use std::sync::Arc;
use tokio::task;

#[tokio::main]
async fn main() {
    let shared_config = Arc::new(String::from("Production Config v2.4"));

    let config_clone = Arc::clone(&shared_config);
    let handle = tokio::spawn(async move {
        println!("Background task reading: {config_clone}");
    });

    println!("Main task reading: {shared_config}");
    handle.await.unwrap();
}
```

---

## 4. Interior Mutability: `Arc<RwLock<T>>` (In-Memory Caches)

What if multiple async requests need to **read AND write** shared memory (e.g. an in-memory session store or stock price cache)?

Combine `Arc<T>` (shared ownership) with `RwLock<T>` (thread-safe locking):

```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

struct AppState {
    // Many threads can read concurrently; writers get exclusive access
    live_tickers: Arc<RwLock<HashMap<String, f64>>>,
}

fn main() {
    let state = AppState {
        live_tickers: Arc::new(RwLock::new(HashMap::new())),
    };

    // 1. Write Lock (Exclusive mutation)
    {
        let mut tickers = state.live_tickers.write().unwrap();
        tickers.insert("GP".to_string(), 285.50);
        tickers.insert("BATBC".to_string(), 518.20);
    } // Write lock automatically drops and unlocks here!

    // 2. Read Lock (Concurrent reads)
    {
        let tickers = state.live_tickers.read().unwrap();
        if let Some(price) = tickers.get("GP") {
            println!("GP Current Price: ৳{price:.2}");
        }
    } // Read lock automatically drops and unlocks here!
}
```

---

## 🛠️ Exercises: Module 08

1. **Global App Metadata**:
   Use `std::sync::LazyLock` to define a global `APP_METADATA` struct containing `version: &'static str` and `environment: &'static str`. Read it from `main()`.
2. **Thread-Safe Cache**:
   Create an `Arc<RwLock<Vec<String>>>`. Spawn 3 Tokio tasks that read the list, and 1 task that appends a new item. Observe that the borrow checker and `RwLock` guarantee thread safety.
3. Check your work in [Comprehensive Exercise Solutions](../exercises-solutions/README.md).
