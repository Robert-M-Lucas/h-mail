use crate::config::config_file::CONFIG;
use crate::database::diesel_interface::diesel_structs::NewUser;
use crate::database::diesel_interface::schema::users::dsl as users;
use crate::database::{DB_POOL, Db, get_salt};
use argon2::{Argon2, PasswordHasher};
use diesel::result::{DatabaseErrorKind, Error};
use diesel_async::RunQueryDsl;
use h_mail_interface::interface::routes::native::create_account::CreateAccountResponse;
use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug)]
pub enum CreateAccountFailureReason {
    UsernameInUse,
    BadUsername(String),
    BadPassword(String),
}

impl CreateAccountFailureReason {
    pub fn into_create_account_response(self) -> CreateAccountResponse {
        match self {
            CreateAccountFailureReason::UsernameInUse => CreateAccountResponse::UsernameInUse,
            CreateAccountFailureReason::BadUsername(r) => CreateAccountResponse::BadUsername(r),
            CreateAccountFailureReason::BadPassword(r) => CreateAccountResponse::BadPassword(r),
        }
    }
}

static PASSWORD_REGEX_ENGINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(CONFIG.password_regex()).expect("Invalid password regex"));
static NEGATIVE_USERNAME_REGEX_ENGINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(^[-.]|[-.]$|--|\.\.|-\.|\.-)").unwrap());

pub fn preload_regexes() {
    let _ = PASSWORD_REGEX_ENGINE.is_match("");
    assert!(!NEGATIVE_USERNAME_REGEX_ENGINE.is_match("test"));
}

impl Db {
    pub async fn create_user(
        username: &str,
        password: &str,
    ) -> Result<String, CreateAccountFailureReason> {
        let mut connection = DB_POOL.get().await.unwrap();

        let username = username.to_ascii_lowercase();
        if NEGATIVE_USERNAME_REGEX_ENGINE.is_match(&username)
            || username.len() < 4
            || username.len() > 64
        {
            return Err(CreateAccountFailureReason::BadUsername(
                "Username must be 4 - 64 alphanumeric/dot/dash characters. \
            Dots/dashes may not appear at the start, end, or next to each-other."
                    .to_string(),
            ));
        }
        if !PASSWORD_REGEX_ENGINE.is_match(password) {
            return Err(CreateAccountFailureReason::BadPassword(
                CONFIG.password_requirement_text().to_string(),
            ));
        }

        let salt_string = get_salt();
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt_string)
            .unwrap();

        let new_user = NewUser::new(
            username.to_string(),
            password_hash.to_string(),
            *CONFIG.default_user_pow_policy().minimum() as i32,
            *CONFIG.default_user_pow_policy().accepted() as i32,
            *CONFIG.default_user_pow_policy().personal() as i32,
        );

        let r = diesel::insert_into(users::users)
            .values(&new_user)
            .execute(&mut connection)
            .await;

        match r {
            Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                Err(CreateAccountFailureReason::UsernameInUse)
            }
            Err(e) => panic!("{e:?}"),
            Ok(_) => Ok(username),
        }
    }

    #[allow(dead_code)]
    pub async fn has_user(user: &str) -> bool {
        let mut connection = DB_POOL.get().await.unwrap();
        Self::get_user_id(&mut connection, user).await.is_some()
    }
}
