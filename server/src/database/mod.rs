use crate::config::args::ARGS;
use crate::config::salt::SALT;

use argon2::password_hash::SaltString;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use h_mail_interface::server_config::MIN_SALT_BYTES;
use once_cell::sync::Lazy;

pub mod auth;
mod diesel_interface;
pub mod hmail;
pub mod pow_policy;
pub mod user;
pub mod user_id;
pub mod whitelist;

pub type UserId = i64;
pub type HmailId = i64;

// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub type DbPool = Pool<AsyncPgConnection>;

static DB_POOL: Lazy<DbPool> = Lazy::new(|| {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
        std::env::var("DATABASE_URL").expect("DATABASE_URL env variable not set"),
    );
    Pool::builder(config)
        .build()
        .expect("Failed to create DB connection pool")
});

pub async fn initialise_db_pool() {
    let _ = DB_POOL.get().await.expect("Couldn't connect to database");
}

fn get_salt() -> SaltString {
    if ARGS.no_salt() {
        SaltString::encode_b64(&[0u8; MIN_SALT_BYTES]).unwrap()
    } else {
        SALT.clone().expect("SECRET_SALT not set")
    }
}

pub struct Db;
