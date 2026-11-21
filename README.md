# Rust for PHP & Laravel Developers: The Zero-to-Production Guide

[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![Laravel](https://img.shields.io/badge/Laravel-PHP_8+-red?logo=laravel)](https://laravel.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/saroven/rust-for-laravel-devs/pulls)

Welcome to the definitive Rust curriculum designed specifically for **PHP and Laravel developers**.

In PHP, the Zend Engine's garbage collector, dynamic typing, and PHP-FPM's shared-nothing process model abstract away hardware memory, pointers, and threads. In Rust, you write high-level, expressive code that compiles down to raw, zero-overhead machine instructions.

This curriculum bridges the exact mental models you already know from modern PHP 8 and Laravel (Dependency Injection, Service Containers, Eloquent, Queues, FormRequests, Middleware, Pest/PHPUnit) directly to their idiomatic Rust equivalents.

---

## 🗺️ Visual Curriculum Roadmap

```mermaid
flowchart TD
    subgraph Phase1["Phase 1: The Core Mental Shift (Beginner)"]
        M0["00. Tooling & Environment<br/><i>rustup, cargo vs composer/artisan</i>"]
        M1["01. Language Basics<br/><i>Types, mut, expressions vs statements</i>"]
        M2["02. Ownership & Borrowing<br/><i>Stack vs Heap, Move semantics, & vs &mut</i>"]
        M2S["02-B. Borrow Checker Survival Guide<br/><i>Top 5 errors & fixes</i>"]
        M2Str["02-C. Strings Demystified<br/><i>String vs &str</i>"]
        M3["03. Compound Types & Matching<br/><i>Structs, Tagged Unions, match, let-else</i>"]
        M4["04. Error Handling<br/><i>Killing null & exceptions: Option, Result, ?</i>"]
        M5["05. Traits & Generics<br/><i>Interfaces & traits, composition over inheritance</i>"]
        M6["06. Collections & Iterators<br/><i>Vec, HashMap, lazy iterators vs Laravel collections</i>"]
    end

    subgraph Phase2["Phase 2: Mastering Memory & Concurrency (Intermediate)"]
        M7["07. Lifetimes Demystified<br/><i>Borrow scope, 'a, lifetime elision</i>"]
        M8["08. Smart Pointers & Shared State<br/><i>Box, Rc, Arc, RefCell, Mutex, RwLock</i>"]
        M9["09. Async & Concurrency<br/><i>Tokio async vs PHP-FPM, green tasks, mpsc channels</i>"]
    end

    subgraph Phase3["Phase 3: Production Backend Engineering (Advanced)"]
        M10["10. Web Backend: Laravel to Axum<br/><i>Routing, Extractors, State, SQLx vs Eloquent</i>"]
        M10DB["10-B. SQLx Database & Migrations<br/><i>Connection pools, migrations, transactions</i>"]
        M11["11. Clean Architecture & Production<br/><i>Hexagonal layers, tracing, 15MB Docker containers</i>"]
        M12["12. Testing & Quality<br/><i>Pest vs cargo test, integration tests, mocks</i>"]
        M13["13. Practical Production Projects<br/><i>Artisan CLI, Axum REST API, Async Queue Worker</i>"]
    end

    subgraph Phase4["Phase 4: Expert Tier & Polyglot Architecture"]
        M14["14. Macros & Metaprogramming<br/><i>macro_rules!, procedural macros</i>"]
        M15["15. Extending PHP with Rust<br/><i>Writing native PHP extensions with ext-php-rs</i>"]
        M16["16. Profiling & Performance<br/><i>Flamegraphs, zero-copy parsing, compiler tuning</i>"]
    end

    Phase1 --> Phase2 --> Phase3 --> Phase4
```

---

## 📚 Curriculum Table of Contents

### 🟢 Phase 1: The Core Mental Shift (Beginner)
| Module | Topic | Description | Link |
|---|---|---|---|
| **00** | **Tooling & Setup** | `rustup`, `cargo`, `rustc`, `clippy`, Composer vs Cargo commands, IDE setup | [00-tooling-and-setup](00-tooling-and-setup/README.md) |
| **01** | **Basics: PHP vs Rust** | Variables, `mut`, Scalar types, Shadowing, Expressions vs Statements | [01-basics-php-vs-rust](01-basics-php-vs-rust/README.md) |
| **02** | **Ownership & Borrowing** | Stack vs Heap, Move semantics, References `&` and `&mut`, Copy types | [02-ownership-and-borrowing](02-ownership-and-borrowing/README.md) |
| **02-B** | **Survival Guide** | **Top 5 Borrow Checker Errors** for PHP devs and how to fix them | [Borrow Checker Survival Guide](02-ownership-and-borrowing/borrow-checker-survival-guide.md) |
| **02-C** | **Strings Demystified** | `String` vs `&str`, concatenation, formatting, slicing, conversions | [Strings Demystified](02-ownership-and-borrowing/strings-demystified.md) |
| **03** | **Structs & Enums** | DTOs as Structs, Tagged Unions, `match`, `if let`, `let-else` guards | [03-structs-enums-pattern-matching](03-structs-enums-pattern-matching/README.md) |
| **04** | **Error Handling** | Killing `null` with `Option<T>`, Killing Exceptions with `Result<T, E>`, `?` operator | [04-error-handling](04-error-handling/README.md) |
| **05** | **Traits & Generics** | Polymorphism without inheritance, Trait bounds, `impl Trait`, `dyn Trait` | [05-traits-and-generics](05-traits-and-generics/README.md) |
| **06** | **Collections & Iterators**| `Vec`, `HashMap`, Lazy Iterators vs Laravel Collections | [06-collections-and-iterators](06-collections-and-iterators/README.md) |

### 🟡 Phase 2: Mastering Memory & Concurrency (Intermediate)
| Module | Topic | Description | Link |
|---|---|---|---|
| **07** | **Lifetimes Demystified** | References scope, Preventing dangling pointers, Lifetime elision, Struct lifetimes | [07-lifetimes-and-references](07-lifetimes-and-references/README.md) |
| **08** | **Smart Pointers & State** | `Box<T>`, `Rc<T>`, `Arc<T>`, `RefCell<T>`, `Mutex<T>`, `RwLock<T>` | [08-smart-pointers-and-memory](08-smart-pointers-and-memory/README.md) |
| **09** | **Async & Concurrency** | PHP-FPM vs Tokio async event loop, `tokio::spawn`, MPSC channels | [09-async-and-concurrency](09-async-and-concurrency/README.md) |

### 🟠 Phase 3: Production Backend Engineering (Advanced)
| Module | Topic | Description | Link |
|---|---|---|---|
| **10** | **Web Backend (Axum)** | Axum routing, extractors (FormRequest equivalent), State DI, SQLx vs Eloquent | [10-web-backend-laravel-to-axum](10-web-backend-laravel-to-axum/README.md) |
| **10-B** | **SQLx & Migrations** | Connection pooling, compile-time verified queries, migrations, transactions | [SQLx Database Guide](10-web-backend-laravel-to-axum/database-and-migrations-sqlx.md) |
| **11** | **Clean Architecture** | Hexagonal architecture, DTOs, structured logging (`tracing`), multi-stage Docker | [11-production-patterns-and-clean-arch](11-production-patterns-and-clean-arch/README.md) |
| **12** | **Testing & Benchmarks** | Pest vs `cargo test`, unit tests in `src/`, integration tests in `tests/`, mocks | [12-testing-and-benchmarking](12-testing-and-benchmarking/README.md) |
| **13** | **Practical Projects** | Complete code: 1. Artisan CLI, 2. Axum REST API, 3. Async Queue Worker | [13-practical-projects](13-practical-projects/README.md) |

### 🔴 Phase 4: Expert Tier & Polyglot Architecture
| Module | Topic | Description | Link |
|---|---|---|---|
| **14** | **Macros & Metaprogramming** | Declarative macros (`macro_rules!`), building a `dd!` macro, procedural macros | [14-macros-and-metaprogramming](14-macros-and-metaprogramming/README.md) |
| **15** | **Extending PHP with Rust** | Writing native `.so` PHP extensions with `ext-php-rs`, 50x speedups in Laravel | [15-extending-php-with-rust](15-extending-php-with-rust/README.md) |
| **16** | **Profiling & Optimization** | CPU Flamegraphs (`flamegraph`), zero-copy parsing, compiler release tuning | [16-profiling-and-performance](16-profiling-and-performance/README.md) |

---

## ⚡ Essential Companions
- **The Rosetta Stone (PHP to Rust Dictionary)**: [cheat-sheets/php-to-rust-dictionary.md](cheat-sheets/php-to-rust-dictionary.md)
- **Comprehensive Exercise Solutions**: [exercises-solutions/README.md](exercises-solutions/README.md)

---

## 💡 The Top 5 PHP Habits to Unlearn in Rust

1. **Don't search for `null`**: You cannot have an uninitialized or null variable in Rust. Wrap optional data in `Option<T>` (`Some(val)` or `None`).
2. **Don't use `try/catch` for control flow**: In Rust, errors are ordinary data returned via `Result<T, E>`. Use the `?` operator to bubble them up.
3. **Values are MOVED by default, not copied**: In PHP, assigning an array or passing a variable copies it (Copy-on-Write). In Rust, passing a `String` or struct to a function transfers ownership. If you still need it, pass a reference (`&my_var`).
4. **Composition over Inheritance**: There is no `class` and no `extends`. Combine data in a `struct` and implement shared behaviors with `trait`.
5. **No shared-nothing restart**: In PHP-FPM, all memory is wiped after each HTTP request. In Rust, your web server is a single, permanent, multi-threaded binary. Shared caches, connection pools, and in-memory state stay alive across requests.
