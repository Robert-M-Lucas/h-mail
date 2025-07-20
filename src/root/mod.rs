use std::sync::Mutex;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use communication::comm_main_blocking;
use database::Database;
use pow::PowProvider;

mod pow;
mod database;
mod communication;
mod shared;

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
    let _x = DB.lock();
    let _y = POW_PROVIDER.read();
    comm_main_blocking().await
}