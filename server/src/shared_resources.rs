use crate::auth_token_provider::AuthTokenProvider;
use crate::config::args::ARGS;
use crate::config::config_file::CONFIG;
use crate::database::{UserId, initialise_db_pool};
use crate::pow_provider::PowProvider;
use crate::test_user::create_test_user;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use tracing::info;

pub async fn initialise_shared() {
    initialise_db_pool().await;
    if ARGS.test_user() {
        create_test_user().await;
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
    let x = RwLock::new(PowProvider::new());
    info!("POW Provider initialised");
    x
});

pub static ACCESS_TOKEN_PROVIDER: Lazy<RwLock<AuthTokenProvider<UserId>>> = Lazy::new(|| {
    let x = RwLock::new(AuthTokenProvider::new(CONFIG.access_token_expiry_ms()));
    info!("Access Token Provider initialised");
    x
});

pub static REFRESH_TOKEN_PROVIDER: Lazy<RwLock<AuthTokenProvider<UserId>>> = Lazy::new(|| {
    let x = RwLock::new(AuthTokenProvider::new(CONFIG.refresh_token_expiry_ms()));
    info!("Refresh Token Provider initialised");
    x
});

pub static VERIFY_IP_TOKEN_PROVIDER: Lazy<RwLock<AuthTokenProvider<()>>> = Lazy::new(|| {
    let x = RwLock::new(AuthTokenProvider::new(CONFIG.verify_ip_token_expiry_ms()));
    info!("Verify IP Token Provider initialised");
    x
});
