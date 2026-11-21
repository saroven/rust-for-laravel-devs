# Module 08: Smart Pointers & Shared State

In PHP, all objects are implicitly reference-counted pointers. When you pass an object around (`$order->setCustomer($customer)`), PHP passes an internal pointer handle. If multiple objects reference `$customer`, PHP's garbage collector tracks the reference count.

In Rust, variables have single ownership by default. When you need **shared ownership** or **mutation across multiple owners**, you use **Smart Pointers**.

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

---

## 2. `Box<T>`: Simple Heap Allocation

In Rust, local variables live on the Stack by default. If you have a very large struct or a recursive data structure, you store it on the Heap using `Box::new()`:

```rust
// Recursive type: A binary tree node
enum BinaryTree {
    Leaf(i32),
    Node {
        value: i32,
        left: Box<BinaryTree>,  // Must be Boxed because size is recursive!
        right: Box<BinaryTree>,
    },
}

fn main() {
    let large_number = Box::new(100_000); // Stored on heap
    println!("Value: {}", *large_number); // Dereference with *
} // Deallocated automatically here!
```

---

## 3. `Rc<T>`: Shared Ownership in a Single Thread

What if you need two different structs to reference the same `User`?

```
    Struct A ──┐
               ├──► Rc<User> (Ref Count = 2)
    Struct B ──┘
```

```rust
use std::rc::Rc;

struct Client {
    name: String,
}

struct Trade {
    id: u64,
    client: Rc<Client>, // Shared ownership!
}

fn main() {
    let client = Rc::new(Client { name: String::from("Alice") });
    println!("Ref count: {}", Rc::clone(&client)); // Count = 1

    let trade1 = Trade { id: 1, client: Rc::clone(&client) };
    println!("Ref count: {}", Rc::clone(&client)); // Count = 2

    let trade2 = Trade { id: 2, client: Rc::clone(&client) };
    println!("Ref count: {}", Rc::clone(&client)); // Count = 3
} // When trade1, trade2, and client go out of scope, ref count drops to 0 and memory is freed!
```
*Note: `Rc::clone(&client)` does NOT deep-copy the data; it only increments the integer reference counter (instant O(1) operation).*

---

## 4. `Arc<T>`: The Multithreaded Version of `Rc<T>`

In modern web backends (like Axum), your application runs on multiple threads across CPU cores.  
`Rc<T>` is NOT thread-safe (incrementing its counter across threads could cause race conditions).

**`Arc<T>` (Atomic Reference Counting)** uses atomic CPU instructions to safely increment and decrement the counter across threads:

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let shared_data = Arc::new(String::from("System Config"));

    let data_clone = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        println!("Background thread reading: {data_clone}");
    });

    println!("Main thread reading: {shared_data}");
    handle.join().unwrap();
}
```

---

## 5. Interior Mutability: `Arc<RwLock<T>>` (The Web Backend Pattern)

What if multiple threads need to **read AND write** shared data (e.g. an in-memory cache, active sessions, or a database connection pool)?

In Rust, you combine:
- `Arc<T>` (Provides shared ownership across threads)
- `RwLock<T>` (Provides thread-safe read/write locking)

```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

struct AppState {
    // Shared, thread-safe hash map
    active_users: Arc<RwLock<HashMap<u64, String>>>,
}

fn main() {
    let state = AppState {
        active_users: Arc::new(RwLock::new(HashMap::new())),
    };

    // 1. Write lock (Exclusive access to mutate)
    {
        let mut map = state.active_users.write().unwrap();
        map.insert(101, String::from("Alice"));
    } // Write lock automatically releases here when `map` drops!

    // 2. Read lock (Multiple threads can read concurrently)
    {
        let map = state.active_users.read().unwrap();
        if let Some(user) = map.get(&101) {
            println!("Active user found: {user}");
        }
    } // Read lock automatically releases here!
}
```

> **Why this matters for Laravel devs:**  
> In Laravel, shared state across requests requires external services like Redis or Memcached because PHP-FPM processes don't share memory.  
> In Rust, you can safely store high-speed shared state **in application RAM** with `Arc<RwLock<T>>` with sub-microsecond access!

---

## 🛠️ Exercises: Module 08

1. **Shared Config**:
   Create a struct `Config` with `app_name: String` and `port: u16`. Wrap it in an `Arc<Config>`, spawn two threads with `thread::spawn`, and print the configuration from both threads.
2. **Thread-Safe Counter**:
   Create a counter using `Arc<std::sync::Mutex<u32>>`. Spawn 5 threads where each thread increments the counter by 1. Join all threads and assert that the final value is 5.
