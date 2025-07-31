use crate::args::ARGS;
use crate::shared_resources::initialise_shared;
use color_print::cprintln;
use dotenvy::dotenv;
use receiving::recv_main_blocking;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

mod args;
pub mod auth_token_provider;
pub mod config;
mod database;
mod pow_provider;
pub mod receiving;
pub mod sending;
pub mod shared_resources;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let _ = &*ARGS; // Force arg parsing

    if ARGS.no_spf() {
        cprintln!("<w,R,bold>SPF verification is disabled - DO NOT USE IN PRODUCTION</>\n");
    }

    if ARGS.no_salt() {
        cprintln!("<w,R,bold>Using zeroed salt - DO NOT USE IN PRODUCTION</>\n");
    } else {
        panic!("Using salt not implemented. Disable with --no-salt.")
    }

    if ARGS.no_rate_limit() {
        cprintln!("<w,R,bold>No rate limiting - DO NOT USE IN PRODUCTION</>\n");
    } else {
        panic!("Rate limiting not implemented. Disable with --no-rate-limit.")
    }

    let shutdown = Arc::new(AtomicBool::new(false));

    let shutdown_clone = shutdown.clone();
    ctrlc::set_handler(move || shutdown_clone.store(true, Ordering::SeqCst))
        .expect("Error setting Ctrl-C handler");

    println!("Starting...");
    initialise_shared().await;

    let handle = tokio::spawn(recv_main_blocking());

    loop {
        if shutdown.load(Ordering::Relaxed) {
            println!("Exiting...");
            handle.abort();
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
