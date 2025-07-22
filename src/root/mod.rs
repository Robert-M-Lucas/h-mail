use communication::comm_main_blocking;
use database::Database;
use once_cell::sync::Lazy;
use pow::PowProvider;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};

pub mod communication;
mod database;
mod pow;
pub mod shared;

static DB: Lazy<Mutex<Option<Database>>> = Lazy::new(|| {
    println!("Initialising Database");
    let x = Mutex::new(Some(Database::connect()));
    println!("Database initialised");
    x
});

static POW_PROVIDER: Lazy<RwLock<PowProvider>> = Lazy::new(|| {
    println!("Initialising POW Provider");
    let x = RwLock::new(PowProvider::new());
    println!("POW Provider initialised");
    x
});

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
