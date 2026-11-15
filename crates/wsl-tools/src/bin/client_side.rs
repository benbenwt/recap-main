// This works with the elefant/socket_test/wsl_uds.py from
// https://github.com/elefant-ai/ml-playground/pull/313 which is assumed
// to be running in WSL and listening on the specified Unix domain socket.
use rand::Rng;
use std::env;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{Mutex, mpsc};
use tokio::time;
use wsl_tools::SocatStream;

struct Config {
    socket_path: String,
    rate: u64,
    size: usize,
    duration: u64,
    stats_interval: u64,
}

impl Config {
    fn from_env() -> Self {
        Self {
            socket_path: env::var("SOCKET_PATH").unwrap_or_else(|_| "/tmp/uds.test".to_string()),
            rate: env::var("RATE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            size: env::var("SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1_000_000),
            duration: env::var("DURATION")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
            stats_interval: env::var("STATS_INTERVAL")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
        }
    }
}

struct Statistics {
    total_requests: u64,
    total_bytes_sent: u64,
    total_bytes_received: u64,
    response_times: Vec<Duration>,
    start_time: Instant,
    last_stats_time: Instant,
}

impl Statistics {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            total_requests: 0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            response_times: Vec::new(),
            start_time: now,
            last_stats_time: now,
        }
    }

    fn record_request(&mut self, sent_bytes: u64, received_bytes: u64, response_time: Duration) {
        self.total_requests += 1;
        self.total_bytes_sent += sent_bytes;
        self.total_bytes_received += received_bytes;
        self.response_times.push(response_time);
    }

    fn print_and_reset_interval_stats(&mut self) {
        let now = Instant::now();
        let elapsed_since_last = now.duration_since(self.last_stats_time);
        let elapsed_total = now.duration_since(self.start_time);

        if self.response_times.is_empty() {
            println!("No requests processed in the last interval");
            self.last_stats_time = now;
            return;
        }

        // Calculate response time statistics
        self.response_times.sort();
        let min = self.response_times.first().unwrap();
        let max = self.response_times.last().unwrap();
        let p50 = self.response_times[self.response_times.len() / 2];
        let p99_idx = (self.response_times.len() as f64 * 0.99) as usize;
        let p99 = self.response_times[p99_idx];

        let avg = self.response_times.iter().sum::<Duration>() / self.response_times.len() as u32;

        let interval_reqs = self.response_times.len();
        let reqs_per_sec = interval_reqs as f64 / elapsed_since_last.as_secs_f64();
        let mb_sent = self.total_bytes_sent as f64 / 1_000_000.0;
        let mb_per_sec = mb_sent / elapsed_since_last.as_secs_f64();

        println!(
            "=== Stats over last {:.1}s (total runtime: {:.1}s) ===",
            elapsed_since_last.as_secs_f64(),
            elapsed_total.as_secs_f64()
        );
        println!("Requests: {interval_reqs} ({reqs_per_sec:.2}/sec)");
        println!("Data sent: {mb_sent:.2} MB ({mb_per_sec:.2} MB/s)");
        println!(
            "Response times: min={min:.2?}, avg={avg:.2?}, p50={p50:.2?}, p99={p99:.2?}, max={max:.2?}"
        );
        println!("Total requests since start: {}", self.total_requests);
        println!();

        // Reset interval statistics
        self.response_times.clear();
        self.total_bytes_sent = 0;
        self.total_bytes_received = 0;
        self.last_stats_time = now;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Tokio runtime
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async_main())
}

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting client-side test...");

    let config = Config::from_env();

    let sleep_duration = Duration::from_micros(1_000_000 / config.rate);
    let stats_interval = Duration::from_secs(config.stats_interval);
    let end_time = if config.duration > 0 {
        Some(Instant::now() + Duration::from_secs(config.duration))
    } else {
        None
    };

    println!("Connecting to socket: {}", config.socket_path);
    println!(
        "Sending {} bytes every {:?} ({} per second)",
        config.size, sleep_duration, config.rate
    );

    // Connect to the socket and split the stream
    let stream = SocatStream::connect(&config.socket_path)?;
    let (mut reader, mut writer) = stream.split();

    // Channel for collecting response times
    let (tx, mut rx) = mpsc::channel::<(u64, u64, Duration)>(100);

    // Shared end time signal
    let end_time = Arc::new(Mutex::new(end_time));
    let end_time_reader = Arc::clone(&end_time);

    // Spawn a task to handle reading responses
    let read_task = tokio::spawn(async move {
        loop {
            // Check if test duration has elapsed
            {
                let end_time = end_time_reader.lock().await;
                if let Some(end_time) = *end_time {
                    if Instant::now() > end_time {
                        break;
                    }
                }
            }

            // Read response
            let mut resp = [0u8; 4];
            match reader.read_exact(&mut resp).await {
                Ok(_) => {
                    // In a real implementation, you might track the received time
                    // and use it for more precise timing calculations

                    // Send response size through the channel
                    // The duration is set to zero here since we're tracking it in the sender
                    let _ = tx.send((0, 4, Duration::from_millis(0))).await;
                }
                Err(e) => {
                    eprintln!("Error reading response: {e}");
                    break;
                }
            }
        }
    });

    let mut rng = rand::rng();
    let mut stats = Statistics::new();
    let mut next_stats_time = Instant::now() + stats_interval;

    loop {
        // Check if test duration has elapsed
        {
            let mut end_time_guard = end_time.lock().await;
            if let Some(end_time) = *end_time_guard {
                if Instant::now() > end_time {
                    println!("Test duration completed");
                    *end_time_guard = Some(Instant::now()); // Signal reader to stop
                    break;
                }
            }
        }

        // Generate random payload
        let mut payload = vec![0u8; config.size];
        rng.fill(&mut payload[..]);

        // Prepare length prefix
        let len_prefix = (payload.len() as u32).to_le_bytes();

        // Send request and time it
        let start = Instant::now();
        writer.write_all(&len_prefix).await?;
        writer.write_all(&payload).await?;
        writer.flush().await?;
        // Wait for response from read task
        if let Some((_, received_bytes, _)) = rx.recv().await {
            let response_time = start.elapsed();

            // Update statistics
            stats.record_request(
                (len_prefix.len() + payload.len()) as u64,
                received_bytes,
                response_time,
            );
        }

        // Print stats periodically
        let now = Instant::now();
        if now >= next_stats_time {
            stats.print_and_reset_interval_stats();
            next_stats_time = now + stats_interval;
        }

        // Sleep to maintain the desired rate
        time::sleep(sleep_duration).await;
    }

    // Wait for read task to complete
    let _ = read_task.await;

    // Print final stats
    stats.print_and_reset_interval_stats();

    Ok(())
}
