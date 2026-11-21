# Project 03: Async Background Job Worker (Laravel Horizon in Rust)

In Laravel, you write queued jobs with `dispatch(new ProcessTradeAuditJob($trade))` processed by `php artisan queue:work` (or managed by Laravel Horizon).

In Rust, you can build a blazing-fast, in-process multi-threaded background job worker pool using **Tokio MPSC Channels** and **async worker tasks**.

---

## 1. What This Project Demonstrates

1. **Job Enums (Algebraic Payloads)**: Different types of jobs carrying strongly-typed payloads.
2. **Producer-Consumer Architecture**: Web request handlers dispatch jobs into a bounded channel without blocking.
3. **Worker Pool**: Spawns $N$ concurrent worker tasks that pull jobs from the shared channel.
4. **Retry & Error Handling**: Simulates failures and retries without crashing the daemon.
5. **Graceful Shutdown**: Ensures all in-flight jobs finish processing before the worker terminates.

---

## 2. Project Setup (`Cargo.toml`)

```toml
[package]
name = "qcrm_worker"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
colored = "2.1"
chrono = "0.4"
```

---

## 3. Running the Project

```bash
cd /home/saroven/Projects/Learnings/rust-forge/13-practical-projects/03-async-job-worker
cargo run
```

### What You Will See:
- 3 concurrent worker threads boot up.
- The producer dispatches a batch of mixed jobs (emails, trade ledger syncs, PDF reports).
- The workers process jobs concurrently across CPU threads.
- You will see colored terminal output tracking job IDs, execution times, and simulated retries.
