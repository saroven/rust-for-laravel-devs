# Project 01: Artisan-Style CLI Tool in Rust

In Laravel, you write custom Artisan console commands using `php artisan make:command`:
```bash
php artisan account:create --code=CL-100 --deposit=5000
php artisan account:balance CL-100
```

In Rust, the gold standard for building CLIs is **`clap`** (Command Line Argument Parser). It provides automated `--help` generation, validation, and subcommands with zero boilerplate using Rust macros.

---

## 1. Project Setup

Create a new directory or inspect the accompanying source code:

```bash
cargo new artisan_cli
cd artisan_cli
```

### `Cargo.toml`:
```toml
[package]
name = "artisan_cli"
version = "0.1.0"
edition = "2021"

[dependencies]
# Derive macro support gives us declarative CLI definitions
clap = { version = "4.4", features = ["derive"] }
colored = "2.1" # Terminal colors (like Symfony Console style)
```

---

## 2. Source Code (`src/main.rs`)

```rust
use clap::{Parser, Subcommand};
use colored::*;

/// Artisan-style CLI tool built in Rust
#[derive(Parser, Debug)]
#[command(name = "qcrm-cli")]
#[command(author = "qCRM Team <developer@example.com>")]
#[command(version = "1.0")]
#[command(about = "Manages client accounts and financial batches", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new client account (like artisan account:create)
    Create {
        /// Client identifier code (e.g. CL-1002)
        #[arg(short, long)]
        code: String,

        /// Initial deposit balance in BDT
        #[arg(short, long, default_value_t = 0.0)]
        deposit: f64,
    },

    /// Check account balance (like artisan account:balance)
    Balance {
        /// Client identifier code
        code: String,
    },

    /// Run system health inspection
    Inspect {
        /// Enable verbose diagnostic output
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { code, deposit } => {
            if deposit < 0.0 {
                eprintln!("{}", "Error: Deposit cannot be negative!".bold().red());
                std::process::exit(1);
            }

            println!(
                "{} Created client account [{}] with initial deposit ৳{:.2}",
                "SUCCESS:".bold().green(),
                code.bold().yellow(),
                deposit
            );
        }

        Commands::Balance { code } => {
            println!("Querying balance for client: {}", code.bold().cyan());
            // Simulated response
            let simulated_balance = 125_450.75;
            println!("Current Ledger Balance: ৳{:.2}", simulated_balance);
        }

        Commands::Inspect { verbose } => {
            println!("{}", "Running system diagnostic inspection...".bold());
            if verbose {
                println!("- Memory check: OK");
                println!("- Gateway connectivity: OK");
                println!("- Database latency: 0.8ms");
            }
            println!("{}", "All systems operational.".green());
        }
    }
}
```

---

## 3. Running & Testing Your CLI

### 1. View Automated Help:
```bash
cargo run -- --help
```
Output:
```text
Manages client accounts and financial batches

Usage: qcrm-cli <COMMAND>

Commands:
  create   Create a new client account (like artisan account:create)
  balance  Check account balance (like artisan account:balance)
  inspect  Run system health inspection
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### 2. Create an Account:
```bash
cargo run -- create --code CL-9088 --deposit 25000.50
```
Output:
```text
SUCCESS: Created client account [CL-9088] with initial deposit ৳25000.50
```

### 3. Check Balance:
```bash
cargo run -- balance CL-9088
```

### 4. Build a Standalone Fast Native Binary:
```bash
cargo build --release
```
The compiled binary will be placed at `target/release/artisan_cli`.  
You can copy this single binary to `/usr/local/bin` and run it anywhere on Linux without needing PHP, Node, or any runtime installed!
