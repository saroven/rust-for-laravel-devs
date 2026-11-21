# Module 06: Collections & Iterators (Laravel Collections vs Rust)

If you love Laravel's `Collection` class (`collect($users)->filter(...)->map(...)->values()`), you are going to feel right at home with **Rust Iterators**.

However, Rust iterators have a massive performance advantage: **they are lazy and compile down to raw assembly loops with zero memory allocations.**

---

## 1. Rust Standard Collections

In PHP, a single type (`array`) is used for lists, associative key-value maps, queues, and stacks.  
In Rust, you select the specific data structure tailored for performance:

| Data Structure | Description | PHP Equivalent |
|---|---|---|
| `Vec<T>` | Contiguous dynamic array on the heap | Indexed array `['a', 'b', 'c']` |
| `HashMap<K, V>` | Hash table with O(1) lookups | Associative array `['id' => 1]` |
| `HashSet<T>` | Set of unique items (no duplicates) | `array_unique($arr)` |
| `BTreeMap<K, V>` | Sorted key-value tree | `ksort($arr)` |

---

## 2. Working with `Vec<T>`

```rust
fn main() {
    // 1. Creation with the vec! macro
    let mut numbers = vec![10, 20, 30];

    // 2. Push elements (like $numbers[] = 40)
    numbers.push(40);
    numbers.push(50);

    // 3. Length and indexing
    println!("Total items: {}", numbers.len());
    println!("First item: {}", numbers[0]);

    // 4. Safe access with .get() -> returns Option<&T>
    match numbers.get(10) {
        Some(val) => println!("Found: {val}"),
        None => println!("Index out of bounds! (No crash!)"),
    }
}
```

---

## 3. Working with `HashMap<K, V>`

```rust
use std::collections::HashMap;

fn main() {
    let mut stock_prices = HashMap::new();

    // Insert keys & values
    stock_prices.insert("GP", 285.50);
    stock_prices.insert("BATBC", 518.20);
    stock_prices.insert("BEXIMCO", 115.60);

    // Lookup
    let symbol = "GP";
    if let Some(price) = stock_prices.get(symbol) {
        println!("{symbol} price: ৳{price:.2}");
    }

    // Entry API (Insert if key doesn't exist - like Laravel $arr[$k] ??= $v)
    stock_prices.entry("SQURPHARMA").or_insert(212.00);
}
```

---

## 4. Laravel Collections vs Rust Iterators

Let's compare a typical Laravel pipeline to its Rust equivalent:

### Laravel Collection:
```php
<?php
$users = [
    ['name' => 'Alice', 'role' => 'client', 'balance' => 5000],
    ['name' => 'Bob',   'role' => 'admin',  'balance' => 0],
    ['name' => 'Charlie','role' => 'client', 'balance' => 12000],
];

$clientNames = collect($users)
    ->where('role', 'client')
    ->filter(fn($u) => $u['balance'] > 1000)
    ->map(fn($u) => strtoupper($u['name']))
    ->values()
    ->all();
```

### Rust Iterator:
```rust
struct User {
    name: String,
    role: String,
    balance: f64,
}

fn main() {
    let users = vec![
        User { name: "Alice".into(), role: "client".into(), balance: 5000.0 },
        User { name: "Bob".into(),   role: "admin".into(),  balance: 0.0 },
        User { name: "Charlie".into(), role: "client".into(), balance: 12000.0 },
    ];

    let client_names: Vec<String> = users
        .iter()
        .filter(|u| u.role == "client" && u.balance > 1000.0)
        .map(|u| u.name.to_uppercase())
        .collect(); // Consumes iterator into a new Vec

    println!("{:?}", client_names); // ["ALICE", "CHARLIE"]
}
```

---

## 5. Why Rust Iterators are Superior: Laziness

In PHP, each chained call (`->filter()`, `->map()`) creates an intermediate array in memory. If you have 100,000 items, Laravel allocates multiple temporary arrays.

In Rust, **iterators are lazy**.  
Calling `.iter().filter(...).map(...)` allocates **zero** memory. It only sets up a pipeline of pointer calculations. No computation occurs until you call a consumer like:
- `.collect()` (collects into a `Vec`, `HashMap`, etc.)
- `.sum()`
- `.count()`
- `.for_each()`
- `for item in iter`

The compiler unrolls this pipeline into a single, blazing-fast CPU loop.

---

## 6. The Three Types of Iteration: `iter()`, `iter_mut()`, `into_iter()`

Understanding which iterator to use based on ownership:

| Method | Yields | Ownership | When to use |
|---|---|---|---|
| `.iter()` | `&T` (reference) | Borrows items read-only | Inspecting, searching, or mapping without destroying original vector |
| `.iter_mut()` | `&mut T` (mutable ref) | Borrows items with write permission | Modifying elements in-place |
| `.into_iter()` | `T` (value) | Consumes / Moves ownership | Transforming or transferring the collection |

### Example of in-place mutation (`iter_mut`):
```rust
let mut balances = vec![100.0, 250.0, 50.0];

// Add 5% interest in-place
for balance in balances.iter_mut() {
    *balance *= 1.05; // Dereference to mutate
}

println!("{:?}", balances); // [105.0, 262.5, 52.5]
```

---

## 🛠️ Exercises: Module 06

1. **Portfolio Calculator**:
   Given a vector of trades:
   ```rust
   struct Trade {
       symbol: String,
       price: f64,
       quantity: u32,
   }
   ```
   Calculate the total portfolio value using `.iter()`, `.map()`, and `.sum::<f64>()`.
2. **Frequency Counter**:
   Given a sentence `"apple banana apple orange banana apple"`, use a `HashMap<&str, u32>` and the `.entry()` API to count the frequency of each word.
3. **Filter and Group**:
   Filter a list of numbers to retain only even numbers, multiply each by 10, and collect into a `Vec<i32>`.
