use communication::comm_main_blocking;
use database::Database;
use once_cell::sync::Lazy;
use pow::PowProvider;
use std::sync::Mutex;
use tokio::sync::RwLock;

pub mod communication;
mod database;
mod pow;
pub mod shared;

static DB: Lazy<Mutex<Database>> = Lazy::new(|| {
    println!("Initialising Database");
    let x = Mutex::new(Database::connect());
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
    println!("Starting...");
    let x = DB.lock();
    let y = POW_PROVIDER.read();
    drop(x);
    drop(y);
    comm_main_blocking().await
}
