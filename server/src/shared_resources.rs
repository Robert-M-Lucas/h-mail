use crate::args::ARGS;
use crate::auth_token_provider::AuthTokenProvider;
use crate::config::{ACCESS_TOKEN_EXPIRY_MS, REFRESH_TOKEN_EXPIRY_MS, VERIFY_IP_TOKEN_EXPIRY_MS};
use crate::database::{UserId, initialise_db_pool, Db};
use crate::pow_provider::PowProvider;
use once_cell::sync::Lazy;
use tokio::sync::{Mutex, RwLock};

pub async fn initialise_shared() {
    initialise_db_pool();
    if ARGS.test_user() {
        println!("Creating test user");
        Db::create_user("test", "test").ok();
    }

    let pow = POW_PROVIDER.read().await;
    drop(pow);

    let access_token_provider = ACCESS_TOKEN_PROVIDER.read().await;
    drop(access_token_provider);
    let refresh_token_provider = REFRESH_TOKEN_PROVIDER.read().await;
    drop(refresh_token_provider);
    let verify_ip_token_provider = VERIFY_IP_TOKEN_PROVIDER.read().await;
    drop(verify_ip_token_provider);
}

pub static POW_PROVIDER: Lazy<RwLock<PowProvider>> = Lazy::new(|| {
    println!("Initialising POW Provider");
    let x = RwLock::new(PowProvider::new());
    println!("POW Provider initialised");
    x
});

pub static ACCESS_TOKEN_PROVIDER: Lazy<RwLock<AuthTokenProvider<UserId>>> = Lazy::new(|| {
    println!("Initialising Access Token Provider");
    let x = RwLock::new(AuthTokenProvider::new(ACCESS_TOKEN_EXPIRY_MS));
    println!("Access Token Provider initialised");
    x
});

pub static REFRESH_TOKEN_PROVIDER: Lazy<RwLock<AuthTokenProvider<UserId>>> = Lazy::new(|| {
    println!("Initialising Refresh Token Provider");
    let x = RwLock::new(AuthTokenProvider::new(REFRESH_TOKEN_EXPIRY_MS));
    println!("Refresh Token Provider initialised");
    x
});

pub static VERIFY_IP_TOKEN_PROVIDER: Lazy<RwLock<AuthTokenProvider<()>>> = Lazy::new(|| {
    println!("Initialising Verify IP Token Provider");
    let x = RwLock::new(AuthTokenProvider::new(VERIFY_IP_TOKEN_EXPIRY_MS));
    println!("Verify IP Token Provider initialised");
    x
});
