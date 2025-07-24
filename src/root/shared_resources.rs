use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use std::sync::Mutex;
use crate::root::database::Database;
use crate::root::pow::PowProvider;

pub static DB: Lazy<Mutex<Database>> = Lazy::new(|| {
    println!("Initialising Database");
    let x = Mutex::new(Database::connect());
    println!("Database initialised");
    x
});
pub static POW_PROVIDER: Lazy<RwLock<PowProvider>> = Lazy::new(|| {
    println!("Initialising POW Provider");
    let x = RwLock::new(PowProvider::new());
    println!("POW Provider initialised");
    x
});