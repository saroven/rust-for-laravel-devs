# Module 13: Practical Hands-On Production Projects

To cement your Rust mastery, here are three complete, production-grade projects designed around concepts you use in Laravel every day:

---

## 📦 Project Directory

| Project | Description | What You Learn | Link |
|---|---|---|---|
| **01. Artisan-Style CLI** | Build a command-line tool with subcommands, arguments, and flags | `clap` crate, subcommands, parsing input, terminal formatting, exit codes | [01-artisan-like-cli](../13-practical-projects/01-artisan-like-cli/README.md) |
| **02. JSON REST API Service** | Build a high-performance REST API with Axum | Axum router, Serde JSON, in-memory thread-safe state (`Arc<RwLock>`), error handling, CRUD endpoints | [02-rest-api-service](../13-practical-projects/02-rest-api-service/README.md) |
| **03. Async Job Worker (Queue)** | Concurrent background job processor (Like Laravel Horizon) | Tokio MPSC channels, worker pool, concurrency limits, retries, graceful shutdown | [03-async-job-worker](../13-practical-projects/03-async-job-worker/README.md) |

---

## 🚀 How to Run These Projects

Each project has its own dedicated directory containing:
1. Complete step-by-step documentation explaining every line.
2. A ready-to-build `Cargo.toml` file.
3. Fully implemented, compilable `src/main.rs` source code.

You can `cd` into any project directory and execute:
```bash
# Project 1 (CLI)
cd /home/saroven/Projects/Learnings/rust-forge/13-practical-projects/01-artisan-like-cli
cargo run -- --help

# Project 2 (Web API)
cd /home/saroven/Projects/Learnings/rust-forge/13-practical-projects/02-rest-api-service
cargo run

# Project 3 (Async Queue Worker)
cd /home/saroven/Projects/Learnings/rust-forge/13-practical-projects/03-async-job-worker
cargo run
```
