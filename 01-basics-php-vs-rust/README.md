# Module 01: Language Basics (PHP vs Rust)

In PHP, types are dynamic and checked at runtime. In Rust, types are static and checked at compile time, but Rust uses **type inference**, meaning you don't always have to write the types explicitly.

---

## 1. Variables and Mutability

### In PHP:
```php
<?php
$price = 100;
$price = 150; // Perfectly valid; all variables are reassignable
$price = "one hundred"; // Dynamic typing allows changing the type entirely!
```

### In Rust:
```rust
fn main() {
    let price = 100;
    // price = 150; 
    // ❌ COMPILE ERROR: cannot assign twice to immutable variable `price`

    let mut mutable_price = 100;
    mutable_price = 150; // ✅ OK because of `mut`
    
    // mutable_price = "one hundred"; 
    // ❌ COMPILE ERROR: mismatched types (expected integer, found &str)
}
```

> **Why this matters to a PHP developer:**  
> In PHP, unintended reassignment causes insidious bugs (e.g. overwriting a variable inside a loop). Rust defaults to **immutability**. If a variable shouldn't change, the compiler guarantees it won't.

---

## 2. Variable Shadowing (A Rust Superpower)

In PHP, if you receive a string input and want to cast it to an integer, you often have to invent new variable names:

```php
<?php
// PHP
$userIdInput = "42";
$userId = (int) $userIdInput; // Need a different variable name
```

In Rust, you can **shadow** a variable by redeclaring it with `let`:

```rust
fn main() {
    let user_id = "42";         // &str
    let user_id: u32 = user_id.parse().expect("Not a number"); // u32!
    
    println!("User ID is: {user_id}");
}
```

Shadowing lets you transform a value (and change its type) without inventing dummy names like `user_id_raw`, `user_id_parsed`.

---

## 3. Data Types

Rust is statically typed. Every variable's type must be known at compile time.

### Scalar Types

| Type | Rust Syntax | PHP Equivalent | Description |
|---|---|---|---|
| **Signed Integers** | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | `int` | Can be positive or negative. Default is `i32`. |
| **Unsigned Integers**| `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | `int` (no unsigned in PHP) | Only zero or positive (e.g., IDs, array lengths). `usize` matches pointer width (64-bit). |
| **Floats** | `f32`, `f64` | `float` | `f64` is standard 64-bit float. |
| **Booleans** | `bool` | `bool` | `true` or `false`. No truthy/falsy coercion! (`if (1)` is an error in Rust). |
| **Characters** | `char` | Single-char `string` | 4-byte Unicode scalar (e.g., `'a'`, `'৳'`, `'🦀'`). Note: single quotes! |

> **⚠️ Strict Booleans:** In PHP, `if (1)`, `if ("hello")`, and `if ($count)` work via type juggling. In Rust, conditions **MUST** evaluate to a `bool`. `if count > 0` is required.

---

## 4. Compound Types: Tuples & Arrays

### Tuples
A tuple groups together multiple values of **different types** with a fixed length:

```rust
fn main() {
    // PHP equivalent: [$id, $email, $isActive] = [1, "test@example.com", true];
    let user: (u32, &str, bool) = (1, "test@example.com", true);

    // Access by index
    println!("User ID: {}", user.0);
    println!("Email: {}", user.1);

    // Destructuring (just like PHP [$id, $email] = ...)
    let (id, email, is_active) = user;
    println!("Active: {is_active}");
}
```

### Arrays
In Rust, an array `[T; N]` has a **fixed length** and lives entirely on the **stack**. All elements must be the exact same type:

```rust
fn main() {
    let days: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    println!("First day: {}", days[0]);
    // days[7] -> ❌ Panics at runtime or caught at compile time!
}
```

*(Note: For dynamic-length lists like PHP's `[]`, Rust uses `Vec<T>`, covered in Module 06).*

---

## 5. Statements vs Expressions (The Big PHP Shift)

In PHP, statements (like `if`, `switch`, `for`) perform an action and do not produce a value (though PHP 8 introduced `match` expressions).

In Rust, **almost everything is an expression that evaluates to a value**.

### Expressions return values without `return`:
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // Notice: NO SEMICOLON! This is an expression returning `a + b`.
}
```

> **The Semicolon Rule:**  
> - Without semicolon: It's an **expression** that returns a value.
> - With semicolon (`;`): It's a **statement** that returns the unit type `()`.

### `if` as an Expression (Replacing the Ternary Operator):
In PHP:
```php
<?php
$role = $isAdmin ? "admin" : "client";
```

In Rust:
```rust
let is_admin = true;
let role = if is_admin { "admin" } else { "client" };
```

Both branches must return the **same type**.

---

## 6. Functions

### PHP:
```php
<?php
function calculateDiscount(float $total, float $percent = 0.10): float {
    return $total * (1 - $percent);
}
```

### Rust:
```rust
fn calculate_discount(total: f64, percent: f64) -> f64 {
    total * (1.0 - percent)
}

fn main() {
    let final_price = calculate_discount(100.0, 0.10);
    println!("Discounted: ৳{final_price:.2}");
}
```
*Note: Rust does not have default arguments (`percent = 0.10`). Instead, idiomatic Rust uses the Builder pattern or `Option<f64>`.*

---

## 🛠️ Exercises: Module 01

Try these in a scratch cargo project (`cargo new module01 && cd module01`):

1. **Temperature Converter**: Write a function `celsius_to_fahrenheit(c: f64) -> f64` using the formula `(c * 9.0 / 5.0) + 32.0`. Return the result without using the `return` keyword.
2. **Shadowing Practice**: Create a variable `let input = " 100 ";`, trim whitespace with `.trim()`, and parse it into an integer `u64`, shadowing the variable name `input`.
3. **Control Flow Expression**: Write a function that takes an integer HTTP status code (e.g., `200`, `404`, `500`) and assigns an `&str` message (`"Success"`, `"Not Found"`, etc.) to a variable using an `if / else if / else` expression.
