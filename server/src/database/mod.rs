use crate::config::args::ARGS;
use crate::config::config_file::CONFIG;
use crate::config::salt::SALT;
use crate::database::diesel_structs::{
    GetCc, GetEmail, GetTo, NewCc, NewEmail, NewTo, NewUser, NewUserWhitelisted,
};
use crate::database::schema::EmailCcMap::dsl as EmailCcMap;
use crate::database::schema::EmailToMap::dsl as EmailToMap;
use crate::database::schema::Emails::dsl as Emails;
use crate::database::schema::UserWhitelists::dsl as UserWhitelists;
use crate::database::schema::Users::dsl as Users;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use diesel::Connection;
use diesel::connection::LoadConnection;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::{DatabaseErrorKind, Error};
use diesel::sql_types::Integer;
use diesel::sqlite::Sqlite;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use h_mail_interface::interface::email::{EmailPackage, EmailUser};
use h_mail_interface::interface::fields::big_uint::BigUintField;
use h_mail_interface::interface::fields::system_time::SystemTimeField;
use h_mail_interface::interface::pow::{PowClassification, PowIters, PowPolicy};
use h_mail_interface::interface::routes::native::get_emails::GetEmailsEmail;
use h_mail_interface::interface::routes::native::get_whitelist::WhitelistEntry;
use h_mail_interface::reexports::BigUint;
use h_mail_interface::server_config::MIN_SALT_BYTES;
use h_mail_interface::shared::{ms_since_epoch_to_system_time, system_time_to_ms_since_epoch};
use itertools::Itertools;
use once_cell::sync::Lazy;
use rusqlite::Connection as RusqliteConnection;
use std::time::SystemTime;

mod diesel_structs;
mod schema;

pub type UserId = i32;
pub type EmailId = i32;

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
    if ARGS.no_salt() {
        SaltString::encode_b64(&[0u8; MIN_SALT_BYTES]).unwrap()
    } else {
        SALT.clone().expect("SECRET_SALT not set")
    }
}

pub struct Db;

impl Db {
    pub fn get_user_id_dangerous(user: &str) -> Option<UserId> {
        let mut connection = DB_POOL.get().unwrap();
        Users::Users
            .filter(Users::username.eq(user))
            .select(Users::user_id)
            .first::<UserId>(&mut connection)
            .ok()
    }

    fn get_user_id<C: Connection<Backend = Sqlite> + LoadConnection>(
        connection: &mut C,
        user: &str,
    ) -> Option<UserId> {
        Users::Users
            .filter(Users::username.eq(user))
            .select(Users::user_id)
            .first::<UserId>(connection)
            .ok()
    }

    pub fn create_user(username: &str, password: &str) -> Result<(), ()> {
        let mut connection = DB_POOL.get().unwrap();

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

        let r = diesel::insert_into(Users::Users)
            .values(&new_user)
            .execute(&mut connection);

        match r {
            Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Err(()),
            Err(e) => panic!("{e:?}"),
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

    #[allow(dead_code)]
    pub fn has_user(user: &str) -> bool {
        let mut connection = DB_POOL.get().unwrap();
        Self::get_user_id(&mut connection, user).is_some()
    }

    pub fn user_whitelisted(our_user: &str, address: &str) -> Option<PowClassification> {
        let mut connection = DB_POOL.get().unwrap();
        let Some(user_id) = Self::get_user_id(&mut connection, our_user) else {
            return None;
        };
        let mut connection = DB_POOL.get().unwrap();
        UserWhitelists::UserWhitelists
            .filter(UserWhitelists::user_id.eq(user_id))
            .filter(UserWhitelists::whitelisted.eq(address))
            .select(UserWhitelists::place_in)
            .limit(1)
            .first::<String>(&mut connection)
            .optional()
            .unwrap()
            .map(|s| PowClassification::from_ident(&s).unwrap())
    }

    pub fn add_whitelist(user_id: UserId, address: &str, classification: PowClassification) {
        let mut connection = DB_POOL.get().unwrap();

        diesel::insert_into(UserWhitelists::UserWhitelists)
            .values(NewUserWhitelisted::new(
                user_id,
                address.to_string(),
                classification.to_ident().to_string(),
            ))
            .on_conflict((UserWhitelists::user_id, UserWhitelists::whitelisted))
            .do_update()
            .set(UserWhitelists::place_in.eq(classification.to_ident().to_string()))
            .execute(&mut connection)
            .unwrap();
    }

    pub fn remove_whitelist(user_id: UserId, address: &str) -> bool {
        let mut connection = DB_POOL.get().unwrap();

        let deleted = diesel::delete(
            UserWhitelists::UserWhitelists
                .filter(UserWhitelists::user_id.eq(user_id))
                .filter(UserWhitelists::whitelisted.eq(address)),
        )
        .execute(&mut connection)
        .unwrap();

        deleted > 0
    }

    pub fn get_whitelist(user_id: UserId) -> Vec<WhitelistEntry> {
        let mut connection = DB_POOL.get().unwrap();

        let whitelist: Vec<(String, String)> = UserWhitelists::UserWhitelists
            .filter(UserWhitelists::user_id.eq(user_id))
            .select((UserWhitelists::whitelisted, UserWhitelists::place_in))
            .load::<(String, String)>(&mut connection)
            .unwrap();

        whitelist
            .into_iter()
            .map(|(a, p)| WhitelistEntry::new(a, PowClassification::from_ident(&p).unwrap()))
            .collect_vec()
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
            PowPolicy::new(min as PowIters, accepted as PowIters, personal as PowIters)
        })
    }

