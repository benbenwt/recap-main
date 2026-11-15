#[cfg(windows)]
use std::{env, thread, time::Duration};
#[cfg(windows)]
use window_tracking::enumerate_windows;

#[cfg(windows)]
fn main() {
    // Get the interval from the command line arguments
    let args: Vec<String> = env::args().collect();
    let mut interval: u64 = 30;
    if args.len() > 1 {
        interval = args[1]
            .parse()
            .expect("Please provide a valid number for the interval");
    }

    loop {
        let windows = enumerate_windows();
        println!("Enumerated Windows:");
        for window in windows {
            println!("Title: {}", window.title);
        }
        println!("-----------------------------");

        thread::sleep(Duration::from_secs(interval));
    }
}

#[cfg(not(windows))]
fn main() {
    println!("This program is only supported on Windows.");
}
