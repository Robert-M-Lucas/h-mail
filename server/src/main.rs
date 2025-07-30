use crate::args::ARGS;
use crate::shared_resources::initialise_shared;
use color_print::cprintln;
use receiving::recv_main_blocking;
use shared_resources::DB;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use dotenvy::dotenv;

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

    #[cfg(feature = "no_spf")]
    cprintln!("<w,R,bold>SPF verification is disabled - DO NOT USE IN PRODUCTION</>\n");

    #[cfg(feature = "no_salt")]
    cprintln!("<w,R,bold>Using zeroed salt - DO NOT USE IN PRODUCTION</>\n");

    #[cfg(feature = "no_rate_limit")]
    cprintln!("<w,R,bold>No rate limiting - DO NOT USE IN PRODUCTION</>\n");
    #[cfg(not(feature = "no_rate_limit"))]
    compile_error!("Rate limiting not implemented");

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
            DB.lock().await.take().unwrap().close();
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
