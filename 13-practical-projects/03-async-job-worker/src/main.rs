use colored::*;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

// ----------------------------------------------------------------------------
// 1. Job Definitions (Like Laravel Job Classes)
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Job {
    SendEmail {
        to: String,
        subject: String,
    },
    SyncLedger {
        client_code: String,
        amount: f64,
    },
    GeneratePdfReport {
        report_id: u64,
        client_code: String,
    },
}

#[derive(Debug)]
pub struct JobEnvelope {
    pub id: u64,
    pub job: Job,
    pub attempts: u32,
}

// ----------------------------------------------------------------------------
// 2. Job Handler Logic
// ----------------------------------------------------------------------------

async fn execute_job(worker_id: usize, envelope: &JobEnvelope) -> Result<(), String> {
    match &envelope.job {
        Job::SendEmail { to, subject } => {
            println!(
                "[{}] Worker #{} sending email to {} | Subject: '{}'",
                "EMAIL".cyan(),
                worker_id,
                to.bold(),
                subject
            );
            sleep(Duration::from_millis(300)).await;
            Ok(())
        }

        Job::SyncLedger { client_code, amount } => {
            println!(
                "[{}] Worker #{} syncing ledger for {} (৳{:.2})",
                "LEDGER".yellow(),
                worker_id,
                client_code.bold(),
                amount
            );
            sleep(Duration::from_millis(200)).await;
            Ok(())
        }

        Job::GeneratePdfReport { report_id, client_code } => {
            println!(
                "[{}] Worker #{} generating PDF report #{} for {}",
                "PDF".magenta(),
                worker_id,
                report_id,
                client_code.bold()
            );
            // Simulate occasional transient error on first attempt
            if envelope.attempts == 1 && *report_id == 3 {
                return Err(format!("Network timeout connecting to PDF renderer"));
            }
            sleep(Duration::from_millis(400)).await;
            Ok(())
        }
    }
}

// ----------------------------------------------------------------------------
// 3. Worker Daemon Task
// ----------------------------------------------------------------------------

async fn start_worker(
    worker_id: usize,
    receiver: Arc<tokio::sync::Mutex<mpsc::Receiver<JobEnvelope>>>,
    retry_sender: mpsc::Sender<JobEnvelope>,
) {
    loop {
        // Safely acquire next job from channel
        let envelope = {
            let mut rx = receiver.lock().await;
            rx.recv().await
        };

        match envelope {
            Some(mut env) => {
                match execute_job(worker_id, &env).await {
                    Ok(_) => {
                        println!(
                            "  {} Job #{} completed successfully by Worker #{}",
                            "✓ SUCCESS:".bold().green(),
                            env.id,
                            worker_id
                        );
                    }
                    Err(err) => {
                        eprintln!(
                            "  {} Job #{} failed: {}. (Attempt {}/3)",
                            "✗ FAILED:".bold().red(),
                            env.id,
                            err,
                            env.attempts
                        );

                        if env.attempts < 3 {
                            env.attempts += 1;
                            println!("  {} Re-queuing Job #{} for retry...", "↻ RETRY:".yellow(), env.id);
                            let _ = retry_sender.send(env).await;
                        } else {
                            eprintln!("  {} Job #{} exceeded max retries! Moved to Dead Letter Queue.", "💀 DLQ:".red().bold(), env.id);
                        }
                    }
                }
            }
            None => {
                // Channel closed, worker shuts down
                break;
            }
        }
    }
}

// ----------------------------------------------------------------------------
// 4. Main Entrypoint & Queue Dispatcher
// ----------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    println!("{}", "=========================================================".bold());
    println!("{}", "  qCRM High-Throughput Async Job Worker (Rust Queue)     ".bold().green());
    println!("{}", "=========================================================".bold());

    let (tx, rx) = mpsc::channel::<JobEnvelope>(100);
    let shared_rx = Arc::new(tokio::sync::Mutex::new(rx));

    // Spawn 3 Concurrent Worker Tasks
    let worker_count = 3;
    let mut worker_handles = Vec::new();

    for id in 1..=worker_count {
        let rx_clone = Arc::clone(&shared_rx);
        let retry_tx = tx.clone();
        let handle = tokio::spawn(async move {
            start_worker(id, rx_clone, retry_tx).await;
        });
        worker_handles.push(handle);
    }

    println!("Started {} concurrent worker threads.", worker_count);
    println!("Dispatching test batch of queued jobs...\n");

    // Simulate dispatching jobs from HTTP requests
    let demo_jobs = vec![
        Job::SendEmail { to: "investor1@example.com".into(), subject: "Daily Portfolio Summary".into() },
        Job::SyncLedger { client_code: "CL-1001".into(), amount: 75_000.0 },
        Job::GeneratePdfReport { report_id: 3, client_code: "CL-2005".into() }, // Will trigger simulated retry
        Job::SendEmail { to: "admin@example.com".into(), subject: "Security Audit Alert".into() },
        Job::SyncLedger { client_code: "CL-3009".into(), amount: 120_500.0 },
        Job::GeneratePdfReport { report_id: 4, client_code: "CL-1001".into() },
    ];

    for (idx, job) in demo_jobs.into_iter().enumerate() {
        tx.send(JobEnvelope {
            id: (idx as u64) + 1,
            job,
            attempts: 1,
        })
        .await
        .unwrap();
    }

    // Wait a brief moment for all jobs (and retries) to finish processing
    sleep(Duration::from_millis(2500)).await;

    // Drop the sender to close the channel so workers finish
    drop(tx);

    println!("\n{}", "All jobs processed. Worker pool shut down gracefully.".green().bold());
}
