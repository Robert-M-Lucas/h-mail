use crate::auth_token_provider::AuthTokenProvider;
use crate::config::args::ARGS;
use crate::config::config_file::CONFIG;
use crate::database::{Db, UserId, initialise_db_pool};
use crate::pow_provider::PowProvider;
use h_mail_interface::interface::fields::hmail_address::HmailAddress;
use h_mail_interface::interface::fields::system_time::SystemTimeField;
use h_mail_interface::interface::hmail::{HmailUser, SendHmailPackage};
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
        Db::add_whitelist(
            test_id,
            &HmailAddress::new("minimum#example.com").unwrap(),
            PowClassification::Minimum,
        );
        Db::add_whitelist(
            test_id,
            &HmailAddress::new("personal#example.com").unwrap(),
            PowClassification::Personal,
        );
        let hmail: SendHmailPackage = SendHmailPackage::new(
            vec![
                HmailUser::new(
                    HmailAddress::from_username_domain("test", &CONFIG.domain).unwrap(),
                    Some("Test".to_string()),
                ),
                HmailUser::new(HmailAddress::new("other#example.com").unwrap(), None),
            ],
            "Test Subject".to_string(),
            SystemTimeField::new(&SystemTime::now()),
            thread_rng().next_u32(),
            Some(HmailUser::new(
                HmailAddress::new("test#example.com").unwrap(),
                Some("Test Sender".to_string()),
            )),
            vec![],
            None,
            lipsum(150),
        );
        let hash = hmail.pow_hash();
        let hmail = hmail.decode().unwrap();
        Db::deliver_hmail(
            "test",
            &HmailUser::new(HmailAddress::new("example#example.com").unwrap(), Some("Test".to_string())),
            hmail,
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
