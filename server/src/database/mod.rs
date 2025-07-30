use crate::config::DEFAULT_USER_POW_POLICY;
use crate::database::diesel_structs::{NewEmail, NewUser};
use crate::database::schema::Emails::dsl as Emails;
use crate::database::schema::Users::dsl as Users;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::{DatabaseErrorKind, Error};
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use h_mail_interface::interface::email::EmailContents;
use h_mail_interface::interface::pow::{PowClassification, PowPolicy};
use h_mail_interface::interface::routes::native::get_emails::GetEmailsEmail;
use once_cell::sync::Lazy;
use rusqlite::Connection as RusqliteConnection;

mod diesel_structs;
mod schema;

pub type UserId = i32;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

static DB_POOL: Lazy<DbPool> = Lazy::new(|| {
    RusqliteConnection::open("data.sqlite")
        .unwrap()
        .close()
        .unwrap(); // Ensure database exists
    let mut connection = SqliteConnection::establish("sqlite://data.sqlite").unwrap();
    connection.run_pending_migrations(MIGRATIONS).unwrap();
    let manager = ConnectionManager::<SqliteConnection>::new("sqlite://data.sqlite");
    Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool")
});

pub fn initialise_db_pool() {
    DB_POOL.get().unwrap();
}

fn get_salt() -> SaltString {
    #[cfg(feature = "no_salt")]
    let salt = [0u8; 8];
    #[cfg(not(feature = "no_salt"))]
    compile_error!("no_salt feature must be enabled. Salt functionality not implemented");

    SaltString::encode_b64(&salt).unwrap()
}

pub struct Db;

impl Db {
    pub fn create_user(username: &str, password: &str) -> Result<(), ()> {
        let mut connection = DB_POOL.get().unwrap();

        let salt_string = get_salt();
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt_string)
            .unwrap();

        let new_user = NewUser::new(
            username.to_string(),
            password_hash.to_string(),
            DEFAULT_USER_POW_POLICY.minimum() as i32,
            DEFAULT_USER_POW_POLICY.accepted() as i32,
            DEFAULT_USER_POW_POLICY.personal() as i32,
        );

        let r = diesel::insert_into(Users::Users)
            .values(&new_user)
            .execute(&mut connection);

        match r {
            Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Err(()),
            Err(e) => Err(e).unwrap(),
            Ok(_) => Ok(()),
        }
    }

    pub fn authenticate(username: &str, password: &str) -> Result<UserId, ()> {
        let mut connection = DB_POOL.get().unwrap();

        let salt_string = get_salt();
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt_string)
            .unwrap();

        let user_result: UserId = Users::Users
            .filter(Users::username.eq(username))
            .filter(Users::password_hash.eq(password_hash.to_string()))
            .select(Users::user_id)
            .first(&mut connection)
            .map_err(|_| ())?;

        Ok(user_result)
    }

    pub fn has_user(user: &str) -> bool {
        let mut connection = DB_POOL.get().unwrap();
        Users::Users
            .filter(Users::username.eq(user))
            .select(Users::user_id)
            .limit(1)
            .first::<UserId>(&mut connection)
            .optional()
            .unwrap()
            .is_some()
    }

    pub fn get_username_from_id(id: UserId) -> Option<String> {
        let mut connection = DB_POOL.get().unwrap();

        Users::Users
            .filter(Users::user_id.eq(id))
            .select(Users::username)
            .first::<String>(&mut connection)
            .optional()
            .unwrap()
    }

    pub fn get_user_pow_policy(user_name: &str) -> Option<PowPolicy> {
        let mut connection = DB_POOL.get().unwrap();

        let result = Users::Users
            .filter(Users::username.eq(user_name))
            .select((Users::pow_minimum, Users::pow_accepted, Users::pow_personal))
            .first::<(i32, i32, i32)>(&mut connection)
            .optional()
            .expect("Error querying user pow policy");

        result.map(|(min, accepted, personal)| {
            PowPolicy::new(min as u32, accepted as u32, personal as u32)
        })
    }

    pub fn deliver_email(
        user: &str,
        source_user: &str,
        source_domain: &str,
        email: &EmailContents,
        classification: PowClassification,
    ) -> Result<(), ()> {
        let mut connection = DB_POOL.get().unwrap();

        let user_id = Users::Users
            .filter(Users::username.eq(user))
            .select(Users::user_id)
            .first::<UserId>(&mut connection)
            .map_err(|_| ())?;

        let source_addr = format!("{source_user}@{source_domain}");

        let new_email = NewEmail::new(
            user_id,
            source_addr,
            email.contents().clone(),
            classification.to_ident().to_string(),
        );

        diesel::insert_into(Emails::Emails)
            .values(&new_email)
            .execute(&mut connection)
            .map_err(|_| ())?;

        Ok(())
    }

    pub fn get_emails(authed_user: UserId, since: i64) -> Vec<GetEmailsEmail> {
        todo!()
        // let Ok(user_id): rusqlite::Result<i64> = self.connection.query_row(
        //     "SELECT user_id FROM Users WHERE username = ?1",
        //     [authed_user],
        //     |row| row.get(0),
        // ) else {
        //     return None;
        // };

        // let mut stmt = self.connection.prepare(
        //     "SELECT source, email, pow_classification FROM Emails WHERE user_id = ?1 AND email_id >= ?2",
        // ).unwrap();
        //
        // let rows = stmt.query(params![authed_user, since]).unwrap();
        //
        // rows.map(|row| {
        //     let pow_classification: String = row.get(2).unwrap();
        //     Ok(GetEmailsEmail::new(
        //         row.get(0).unwrap(),
        //         row.get(1).unwrap(),
        //         PowClassification::from_ident(&pow_classification).unwrap(),
        //     ))
        // })
        // .unwrap()
        // .collect_vec()
    }
}