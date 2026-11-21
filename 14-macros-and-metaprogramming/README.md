# Module 14: Macros & Metaprogramming (PHP Macros vs Rust)

In Laravel, you use macros to dynamically add methods to classes at runtime:
```php
<?php
// Laravel Runtime Macro
Response::macro('caps', function (string $str) {
    return Response::make(strtoupper($str));
});
```

In Rust, **metaprogramming happens at compile time**.  
Rust macros don't just manipulate strings; they write Rust code for you during compilation with **zero runtime overhead**.

---

## 1. The Two Types of Rust Macros

```
Rust Macros
├── 1. Declarative Macros (`macro_rules!`)
│      Pattern-matching syntax extension (like a supercharged match statement that writes code)
│      Example: `vec![1, 2, 3]`, `println!("Hello")`
│
└── 2. Procedural Macros (Code Generators)
       Runs as a compiler plugin, receives raw Rust tokens and outputs new tokens.
       ├── Derive Macros: `#[derive(Serialize, Deserialize)]`
       ├── Attribute Macros: `#[tokio::main]`, `#[instrument]`
       └── Function-like: `sqlx::query!(...)`
```

---

## 2. Declarative Macros with `macro_rules!`

You have already been using declarative macros every day: `vec![]`, `println!()`, `format!()`. The exclamation mark `!` tells you it is a macro.

### Example: Building a PHP `dd!` (Dump & Die) Macro in Rust

In Laravel, every developer relies on `dd($var)`. Rust has `dbg!(&var)`, but let's build our own `dd!` macro that prints formatted debug info and immediately exits the process:

```rust
#[macro_export]
macro_rules! dd {
    // Match a single expression
    ($val:expr) => {
        println!("{}:{} -> {:#?}", file!(), line!(), $val);
        std::process::exit(1);
    };

    // Match multiple expressions (like dd($a, $b, $c))
    ($($val:expr),+ $(,)?) => {
        println!("Dump at {}:{}", file!(), line!());
        $(
            println!("  => {:#?}", $val);
        )+
        std::process::exit(1);
    };
}

fn main() {
    let client_code = "CL-9081";
    let balance = 150_000.50;

    // Call our custom macro!
    dd!(client_code, balance);

    // This line will NEVER run!
    println!("Continuing execution...");
}
```

### Why Declarative Macros are Safe ("Hygiene"):
In C/C++, macros perform dumb text substitution, which frequently causes variable name collisions and security bugs.  
Rust macros are **hygienic**: variables created inside a macro cannot accidentally shadow or leak into the caller's scope!

---

## 3. Creating a Laravel-like `hashmap!` Macro

Creating HashMaps in standard Rust can be verbose:
```rust
let mut map = HashMap::new();
map.insert("a", 1);
map.insert("b", 2);
```

Let's create a macro that lets us write `hashmap!["a" => 1, "b" => 2]` (just like PHP associative arrays!):

```rust
#[macro_export]
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

fn main() {
    // Clean, PHP-like associative map syntax in Rust!
    let stock_rates = hashmap![
        "GP" => 285.50,
        "BATBC" => 518.20,
        "SQURPHARMA" => 212.00,
    ];

    println!("GP Rate: ৳{}", stock_rates["GP"]);
}
```

---

## 4. Understanding Procedural Macros & Derive

Procedural macros are advanced Rust: they are written as separate compiler crates that read the Abstract Syntax Tree (AST) using the `syn` and `quote` crates.

### When you write:
```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeOrder {
    pub symbol: String,
    pub shares: u32,
}
```

The `Serialize` procedural macro executes **during `cargo build`**, analyzes the fields of `TradeOrder`, and automatically writes hundreds of lines of optimized JSON serialization code behind the scenes!

---

## 🛠️ Exercises: Module 14

1. **Min/Max Macro**:
   Write a declarative macro `max_value!(a, b)` that returns the larger of two values using an expression.
2. **Benchmark Timer Macro**:
   Write a macro `measure_time!(block)` that captures `std::time::Instant::now()`, executes an arbitrary code block `$block:block`, and prints how many microseconds or milliseconds it took.
