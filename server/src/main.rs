use crate::config::args::ARGS;
use crate::config::config_file::CONFIG;
use crate::config::salt::{SALT, generate_salt};
use crate::shared_resources::initialise_shared;
use receiving::recv_main_blocking;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tracing::{info, warn};

pub mod auth_token_provider;
pub mod config;
mod database;
mod pow_provider;
pub mod receiving;
pub mod sending;
pub mod shared_resources;
mod test_user;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let _ = &*ARGS; // Force arg parsing

    if ARGS.generate_salt() {
        let s = generate_salt();
        println!("Salt: {s}");
        return;
    }

    let _ = &*CONFIG; // Force config parsing
    let _ = &*SALT; // Force salt parsing

    if ARGS.no_spf() {
        warn!("SPF verification is disabled - DO NOT USE IN PRODUCTION");
    }

    if ARGS.no_salt() {
        warn!("Using zeroed salt - DO NOT USE IN PRODUCTION");
    }

    if ARGS.no_rate_limit() {
        warn!("No rate limiting - DO NOT USE IN PRODUCTION");
    } else {
        panic!("Rate limiting not implemented. Disable with --no-rate-limit.")
    }

    let shutdown = Arc::new(AtomicBool::new(false));

    let shutdown_clone = shutdown.clone();
    ctrlc::set_handler(move || shutdown_clone.store(true, Ordering::SeqCst))
        .expect("Error setting Ctrl-C handler");

    initialise_shared().await;

    let handle = tokio::spawn(recv_main_blocking());

    loop {
        if shutdown.load(Ordering::Relaxed) {
            info!("Exiting...");
            handle.abort();
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
