# Comprehensive Exercise Solutions & Walkthroughs

This document contains step-by-step solutions and explanations for all exercises across the curriculum.

---

## 📌 Module 01: Language Basics

### Exercise 1: Temperature Converter
```rust
fn celsius_to_fahrenheit(c: f64) -> f64 {
    // Notice: No semicolon and no `return` keyword needed!
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    let boiling = celsius_to_fahrenheit(100.0);
    println!("100°C in Fahrenheit: {boiling}°F"); // 212°F
}
```

### Exercise 2: Shadowing Practice
```rust
fn main() {
    let input = " 100 ";
    // Shadow `input`: trim whitespace and parse to u64
    let input: u64 = input.trim().parse().expect("Valid integer");
    println!("Parsed input: {}", input + 50); // 150
}
```

### Exercise 3: Control Flow Expression
```rust
fn http_status_description(code: u16) -> &'static str {
    // `if / else if / else` as an expression returning &str
    if code == 200 {
        "OK"
    } else if code == 404 {
        "Not Found"
    } else if code >= 500 && code <= 599 {
        "Internal Server Error"
    } else {
        "Unknown Status"
    }
}
```

---

## 📌 Module 02: Ownership & Borrowing

### Exercise 1: Fix the Move
**Problem:** `process_account(account)` took ownership, preventing subsequent use.  
**Solution:** Pass a reference `&str` or `&String`:
```rust
fn process_account(acc: &str) {
    println!("Processing {acc}");
}

fn main() {
    let account = String::from("ACC-10023");
    process_account(&account); // Borrowed!
    println!("Logged account: {account}"); // ✅ Works!
}
```

### Exercise 2: Fix the Borrowing Conflict
**Problem:** Cannot have an immutable reference (`&balance`) and a mutable reference (`&mut balance`) simultaneously.  
**Solution:** Separate the scopes of the references:
```rust
fn main() {
    let mut balance = 500;

    // Use immutable reference first
    let ref1 = &balance;
    println!("Current Balance: {ref1}");

    // ref1 is no longer used, so mutable reference is now safe:
    let ref2 = &mut balance;
    *ref2 += 250;
    println!("Updated Balance: {ref2}");
}
```

---

## 📌 Module 03: Structs & Enums

### Exercise 1 & 2: User Status Enum & Formatter
```rust
enum UserStatus {
    Unverified { email: String },
    Active { last_login_timestamp: u64 },
    Suspended { reason: String, until: u64 },
}

fn format_status(status: &UserStatus) -> String {
    match status {
        UserStatus::Unverified { email } => {
            format!("Pending email verification for: {email}")
        }
        UserStatus::Active { last_login_timestamp } => {
            format!("Active user (Last login: {last_login_timestamp})")
        }
        UserStatus::Suspended { reason, until } => {
            format!("Suspended until timestamp {until}. Reason: {reason}")
        }
    }
}
```

### Exercise 3: Portfolio Struct with `impl`
```rust
struct Portfolio {
    client_id: u64,
    stock_value: f64,
    cash_balance: f64,
}

impl Portfolio {
    pub fn total_asset_value(&self) -> f64 {
        self.stock_value + self.cash_balance
    }
}
```

---

## 📌 Module 04: Error Handling

### Exercise 1: Safe Division
```rust
fn safe_divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err(String::from("Division by zero is not permitted"))
    } else {
        Ok(dividend / divisor)
    }
}
```

### Exercise 2: Parsing User Age
```rust
fn parse_age(input: &str) -> Result<u8, String> {
    // 1. Try parsing
    let age: u8 = input
        .trim()
        .parse()
        .map_err(|_| format!("'{input}' is not a valid number"))?;

    // 2. Validate range
    if age < 18 {
        Err(String::from("Must be at least 18 years old"))
    } else if age > 120 {
        Err(String::from("Age exceeds maximum limit of 120"))
    } else {
        Ok(age)
    }
}
```

---

## 📌 Module 05: Traits & Generics

### Payment Gateway Trait and Implementation:
```rust
pub trait PaymentGateway {
    fn charge(&self, amount: f64) -> Result<String, String>;
}

pub struct StripeGateway;
impl PaymentGateway for StripeGateway {
    fn charge(&self, amount: f64) -> Result<String, String> {
        Ok(format!("Stripe charged ৳{amount:.2}"))
    }
}

pub struct BkashGateway;
impl PaymentGateway for BkashGateway {
    fn charge(&self, amount: f64) -> Result<String, String> {
        Ok(format!("bKash charged ৳{amount:.2}"))
    }
}

// Accepts any type that implements PaymentGateway
pub fn execute_checkout(gateway: &impl PaymentGateway, amount: f64) {
    match gateway.charge(amount) {
        Ok(tx_id) => println!("Success: {tx_id}"),
        Err(e) => eprintln!("Failed: {e}"),
    }
}
```

---

## 📌 Module 06: Collections & Iterators

### Exercise 1: Portfolio Calculator
```rust
struct Trade {
    symbol: String,
    price: f64,
    quantity: u32,
}

fn calculate_total_value(trades: &[Trade]) -> f64 {
    trades
        .iter()
        .map(|t| t.price * (t.quantity as f64))
        .sum()
}
```

### Exercise 2: Word Frequency Counter
```rust
use std::collections::HashMap;

fn main() {
    let sentence = "apple banana apple orange banana apple";
    let mut counts: HashMap<&str, u32> = HashMap::new();

    for word in sentence.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }

    println!("{:#?}", counts);
    // {"apple": 3, "banana": 2, "orange": 1}
}
```