    pub fn deliver_email(
        user: &str,
        source_user: &str,
        source_domain: &str,
        email: EmailPackage,
        hash: &BigUint,
        classification: PowClassification,
    ) -> Result<(), ()> {
        let mut connection = DB_POOL.get().unwrap();

        let Some(user_id) = Self::get_user_id(&mut connection, user) else {
            return Err(());
        };

        let source_addr = format!("{source_user}@{source_domain}");

        let (to, subject, sent_at, _random_id, reply_to, cc, parent, body) = email.dissolve();

        let (reply_to, reply_to_name) = if let Some(reply_to) = reply_to {
            let (reply_to, reply_to_name) = reply_to.dissolve();
            (Some(reply_to), reply_to_name)
        } else {
            (None, None)
        };

        connection
            .transaction::<_, Error, _>(|connection| {
                diesel::insert_into(Emails::Emails)
                    .values(&NewEmail::new(
                        user_id,
                        source_addr,
                        subject,
                        system_time_to_ms_since_epoch(&sent_at) as i64,
                        system_time_to_ms_since_epoch(&SystemTime::now()) as i64,
                        reply_to,
                        reply_to_name,
                        parent.map(|h| BigUintField::new(&h).as_string()),
                        body,
                        BigUintField::new(hash).as_string(),
                        classification.to_ident().to_string(),
                    ))
                    .execute(connection)?;

                let email_id: EmailId =
                    diesel::select(sql::<Integer>("last_insert_rowid()")).get_result(connection)?;

                for to in to {
                    diesel::insert_into(EmailToMap::EmailToMap)
                        .values(&NewTo::new(
                            email_id,
                            to.email().clone(),
                            to.display_name().clone(),
                        ))
                        .execute(connection)?;
                }

                for cc in cc {
                    diesel::insert_into(EmailCcMap::EmailCcMap)
                        .values(&NewCc::new(
                            email_id,
                            cc.email().clone(),
                            cc.display_name().clone(),
                        ))
                        .execute(connection)?;
                }

                Ok(())
            })
            .map_err(|_| ())
    }

    pub fn get_emails(authed_user: UserId, since: SystemTime) -> Vec<GetEmailsEmail> {
        let mut connection = DB_POOL.get().unwrap();

        let since = system_time_to_ms_since_epoch(&since) as i64;

        let results: Vec<GetEmail> = Emails::Emails
            .filter(Emails::user_id.eq(authed_user))
            .filter(Emails::received_at.ge(since))
            .load::<GetEmail>(&mut connection)
            .unwrap();

        results
            .into_iter()
            .map(|e| {
                let (
                    email_id,
                    _user_id,
                    source,
                    subject,
                    sent_at,
                    received_at,
                    reply_to,
                    reply_to_name,
                    parent,
                    body,
                    hash,
                    pow_classification,
                ) = e.dissolve();

                let tos: Vec<GetTo> = EmailToMap::EmailToMap
                    .filter(EmailToMap::email_id.eq(email_id))
                    .load::<GetTo>(&mut connection)
                    .unwrap();

                let ccs: Vec<GetCc> = EmailToMap::EmailToMap
                    .filter(EmailToMap::email_id.eq(email_id))
                    .load::<GetCc>(&mut connection)
                    .unwrap();

                let reply_to = reply_to.map(|reply_to| EmailUser::new(reply_to, reply_to_name));

                GetEmailsEmail::new(
                    source,
                    tos.into_iter()
                        .map(|to| {
                            let (_email_id, email, name) = to.dissolve();
                            EmailUser::new(email, name)
                        })
                        .collect_vec(),
                    subject,
                    SystemTimeField::new(&ms_since_epoch_to_system_time(sent_at as u128)),
                    SystemTimeField::new(&ms_since_epoch_to_system_time(received_at as u128)),
                    reply_to,
                    ccs.into_iter()
                        .map(|cc| {
                            let (_email_id, email, name) = cc.dissolve();
                            EmailUser::new(email, name)
                        })
                        .collect_vec(),
                    parent.map(BigUintField::from_raw),
                    body,
                    BigUintField::from_raw(hash),
                    PowClassification::from_ident(&pow_classification).unwrap(),
                )
            })
            .collect_vec()
    }
}
