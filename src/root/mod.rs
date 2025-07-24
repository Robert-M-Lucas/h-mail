use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use receiving::comm_main_blocking;
use shared_resources::{DB, POW_PROVIDER};

pub mod receiving;
mod database;
mod pow;
pub mod shared;
pub mod shared_resources;
pub mod sending;

pub const ALLOWED_IPS: [&'static str; 1] = [
    "127.0.0.1"
];

#[tokio::main]
pub async fn main() {
    let shutdown = Arc::new(AtomicBool::new(false));

    let shutdown_clone = shutdown.clone();
    ctrlc::set_handler(move || shutdown_clone.store(true, Ordering::SeqCst))
        .expect("Error setting Ctrl-C handler");

    println!("Starting...");
    let x = DB.lock().await;
    drop(x);
    let y = POW_PROVIDER.read().await;
    drop(y);

    let handle = tokio::spawn(comm_main_blocking());

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
