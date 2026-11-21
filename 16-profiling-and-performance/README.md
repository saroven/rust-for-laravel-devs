# Module 16: Profiling, Optimization & Performance Tuning

What separates a junior Rust developer from an expert is the ability to write code that takes full advantage of modern CPU architecture, cache locality, and zero-cost abstractions.

Here is how to profile, benchmark, and squeeze maximum performance out of your Rust applications.

---

## 1. Compiler Release Profiles (`Cargo.toml`)

By default, `cargo build` compiles in **debug mode** (unoptimized, with debug symbols and runtime bounds checks).  
When compiling for production with `cargo build --release`, you can tune the compiler to hyper-optimize:

### Production Tuning Configuration (`Cargo.toml`):
```toml
[profile.release]
# Optimize for maximum speed (3 is highest)
opt-level = 3

# Link-Time Optimization across all crates (inlines functions across dependencies!)
lto = "fat"

# Reduce parallel code generation units to allow LLVM to optimize across entire binary
codegen-units = 1

# Strip all debug symbols to drastically reduce binary size (e.g., from 40MB to 4MB)
strip = true

# Abort immediately on panic (eliminates unwinding landing pads, saving size & speed)
panic = "abort"
```

---

## 2. CPU Profiling with Flamegraphs

A **Flamegraph** visualizes exactly where your CPU spends time, showing which functions are taking up cycles.

### 1. Install `cargo-flamegraph`:
```bash
cargo install flamegraph
```

### 2. Generate a Flamegraph:
```bash
cargo flamegraph --bin my_service
```
This generates an interactive `flamegraph.svg` file that you can open in any browser.

```
┌─────────────────────────────────────────────────────────────┐
│                      Flamegraph Reading                     │
│  - Wider boxes = More CPU time consumed                     │
│  - Vertical stack = Function call depth                     │
│  - Look for wide plateaus at the top to find bottlenecks!   │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. Memory Profiling & Allocation Tracking

In Rust, stack memory is virtually free (a simple CPU pointer adjustment). **Heap allocations** (`String`, `Vec`, `Box`), however, require OS allocator calls.

### Tip 1: Pre-allocate Vector Capacity (`with_capacity`)
If you know you are inserting 10,000 items into a `Vec` or `HashMap`, always specify capacity upfront to prevent multiple heap reallocations:

```rust
// ❌ SLOW: Re-allocates and copies memory 14 times as it grows:
let mut numbers = Vec::new();
for i in 0..10_000 {
    numbers.push(i);
}

// ✅ FAST: Allocates exactly once on the heap!
let mut numbers = Vec::with_capacity(10_000);
for i in 0..10_000 {
    numbers.push(i);
}
```

### Tip 2: SmallVec / ArrayVec for Stack-Only Performance
If a list almost always contains fewer than 8 items, you can use the `smallvec` crate to keep the entire collection on the **Stack**, avoiding the heap entirely!

---

## 4. Zero-Copy Parsing

In PHP, passing data often involves copying strings or converting JSON into new arrays.  
In Rust, you can use **Zero-Copy Deserialization** with Serde:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct FastPayload<'a> {
    // Instead of allocating a new String, &'a str borrows directly
    // from the incoming JSON string buffer! Zero heap allocation!
    #[serde(borrow)]
    client_code: &'a str,
}
```

---

## 5. Summary: The Expert Checklist

| Technique | Impact |
|---|---|
| `lto = "fat"` | 10%–25% faster runtime execution |
| Pre-allocating `with_capacity` | Prevents CPU-costly heap re-allocations |
| Zero-Copy `&str` parsing | Eliminates string cloning during HTTP requests |
| `strip = true` | 70%–85% reduction in binary file size |
| Flamegraph profiling | Pinpoints the exact lines of code burning CPU cycles |
