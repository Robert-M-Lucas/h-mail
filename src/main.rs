use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use crate::communication::comm_main_blocking;
use crate::database::Database;
use crate::pow::PowProvider;

mod pow;
mod database;
mod communication;
mod shared;

static DB: Lazy<Mutex<Database>> = Lazy::new(|| {
    Mutex::new(Database::connect())
});

static POW_PROVIDER: Lazy<RwLock<PowProvider>> = Lazy::new(|| {
    RwLock::new(PowProvider::new())
});

#[tokio::main]
async fn main() {
    comm_main_blocking().await
}