use crate::root::auth_token_provider::AuthTokenProvider;
use crate::root::config::{ACCESS_TOKEN_EXPIRY_MS, REFRESH_TOKEN_EXPIRY_MS};
use crate::root::database::Database;
use crate::root::pow_provider::PowProvider;
use once_cell::sync::Lazy;
use tokio::sync::{Mutex, RwLock};

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

pub static ACCESS_TOKEN_PROVIDER: Lazy<RwLock<AuthTokenProvider>> = Lazy::new(|| {
    println!("Initialising Access Token Provider");
    let x = RwLock::new(AuthTokenProvider::new(ACCESS_TOKEN_EXPIRY_MS));
    println!("Access Token Provider initialised");
    x
});

pub static REFRESH_TOKEN_PROVIDER: Lazy<RwLock<AuthTokenProvider>> = Lazy::new(|| {
    println!("Initialising Refresh Token Provider");
    let x = RwLock::new(AuthTokenProvider::new(REFRESH_TOKEN_EXPIRY_MS));
    println!("Refresh Token Provider initialised");
    x
});
