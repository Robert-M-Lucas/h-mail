use color_print::cprintln;
use receiving::comm_main_blocking;
use shared_resources::{DB, POW_PROVIDER};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

mod database;
mod pow;
pub mod receiving;
pub mod sending;
pub mod shared;
pub mod shared_resources;
pub mod config;

#[tokio::main]
pub async fn main() {
    #[cfg(feature = "no_spf")]
    cprintln!("<w,R,bold>SPF verification is disabled - DO NOT USE IN PRODUCTION</>");
    
    #[cfg(feature = "no_salt")]
    cprintln!("<w,R,bold>Using zeroed salt - DO NOT USE IN PRODUCTION</>");

    let shutdown = Arc::new(AtomicBool::new(false));

    let shutdown_clone = shutdown.clone();
    ctrlc::set_handler(move || shutdown_clone.store(true, Ordering::SeqCst))
        .expect("Error setting Ctrl-C handler");

    println!("Starting...");
    let mut db = DB.lock().await;
    db.as_ref().unwrap().create_user("test", "test").ok();
    
    drop(db);
    let pow = POW_PROVIDER.read().await;
    drop(pow);
    
    

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
