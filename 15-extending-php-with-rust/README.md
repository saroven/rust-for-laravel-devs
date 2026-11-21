# Module 15: Extending PHP with Rust (Writing Native PHP Extensions)

As a Laravel developer, this is the ultimate superpower:  
**You don't have to rewrite your entire Laravel project in Rust.**

Instead, you can write CPU-heavy, performance-critical business logic in Rust (financial calculations, data parsing, image processing, cryptography), compile it into a native `.so` PHP extension using **`ext-php-rs`**, and call it directly inside your Laravel controllers or jobs!

---

## 1. What is `ext-php-rs`?

`ext-php-rs` is a library that allows you to write C-compatible Zend Engine PHP extensions using **100% pure, safe Rust**.

```
PHP Request ──► Laravel Controller
                     │
                     ▼ Calls native C-speed function!
                my_rust_extension.so (Compiled from Rust)
                     ├── SIMD CPU vectorization
                     ├── 50x–100x faster than PHP
                     └── Zero memory leaks
```

---

## 2. Creating a Native PHP Extension in Rust

### Step 1: Create a Library Project
```bash
cargo new --lib fast_calc
cd fast_calc
```

### Step 2: Configure `Cargo.toml`
Set the `crate-type` to `cdylib` (C dynamic library) and add `ext-php-rs`:

```toml
[package]
name = "fast_calc"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
ext-php-rs = "0.11"
```

### Step 3: Write the Rust Functions (`src/lib.rs`)

```rust
use ext_php_rs::prelude::*;

/// Fast financial compound interest calculator
/// Callable directly in PHP as `fast_compound_interest(principal, rate, years)`
#[php_function]
pub fn fast_compound_interest(principal: f64, rate: f64, years: u32) -> f64 {
    principal * (1.0 + rate).powi(years as i32)
}

/// Batch calculate portfolio values for thousands of trades at native speed
#[php_function]
pub fn fast_portfolio_sum(prices: Vec<f64>, quantities: Vec<u32>) -> f64 {
    prices
        .iter()
        .zip(quantities.iter())
        .map(|(&price, &qty)| price * (qty as f64))
        .sum()
}

// Registers this Rust crate as a PHP extension
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
```

---

## 3. Compiling and Installing in PHP

### 1. Build the Release Dynamic Library:
```bash
cargo build --release
```
This produces `target/release/libfast_calc.so`.

### 2. Copy to PHP Extension Directory:
```bash
# Find your PHP extension directory:
PHP_EXT_DIR=$(php-config --extension-dir)

# Copy the compiled Rust binary:
sudo cp target/release/libfast_calc.so "$PHP_EXT_DIR/fast_calc.so"
```

### 3. Enable in `php.ini`:
Add this line to your `php.ini` (e.g. `/etc/php/8.3/cli/conf.d/20-fast_calc.ini`):
```ini
extension=fast_calc.so
```

### 4. Verify in PHP CLI:
```bash
php -m | grep fast_calc
```
Output:
```text
fast_calc
```

---

## 4. Calling Your Rust Code from Laravel

Inside your Laravel controller or job, call your Rust function just like any native PHP function:

```php
<?php

namespace App\Http\Controllers;

use Illuminate\Http\JsonResponse;

class PortfolioController extends Controller
{
    public function calculate(): JsonResponse
    {
        // 100,000 prices and quantities
        $prices = array_fill(0, 100000, 285.50);
        $quantities = array_fill(0, 100000, 500);

        // Runs in compiled Rust assembly in ~0.2 milliseconds!
        // (Pure PHP would take 40ms–150ms and consume 50MB RAM)
        $total = fast_portfolio_sum($prices, $quantities);

        return response()->json([
            'success' => true,
            'total_portfolio_value' => $total,
        ]);
    }
}
```

---

## 💡 The Best of Both Worlds

With this approach:
- You keep Laravel's rapid development, Blade views, authentication, routing, and ecosystem.
- You offload heavy calculations, PDF parsing, or financial number-crunching to **pure native Rust**!
