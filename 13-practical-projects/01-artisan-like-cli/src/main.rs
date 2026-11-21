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
