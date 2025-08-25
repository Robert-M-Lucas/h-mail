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
use lipsum::lipsum;
use tracing::info;

pub async fn create_test_user() {
    info!("Creating test user");
    if Db::create_user("test", "test").await.is_err() {
        info!("Test user already exists");
        return;
    }
    let test_user_id = Db::get_user_id_dangerous("test").await.unwrap();

    // * Whitelist
    Db::add_whitelist(
        test_user_id,
        &HmailAddress::new("minimum#example.com").unwrap(),
        PowClassification::Minimum,
    )
    .await;
    Db::add_whitelist(
        test_user_id,
        &HmailAddress::new("personal#example.com").unwrap(),
        PowClassification::Personal,
    )
    .await;

    // * Big message
    let big_hmail: SendHmailPackage = SendHmailPackage::new(
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
        vec![
            HmailUser::new(HmailAddress::new("otherTwo#example.com").unwrap(), None),
            HmailUser::new(HmailAddress::new("otherThree#example.com").unwrap(), None),
        ],
        None,
        lipsum(1000),
    );
    let big_hash = big_hmail.pow_hash();
    let big_hmail = big_hmail.decode().unwrap();
    Db::deliver_hmail(
        "test",
        big_hmail,
        &big_hash,
        PowClassification::Minimum,
        Vec::new(),
        false
    )
        .await
        .expect("Failed to deliver test h-mail to DB");

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
        false
    )
    .await
    .expect("Failed to deliver test h-mail to DB");

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
    Db::deliver_hmail("test", hmail, &hash, PowClassification::Minimum, Vec::new(), false)
        .await
        .expect("Failed to deliver test h-mail to DB");

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
        false
    )
    .await
    .expect("Failed to deliver test h-mail to DB");
}
