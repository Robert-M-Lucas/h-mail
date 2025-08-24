use crate::config::config_file::CONFIG;
use crate::database::Db;
use h_mail_interface::interface::fields::big_uint::BigUintField;
use h_mail_interface::interface::fields::hmail_address::HmailAddress;
use h_mail_interface::interface::fields::system_time::SystemTimeField;
use h_mail_interface::interface::hmail::{HmailUser, SendHmailPackage};
use h_mail_interface::interface::pow::{PowClassification, PowHash};
use h_mail_interface::reexports::rsa::rand_core::RngCore;
use rand::thread_rng;
use std::time::SystemTime;
use tracing::info;

pub fn create_test_user() {
    info!("Creating test user");
    if let Err(_) = Db::create_user("test", "test") {
        info!("Test user already exists - not creating sample hmails for test user");
        return;
    }
    let test_user_id = Db::get_user_id_dangerous("test").unwrap();

    // * Whitelist
    Db::add_whitelist(
        test_user_id,
        &HmailAddress::new("minimum#example.com").unwrap(),
        PowClassification::Minimum,
    );
    Db::add_whitelist(
        test_user_id,
        &HmailAddress::new("personal#example.com").unwrap(),
        PowClassification::Personal,
    );

    // * Safe parent
    let parent_hmail: SendHmailPackage = SendHmailPackage::new(
        HmailUser::new(
            HmailAddress::new("example#example.com").unwrap(),
            Some("Test".to_string()),
        ),
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
        "This is the parent h-mail".to_string(),
    );
    let parent_hash = parent_hmail.pow_hash();
    let parent_hmail = parent_hmail.decode().unwrap();
    Db::deliver_hmail(
        "test",
        parent_hmail,
        &parent_hash,
        PowClassification::Minimum,
        Vec::new(),
    )
    .ok();

    let hmail: SendHmailPackage = SendHmailPackage::new(
        HmailUser::new(
            HmailAddress::new("example#example.com").unwrap(),
            Some("Test".to_string()),
        ),
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
        Some(BigUintField::new(&parent_hash)),
        "This is the child h-mail".to_string(),
    );
    let hash = hmail.pow_hash();
    let hmail = hmail.decode().unwrap();
    Db::deliver_hmail(
        "test",
        hmail,
        &hash,
        PowClassification::Minimum,
        Vec::new(),
    )
    .ok();

    // ! Unsafe parent
    let parent_hmail: SendHmailPackage = SendHmailPackage::new(
        HmailUser::new(
            HmailAddress::new("example#example.com").unwrap(),
            Some("Test".to_string()),
        ),
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
        "This is the unsafe parent h-mail".to_string(),
    );
    let parent_hash = parent_hmail.pow_hash();
    let parent_hmail = parent_hmail.decode().unwrap();
    let hmail: SendHmailPackage = SendHmailPackage::new(
        HmailUser::new(
            HmailAddress::new("example#example.com").unwrap(),
            Some("Test".to_string()),
        ),
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
        Some(BigUintField::new(&parent_hash)),
        "This is the child h-mail with an unsafe parent".to_string(),
    );
    let hash = hmail.pow_hash();
    let hmail = hmail.decode().unwrap();
    Db::deliver_hmail(
        "test",
        hmail,
        &hash,
        PowClassification::Minimum,
        vec![(parent_hmail, parent_hash)],
    )
    .ok();
}
