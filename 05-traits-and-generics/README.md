# Module 05: Traits & Generics (OOP for Rust)

Coming from PHP and Laravel, you are accustomed to classical Object-Oriented Programming (OOP):
- `class Client extends User`
- `abstract class BaseService`
- `interface RepositoryInterface`
- `trait LogsActivity`

**Rust has NO classes, NO inheritance, and NO `extends`.**  
Instead, Rust uses **Traits** and **Generics** to achieve polymorphism, code reuse, and clean contracts.

---

## 1. What is a Trait?

A Trait in Rust is a combination of a **PHP Interface** and a **PHP Trait**:
1. Like an **Interface**: It defines method signatures that a type must implement.
2. Like a **PHP Trait**: It can provide **default method implementations**.

### In PHP:
```php
<?php
interface Auditable {
    public function auditMessage(): string;
}

class TradeOrder implements Auditable {
    public function __construct(public string $symbol, public int $shares) {}

    public function auditMessage(): string {
        return "Order: {$this->shares} shares of {$this->symbol}";
    }
}
```

### In Rust:
```rust
pub trait Auditable {
    // Required method signature
    fn audit_message(&self) -> String;

    // Default method implementation (like a PHP trait method)
    fn print_audit(&self) {
        println!("[AUDIT LOG] {}", self.audit_message());
    }
}

pub struct TradeOrder {
    pub symbol: String,
    pub shares: u32,
}

// Implement the trait for TradeOrder
impl Auditable for TradeOrder {
    fn audit_message(&self) -> String {
        format!("Order: {} shares of {}", self.shares, self.symbol)
    }
}

fn main() {
    let order = TradeOrder {
        symbol: String::from("BEXIMCO"),
        shares: 500,
    };

    order.print_audit(); // Uses the default implementation!
}
```

---

## 2. Implementing Traits for External Types (The Orphan Rule)

In PHP, you cannot add methods or interfaces to PHP's built-in `stdClass`, `string`, or a Composer vendor class without monkey-patching or wrapping it.

In Rust, **you can implement your trait on ANY type**, including standard library types:

```rust
trait Summary {
    fn summarize(&self) -> String;
}

// Implement our custom trait directly on the standard Rust `i32`!
impl Summary for i32 {
    fn summarize(&self) -> String {
        format!("BDT {}.00", self)
    }
}

fn main() {
    let balance = 1500;
    println!("{}", balance.summarize()); // Output: BDT 1500.00
}
```

*(The Orphan Rule: You can implement a trait on a type as long as either the trait OR the type is defined in your current crate).*

---

## 3. Generics: Writing Reusable Code

Generics allow functions, structs, and enums to work with different types without duplicating code.

### Generic Function:
```rust
// T can be any type that implements the PartialOrd trait (comparable)
fn find_largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("Largest int: {}", find_largest(10, 25));
    println!("Largest float: {}", find_largest(3.14, 2.71));
}
```

### Monomorphization: Zero Runtime Cost
In PHP, generic-like constructs or dynamic types incur runtime overhead.  
In Rust, during compilation, Rust performs **monomorphization**: it generates a dedicated, hyper-optimized machine-code copy of `find_largest` for `i32` and another for `f64`. **Generics have zero runtime overhead in Rust.**

---

## 4. Trait Bounds (Type Constraints)

Similar to type hinting an interface in PHP (`function send(Auditable $item)`):

### Syntax 1: Inline bound
```rust
fn record_log<T: Auditable>(item: &T) {
    println!("{}", item.audit_message());
}
```

### Syntax 2: `impl Trait` Shorthand (Idiomatic & Clean)
```rust
fn record_log(item: &impl Auditable) {
    println!("{}", item.audit_message());
}
```

### Syntax 3: `where` Clause (For complex bounds)
```rust
fn export_report<T, U>(item: &T, config: &U) 
where
    T: Auditable + Clone,
    U: std::fmt::Debug,
{
    println!("Config: {:?}", config);
    println!("Item: {}", item.audit_message());
}
```

---

## 5. Static Dispatch (`impl Trait`) vs Dynamic Dispatch (`dyn Trait`)

In PHP, all method calls on objects use dynamic dispatch (looking up the method at runtime).

In Rust, you choose:
1. **Static Dispatch (`impl Trait` or generics)**: The compiler knows the exact type at compile time. Inlined, fastest possible performance.
2. **Dynamic Dispatch (`Box<dyn Trait>` or `&dyn Trait`)**: When you need a heterogeneous collection (e.g. an array of different structs that all implement the same trait, like Laravel's `$notifications = [$emailNotif, $smsNotif]`).

```rust
// Heterogeneous collection using Trait Objects (`dyn Auditable`)
fn main() {
    let items: Vec<Box<dyn Auditable>> = vec![
        Box::new(TradeOrder { symbol: String::from("GP"), shares: 100 }),
        // Any other struct implementing Auditable can be added here!
    ];

    for item in items {
        println!("{}", item.audit_message());
    }
}
```

---

## 6. The Standard Derive Macros

Rust provides powerful built-in traits that you can automatically implement on your structs using `#[derive(...)]`:

```rust
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Account {
    pub id: u64,
    pub boid: String,
    pub balance: f64,
}
```

| Derive | What it gives you |
|---|---|
| `Debug` | Allows printing with `println!("{:?}", acc)` (like `var_dump($acc)`) |
| `Clone` | Gives `.clone()` method to duplicate the instance |
| `PartialEq` | Allows comparison with `==` (`acc1 == acc2`) |
| `Default` | Creates default instance: `Account::default()` (zeros and empty strings) |
| `serde::Serialize` | Converts struct to JSON (via `serde` crate) |
| `serde::Deserialize` | Parses JSON directly into strongly-typed struct |

---

## 🛠️ Exercises: Module 05

1. **Payment Gateway Trait**:
   Define a trait `PaymentGateway` with a method `charge(&self, amount: f64) -> Result<String, String>`.
2. **Implement for Structs**:
   Create two structs `StripeGateway` and `BkashGateway`. Implement `PaymentGateway` for both.
3. **Dispatch Function**:
   Write a function `execute_checkout(gateway: &impl PaymentGateway, amount: f64)` that runs `charge` and logs the result.
