use communication::comm_main_blocking;
use shared_resources::{DB, POW_PROVIDER};

pub mod communication;
mod database;
mod pow;
pub mod shared;
pub mod shared_resources;

pub const ALLOWED_IPS: [&'static str; 1] = [
    "127.0.0.1"
];

#[tokio::main]
pub async fn main() {
    println!("Starting...");
    let x = DB.lock();
    let y = POW_PROVIDER.read();
    drop(x);
    drop(y);
    comm_main_blocking().await
}
