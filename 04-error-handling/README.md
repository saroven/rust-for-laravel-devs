# Module 04: Error Handling (Killing Null & Exceptions)

In PHP and Laravel, error handling is split between:
1. `null` returns (which cause the infamous `Call to a member function on null`).
2. Exceptions (`throw new ModelNotFoundException()`), which can be thrown from anywhere and crash the request if unhandled.

**Rust has no `null`, and no exceptions.**  
Instead, Rust represents failure explicitly using two enum types: `Option<T>` and `Result<T, E>`.

---

## 1. `Option<T>`: Eliminating the Billion-Dollar Mistake

Tony Hoare (the inventor of `null`) famously called it his "billion-dollar mistake". In PHP:

```php
<?php
function findUser(int $id): ?User {
    if ($id === 1) return new User("Alice");
    return null;
}

$user = findUser(99);
echo $user->getName(); // 💥 Fatal Error: Call to a member function getName() on null
```

### In Rust:
The standard library defines `Option<T>` as:
```rust
enum Option<T> {
    Some(T),
    None,
}
```

```rust
struct User {
    name: String,
}

fn find_user(id: u64) -> Option<User> {
    if id == 1 {
        Some(User { name: String::from("Alice") })
    } else {
        None
    }
}

fn main() {
    let user = find_user(99);
    // user.name; 
    // ❌ COMPILE ERROR: no field `name` on type `Option<User>`
}
```

You **cannot** access `name` directly on an `Option<User>`. The compiler **forces** you to check whether it's `Some` or `None`.

### Handling `Option<T>` Elegantly:

#### Approach 1: `match`
```rust
match find_user(1) {
    Some(user) => println!("Found: {}", user.name),
    None => println!("User does not exist"),
}
```

#### Approach 2: Default value with `unwrap_or` (PHP `$val ?? 'default'`)
```rust
let name = find_user(99)
    .map(|u| u.name)
    .unwrap_or_else(|| String::from("Guest"));
```

#### Approach 3: Guard clause with `if let`
```rust
if let Some(user) = find_user(1) {
    println!("Hello, {}!", user.name);
}
```

---

## 2. `Result<T, E>`: Replacing Exceptions

In PHP, a function can throw an exception without declaring it in its type signature:

```php
<?php
// Caller has no idea this can throw without reading source or PHPDoc
function transferMoney(float $amount): void {
    if ($amount > 1000) {
        throw new InsufficientFundsException("Limit exceeded");
    }
}
```

In Rust, all fallible functions must return a `Result<T, E>`:

```rust
enum Result<T, E> {
    Ok(T),  // Operation succeeded with value T
    Err(E), // Operation failed with error E
}
```

### Example:
```rust
#[derive(Debug)]
enum TransferError {
    InsufficientFunds { balance: f64, requested: f64 },
    AccountSuspended,
}

fn transfer_money(balance: f64, amount: f64) -> Result<f64, TransferError> {
    if amount > balance {
        Err(TransferError::InsufficientFunds {
            balance,
            requested: amount,
        })
    } else {
        Ok(balance - amount)
    }
}

fn main() {
    match transfer_money(500.0, 750.0) {
        Ok(new_balance) => println!("Transfer successful! Remaining: ৳{new_balance}"),
        Err(err) => println!("Failed: {:?}", err),
    }
}
```

If you call `transfer_money(...)` and don't inspect the `Result`, the compiler emits a `#[must_use]` warning: `unused Result that must be used`. You cannot accidentally swallow an error!

---

## 3. The `?` (Try) Operator: Propagating Errors

In Laravel controllers, if a service throws an exception, it bubbles up automatically. In Rust, you propagate errors using the `?` operator:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_api_key_from_disk() -> Result<String, io::Error> {
    // If File::open fails, it returns early with Err(e) immediately!
    let mut file = File::open("api_key.txt")?; 

    let mut key = String::new();
    // If read_to_string fails, it returns early with Err(e)!
    file.read_to_string(&mut key)?;

    Ok(key.trim().to_string())
}
```

The `?` operator replaces nested `if / else` or `try / catch` blocks with a single character.

---

## 4. `unwrap()` vs `expect()` vs `?`

When reading Rust code, you will see `.unwrap()`:

```rust
// ⚠️ DANGEROUS IN PRODUCTION:
let file = File::open("config.json").unwrap(); // Panics (crashes) if file doesn't exist!

// Better for prototypes/tests:
let file = File::open("config.json").expect("config.json must exist in root");

// ✅ IDIOMATIC FOR PRODUCTION:
let file = File::open("config.json")?;
```

| Method | Behavior on Failure | When to use |
|---|---|---|
| `?` | Returns `Err` to caller | Everywhere in real application logic |
| `.expect("msg")` | Panics with informative message | In tests or when failure is genuinely impossible |
| `.unwrap()` | Panics with generic message | Rapid prototyping and test scripts only |

---

## 5. Production Error Handling: `anyhow` & `thiserror`

In modern Rust web backends:

1. **`thiserror`**: Used to define clean, typed domain errors (e.g. `AuthError`, `DatabaseError`).
2. **`anyhow`**: Used in applications/services to cleanly bundle any error with context.

```rust
// In Cargo.toml:
// [dependencies]
// anyhow = "1.0"

use anyhow::{Context, Result};

fn load_server_config() -> Result<String> {
    std::fs::read_to_string("config.toml")
        .context("Failed to load server configuration file")
}
```

---

## 🛠️ Exercises: Module 04

1. **Safe Division**:
   Write a function `safe_divide(dividend: f64, divisor: f64) -> Result<f64, String>` that returns `Err("Division by zero".into())` if divisor is 0.0, or `Ok(dividend / divisor)`.
2. **Parsing User Age**:
   Write a function `parse_age(input: &str) -> Result<u8, String>`:
   - Parse input into an integer using `.parse::<u8>()`.
   - If parsing fails, return an error message.
   - If age is `< 18` or `> 120`, return an error indicating invalid age.
   - Otherwise return `Ok(age)`.
