# Module 12: Testing, Quality & Benchmarking

Laravel developers love testing with **Pest** and **PHPUnit**:
```php
it('calculates balance correctly', function () {
    expect(calculateBalance(100, 50))->toBe(150);
});
```

Rust has a **world-class test suite built directly into the language and Cargo**. You do not need to install third-party packages to write unit, integration, or benchmark tests.

---

## 1. Unit Testing in the Same File (`#[test]`)

In PHP, unit tests live in a separate `tests/Unit/` directory.  
In Rust, **unit tests live in the exact same file as your production code**, placed in an inner module annotated with `#[cfg(test)]`:

```rust
// src/calculator.rs

pub fn add_interest(balance: f64, rate: f64) -> f64 {
    balance * (1.0 + rate)
}

// This module is ONLY compiled when running `cargo test`.
// It adds ZERO bytes to your production release binary!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_interest_correctly() {
        let result = add_interest(1000.0, 0.05);
        assert_eq!(result, 1050.0);
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn it_panics_on_invalid_input() {
        panic!("divide by zero");
    }
}
```

Run tests with:
```bash
cargo test
```

---

## 2. Integration Testing (`tests/` Directory)

Integration tests test your entire library or API from the outside as an external consumer:

```text
my_project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    └── api_integration_test.rs   <-- Integration test file
```

Inside `tests/api_integration_test.rs`:
```rust
use my_project::create_account;

#[test]
fn test_account_creation_workflow() {
    let account = create_account("CL-9988", 5000.0);
    assert_eq!(account.client_code, "CL-9988");
    assert_eq!(account.balance, 5000.0);
}
```

---

## 3. Mocking Dependencies via Traits (Like Mockery in Laravel)

In Laravel, you mock interfaces using Mockery (`$this->mock(PaymentGateway::class)`).  
In Rust, because you code to Traits, you can write mock implementations or use the `mockall` crate:

```rust
pub trait PaymentGateway: Send + Sync {
    fn charge(&self, amount: f64) -> Result<String, String>;
}

// Production Implementation
pub struct BkashGateway;
impl PaymentGateway for BkashGateway {
    fn charge(&self, amount: f64) -> Result<String, String> {
        Ok(format!("Charged ৳{amount} via bKash"))
    }
}

// Test Mock Implementation
pub struct MockPaymentGateway {
    pub should_succeed: bool,
}

impl PaymentGateway for MockPaymentGateway {
    fn charge(&self, amount: f64) -> Result<String, String> {
        if self.should_succeed {
            Ok("TEST_TXN_123".to_string())
        } else {
            Err("Simulated gateway failure".to_string())
        }
    }
}

#[test]
fn test_checkout_service_failure() {
    let mock = MockPaymentGateway { should_succeed: false };
    let result = mock.charge(500.0);
    assert!(result.is_err());
}
```

---

## 4. Benchmarking with `criterion` (Measuring Microsecond Speed)

Want to prove your Rust function is 100x faster than PHP?  
The standard benchmarking crate in Rust is **`criterion`**:

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "portfolio_benchmark"
harness = false
```

Inside `benches/portfolio_benchmark.rs`:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn calculate_portfolio_sum(values: &[f64]) -> f64 {
    values.iter().sum()
}

fn benchmark_sum(c: &mut Criterion) {
    let dataset = vec![150.0; 100_000];

    c.bench_function("sum 100k items", |b| {
        b.iter(|| calculate_portfolio_sum(black_box(&dataset)))
    });
}

criterion_group!(benches, benchmark_sum);
criterion_main!(benches);
```

Run benchmarks:
```bash
cargo bench
```

Output gives nanosecond-level statistical precision and generates interactive HTML graphs in `target/criterion/report/index.html`.
