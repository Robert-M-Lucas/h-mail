use crate::root::database::Database;
use crate::root::pow::PowProvider;
use once_cell::sync::Lazy;
use tokio::sync::{Mutex, RwLock};
use crate::root::auth_token::AuthTokenProvider;

pub static DB: Lazy<Mutex<Option<Database>>> = Lazy::new(|| {
    println!("Initialising Database");
    let x = Mutex::new(Some(Database::connect()));
    println!("Database initialised");
    x
});

pub static POW_PROVIDER: Lazy<RwLock<PowProvider>> = Lazy::new(|| {
    println!("Initialising POW Provider");
    let x = RwLock::new(PowProvider::new());
    println!("POW Provider initialised");
    x
});

pub static AUTH_TOKEN_PROVIDER: Lazy<RwLock<AuthTokenProvider>> = Lazy::new(|| {
    println!("Initialising POW Provider");
    let x = RwLock::new(AuthTokenProvider::new());
    println!("POW Provider initialised");
    x
});