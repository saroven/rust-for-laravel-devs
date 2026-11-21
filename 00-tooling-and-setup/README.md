# Module 00: Tooling & Setup

In PHP and Laravel, you use `composer` for dependencies, `php -v` for the runtime, `php artisan` for scaffolding, and tools like PHPStan/Pint for quality.

In Rust, **the entire developer experience is unified under one world-class tool: `cargo`**.

---

## 1. What Are `rustup`, `rustc`, and `cargo`?

Understanding the trinity of Rust tools:

```
┌─────────────────────────────────────────────────────────────┐
│                           rustup                            │
│  (The Toolchain Manager - like nvm, phpenv, or pyenv)       │
│  Installs, updates, and switches between Rust versions      │
└───────────────┬─────────────────────────────┬───────────────┘
                │                             │
                ▼                             ▼
       ┌─────────────────┐           ┌─────────────────┐
       │      rustc      │           │      cargo      │
       │ (The Compiler)  │◄──────────┤ (The CLI Orchestrator)
       │ like `gcc` or   │           │ Composer + Artisan +
       │ the PHP Zend VM │           │ PHPUnit + Packagist │
       └─────────────────┘           └─────────────────┘
```

- **`rustup`**: The installer and version manager. It manages versions (stable, beta, nightly) and cross-compilation targets.
- **`rustc`**: The actual Rust compiler. You rarely call it directly; Cargo calls it for you.
- **`cargo`**: The package manager, build tool, test runner, and project manager.

---

## 2. Installing Rust on Linux / Ubuntu

You can run the helper script included in this directory:

```bash
chmod +x install-rust.sh
./install-rust.sh
```

Or run the official installer directly:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

Verify your installation:

```bash
rustc --version
cargo --version
rustup --version
```

### Essential Build Essentials (if not already installed)
Rust compiles to native machine binaries via LLVM, so you need standard C build tools and `gcc`:

```bash
sudo apt update && sudo apt install -y build-essential gcc pkg-config libssl-dev
```

---

## 3. Composer vs Cargo: Command Comparison

If you know Composer and Artisan, you already know 80% of Cargo:

| Task | PHP / Laravel | Rust |
|---|---|---|
| Initialize new project | `composer init` | `cargo init` (or `cargo new my_app`) |
| Initialize a library | `composer init --type=library` | `cargo new --lib my_lib` |
| Manifest file | `composer.json` | `Cargo.toml` (TOML format) |
| Lock file | `composer.lock` | `Cargo.lock` |
| Add a dependency | `composer require serde` | `cargo add serde` |
| Add a dev dependency | `composer require --dev pestphp/pest` | `cargo add --dev tokio-test` |
| Install dependencies | `composer install` | `cargo build` (downloads & compiles) |
| Update dependencies | `composer update` | `cargo update` |
| Run application | `php public/index.php` or `php artisan serve` | `cargo run` |
| Run test suite | `php artisan test` or `vendor/bin/pest` | `cargo test` |
| Fast compile check | `vendor/bin/phpstan analyse` | `cargo check` (super fast, verifies types without generating binary) |
| Format code | `vendor/bin/pint` | `cargo fmt` (uses `rustfmt`) |
| Static analysis / Linting | `vendor/bin/phpstan` / `rector` | `cargo clippy` (official, incredibly intelligent linter) |
| Build for production | `composer install --no-dev --optimize-autoloader` | `cargo build --release` (produces hyper-optimized binary in `target/release/`) |

---

## 4. Anatomy of a Cargo Project

When you run `cargo new hello_world`, Cargo creates:

```text
hello_world/
├── Cargo.toml       <-- Like composer.json (Dependencies, project metadata)
├── Cargo.lock       <-- Like composer.lock (Exact resolved versions)
├── .gitignore       <-- Ignores target/
├── src/
│   └── main.rs      <-- Application entrypoint (like public/index.php)
└── target/          <-- Like vendor/, but also contains compiled binary artifacts
```

### `Cargo.toml` Example:
```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

[dependencies]
# Similar to "require": { "serde": "^1.0" }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
# Similar to "require-dev": { ... }
```

---

## 5. Your First Rust Program: "Hello, World!"

Inside `src/main.rs`:

```rust
fn main() {
    println!("Hello, World!");
}
```

Run it:
```bash
cargo run
```

Output:
```text
   Compiling hello_world v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello_world`
Hello, World!
```

---

## 6. Recommended VS Code / Editor Setup

1. **Extension**: `rust-analyzer` (Official, provides real-time type inference, inline errors, and autocompletion).
   - Disable the legacy `Rust` extension if installed.
2. **Settings**: Enable format on save:
   ```json
   {
       "[rust]": {
           "editor.formatOnSave": true,
           "editor.defaultFormatter": "rust-lang.rust-analyzer"
       }
   }
   ```
3. **Clippy check on save**:
   ```json
   {
       "rust-analyzer.check.command": "clippy"
   }
   ```

---

## 💡 Key Takeaway for Laravel Developers

In Laravel, errors often show up when a user hits the endpoint in production (`500 Server Error` or `TypeError`).  
In Rust, **`cargo check` and `cargo clippy` catch nearly 95% of all runtime bugs before your code ever runs.** 

Once you get used to `cargo clippy`, you will wish PHP had a compiler that was this helpful.
