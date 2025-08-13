use crate::auth_token_provider::AuthTokenProvider;
use crate::config::args::ARGS;
use crate::config::config_file::CONFIG;
use crate::database::{Db, UserId, initialise_db_pool};
use crate::pow_provider::PowProvider;
use h_mail_interface::interface::email::{EmailUser, SendEmailPackage};
use h_mail_interface::interface::fields::system_time::SystemTimeField;
use h_mail_interface::interface::pow::{PowClassification, PowHash};
use lipsum::lipsum;
use once_cell::sync::Lazy;
use rand::{RngCore, thread_rng};
use std::time::SystemTime;
use tokio::sync::RwLock;
use tracing::info;

pub async fn initialise_shared() {
    initialise_db_pool();
    if ARGS.test_user() {
        info!("Creating test user");
        Db::create_user("test", "test").ok();
        let test_id = Db::get_user_id_dangerous("test").unwrap();
        Db::add_whitelist(test_id, "minimum@example.com", PowClassification::Minimum);
        Db::add_whitelist(test_id, "personal@example.com", PowClassification::Personal);
        let email: SendEmailPackage = SendEmailPackage::new(
            vec![
                EmailUser::new(format!("test@{}", CONFIG.domain), Some("Test".to_string())),
                EmailUser::new(
                    "other@example.com".to_string(),
                    Some("Other Test".to_string()),
                ),
            ],
            "Test Subject".to_string(),
            SystemTimeField::new(&SystemTime::now()),
            thread_rng().next_u32(),
            Some(EmailUser::new(
                "test@example.com".to_string(),
                Some("Test Sender".to_string()),
            )),
            vec![],
            None,
            lipsum(150),
        );
        let hash = email.pow_hash();
        let email = email.decode().unwrap();
        Db::deliver_email(
            "test",
            "example",
            "example.com",
            email,
            &hash,
            PowClassification::Minimum,
        )
        .ok();
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
