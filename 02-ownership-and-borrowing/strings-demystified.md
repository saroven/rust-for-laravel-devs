# Strings Demystified: `String` vs `&str`

In PHP, a string is simple:
```php
<?php
$str = "Hello, world!";
$str .= " Welcome to Laravel";
echo strlen($str);
```

In Rust, you immediately see:
- `String`
- `&str`
- `&String`
- `str`

This is the #1 point of confusion for web developers. Here is the clear, definitive guide to strings in Rust.

---

## 1. The Core Difference: Owned vs Borrowed

| Type | Name | Analogy | Where is it stored? | Modifiable? |
|---|---|---|---|---|
| **`String`** | Owned String | A growable PHP string buffer | Heap (dynamically sized) | Yes (with `mut`) |
| **`&str`** | String Slice | A read-only "window" or view into a string | Borrows from stack, heap, or binary | No (read-only view) |

```
Memory layout:

let my_string = String::from("Laravel");
my_string (on Stack)                     Heap
[ ptr | len: 7 | cap: 7 ] ────────► [ 'L', 'a', 'r', 'a', 'v', 'e', 'l' ]
                                          ▲
let my_slice: &str = &my_string[0..3];    │
my_slice (on Stack)                       │
[ ptr | len: 3 ] ─────────────────────────┘ (Points to "Lar")
```

---

## 2. When to Use Which: The Golden Rules

### Rule 1: In Function Arguments, Always Accept `&str`
```rust
// ❌ INCONVENIENT: Only accepts an owned String!
fn greet_bad(name: &String) {
    println!("Hello, {name}!");
}

// ✅ IDIOMATIC & FLEXIBLE: Accepts BOTH &String and string literals ("text")!
fn greet_good(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let owned = String::from("Tanvir");
    let literal = "World";

    greet_good(&owned);  // Works automatically (Rust coerces &String to &str)
    greet_good(literal); // Works directly
}
```

### Rule 2: In Structs, Start with `String` (Owned)
When defining database models, DTOs, or configuration structs, use `String`:
```rust
struct Client {
    name: String, // Clean, owned data. No lifetime annotations needed!
    email: String,
}
```

---

## 3. Practical String Operations (PHP vs Rust)

### 1. Concatenation & Interpolation
#### In PHP:
```php
<?php
$name = "Alice";
$msg = "Hello, " . $name . "!";
$interpolated = "Hello, {$name}!";
```
#### In Rust:
```rust
let name = "Alice";

// Method A: format! macro (Recommended - clean and readable like PHP)
let msg = format!("Hello, {name}!");

// Method B: In-place append on String
let mut s = String::from("Hello, ");
s.push_str(name);
s.push('!'); // push a single char
```

### 2. Checking if Empty or Length
```rust
let s = "Hello";
println!("Length: {}", s.len());    // Length in bytes
println!("Is empty: {}", s.is_empty());
```

### 3. Converting Between `&str` and `String`
```rust
// &str -> String (Allocates on Heap)
let s1: String = "hello".to_string();
let s2: String = String::from("hello");
let s3: String = "hello".into();

// String -> &str (Zero cost borrowing)
let owned = String::from("hello");
let slice1: &str = &owned;
let slice2: &str = owned.as_str();
```

### 4. Splitting and Substrings
```rust
let email = "admin@quantbd.com";

// Split by character (like explode('@', $email))
let parts: Vec<&str> = email.split('@').collect();
println!("Username: {}, Domain: {}", parts[0], parts[1]);

// Contains (like str_contains($email, "quantbd"))
if email.contains("quantbd") {
    println!("Corporate email verified");
}

// Trim whitespace (like trim($str))
let raw_input = "  CL-1002  \n";
let clean_code = raw_input.trim(); // "CL-1002"
```

---

## 💡 Summary Checklist for Strings

1. Use **`&str`** when you only need to **read** a string.
2. Use **`String`** when you need to **own**, **mutate**, or **store** data in a struct.
3. Use **`format!("Hello, {name}")`** for string interpolation.
4. Calling **`&my_string`** automatically gives you a `&str` slice for free.
