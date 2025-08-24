use crate::config::args::ARGS;
use crate::config::config_file::CONFIG;
use crate::config::salt::SALT;
use crate::database::diesel_structs::{
    GetCc, GetHmail, GetRecipient, NewCc, NewHmail, NewRecipient, NewUser, NewUserWhitelisted,
};
use crate::database::schema::HmailCcMap::dsl as HmailCcMap;
use crate::database::schema::HmailRecipientsMap::dsl as HmailRecipientsMap;
use crate::database::schema::Hmails::dsl as Hmails;
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
use h_mail_interface::interface::fields::big_uint::BigUintField;
use h_mail_interface::interface::fields::hmail_address::HmailAddress;
use h_mail_interface::interface::fields::system_time::SystemTimeField;
use h_mail_interface::interface::hmail::{HmailPackage, HmailUser};
use h_mail_interface::interface::pow::{PowClassification, PowIters, PowPolicy};
use h_mail_interface::interface::routes::native::get_hmails::GetHmailsHmail;
use h_mail_interface::interface::routes::native::get_whitelist::WhitelistEntry;
use h_mail_interface::reexports::BigUint;
use h_mail_interface::server_config::MIN_SALT_BYTES;
use h_mail_interface::utility::{ms_since_epoch_to_system_time, system_time_to_ms_since_epoch};
use itertools::Itertools;
use once_cell::sync::Lazy;
use rusqlite::Connection as RusqliteConnection;
use std::time::SystemTime;

mod diesel_structs;
mod schema;

pub type UserId = i32;
pub type HmailId = i32;

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

    pub fn user_whitelisted(our_user: &str, address: &HmailAddress) -> Option<PowClassification> {
        let mut connection = DB_POOL.get().unwrap();
        let Some(user_id) = Self::get_user_id(&mut connection, our_user) else {
            return None;
        };
        let mut connection = DB_POOL.get().unwrap();
        UserWhitelists::UserWhitelists
            .filter(UserWhitelists::user_id.eq(user_id))
            .filter(UserWhitelists::address.eq(address.as_str().to_string()))
            .select(UserWhitelists::place_in)
            .limit(1)
            .first::<String>(&mut connection)
            .optional()
            .unwrap()
            .map(|s| PowClassification::from_ident(&s).unwrap())
    }

    pub fn add_whitelist(
        user_id: UserId,
        address: &HmailAddress,
        classification: PowClassification,
    ) {
        let mut connection = DB_POOL.get().unwrap();

        diesel::insert_into(UserWhitelists::UserWhitelists)
            .values(NewUserWhitelisted::new(
                user_id,
                address.as_str().to_string(),
                classification.to_ident().to_string(),
            ))
            .on_conflict((UserWhitelists::user_id, UserWhitelists::address))
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
                .filter(UserWhitelists::address.eq(address)),
        )
        .execute(&mut connection)
        .unwrap();

        deleted > 0
    }

    pub fn get_whitelist(user_id: UserId) -> Vec<WhitelistEntry> {
        let mut connection = DB_POOL.get().unwrap();

        let whitelist: Vec<(String, String)> = UserWhitelists::UserWhitelists
            .filter(UserWhitelists::user_id.eq(user_id))
            .select((UserWhitelists::address, UserWhitelists::place_in))
            .load::<(String, String)>(&mut connection)
            .unwrap();

        whitelist
            .into_iter()
            .map(|(a, p)| {
                WhitelistEntry::new(
                    HmailAddress::new(&a).unwrap(),
                    PowClassification::from_ident(&p).unwrap(),
                )
            })
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

    pub fn get_pow_policy(user_id: UserId) -> PowPolicy {
        let mut connection = DB_POOL.get().unwrap();

        let (minimum, accepted, personal) = Users::Users
            .filter(Users::user_id.eq(user_id))
            .select((Users::pow_minimum, Users::pow_accepted, Users::pow_personal))
            .first::<(i32, i32, i32)>(&mut connection)
            .expect("Error querying user pow policy");

        PowPolicy::new(
            minimum as PowIters,
            accepted as PowIters,
            personal as PowIters,
        )
    }

    pub fn set_pow_policy(user_id: UserId, new_policy: &PowPolicy) {
        let mut connection = DB_POOL.get().unwrap();

        diesel::update(Users::Users.filter(Users::user_id.eq(user_id)))
            .set((
                Users::pow_minimum.eq(*new_policy.minimum() as i32),
                Users::pow_accepted.eq(*new_policy.accepted() as i32),
                Users::pow_personal.eq(*new_policy.personal() as i32),
            ))
            .execute(&mut connection)
            .expect("Error updating user POW policy");
    }

    pub fn deliver_hmail(
        user: &str,
        hmail: HmailPackage,
        hash: &BigUint,
        classification: PowClassification,
        context: Vec<(HmailPackage, BigUint)>,
    ) -> Result<(), ()> {
        let mut connection = DB_POOL.get().unwrap();

        let Some(user_id) = Self::get_user_id(&mut connection, user) else {
            return Err(());
        };

        let (sender, recipients, subject, sent_at, random_id, reply_to, ccs, parent, body) =
            hmail.dissolve();

        let (reply_to, reply_to_name) = if let Some(reply_to) = reply_to {
            let (reply_to, reply_to_name) = reply_to.dissolve();
            (Some(reply_to), reply_to_name)
        } else {
            (None, None)
        };

        connection
            .transaction::<_, Error, _>(|connection| {
                diesel::insert_into(Hmails::Hmails)
                    .values(&NewHmail::new(
                        user_id,
                        None,
                        sender.address().as_str().to_string(),
                        sender.display_name().clone(),
                        subject,
                        system_time_to_ms_since_epoch(&sent_at) as i64,
                        system_time_to_ms_since_epoch(&SystemTime::now()) as i64,
                        random_id as i64,
                        reply_to.map(|a| a.as_str().to_string()),
                        reply_to_name,
                        parent.map(|h| BigUintField::new(&h).to_string()),
                        body,
                        BigUintField::new(hash).to_string(),
                        classification.to_ident().to_string(),
                    ))
                    .execute(connection)
                    .unwrap();

                let hmail_id: HmailId =
                    diesel::select(sql::<Integer>("last_insert_rowid()")).get_result(connection)?;

                for recipient in recipients {
                    diesel::insert_into(HmailRecipientsMap::HmailRecipientsMap)
                        .values(&NewRecipient::new(
                            hmail_id,
                            recipient.address().as_str().to_string(),
                            recipient.display_name().clone(),
                        ))
                        .execute(connection)?;
                }

                for cc in ccs {
                    diesel::insert_into(HmailCcMap::HmailCcMap)
                        .values(&NewCc::new(
                            hmail_id,
                            cc.address().as_str().to_string(),
                            cc.display_name().clone(),
                        ))
                        .execute(connection)?;
                }

                for (context, hash) in context {
                    let (sender, recipients, subject, sent_at, random_id, reply_to, ccs, parent, body) =
                        context.dissolve();

                    let (reply_to, reply_to_name) = if let Some(reply_to) = reply_to {
                        let (reply_to, reply_to_name) = reply_to.dissolve();
                        (Some(reply_to), reply_to_name)
                    } else {
                        (None, None)
                    };

                    diesel::insert_into(Hmails::Hmails)
                        .values(&NewHmail::new(
                            user_id,
                            Some(hmail_id),
                            sender.address().as_str().to_string(),
                            sender.display_name().clone(),
                            subject,
                            system_time_to_ms_since_epoch(&sent_at) as i64,
                            system_time_to_ms_since_epoch(&SystemTime::now()) as i64,
                            random_id as i64,
                            reply_to.map(|a| a.as_str().to_string()),
                            reply_to_name,
                            parent.map(|h| BigUintField::new(&h).to_string()),
                            body,
                            BigUintField::new(&hash).to_string(),
                            classification.to_ident().to_string(),
                        ))
                        .execute(connection)
                        .unwrap();

                    let hmail_id: HmailId = diesel::select(sql::<Integer>("last_insert_rowid()"))
                        .get_result(connection)?;

                    for recipient in recipients {
                        diesel::insert_into(HmailRecipientsMap::HmailRecipientsMap)
                            .values(&NewRecipient::new(
                                hmail_id,
                                recipient.address().as_str().to_string(),
                                recipient.display_name().clone(),
                            ))
                            .execute(connection)?;
                    }

                    for cc in ccs {
                        diesel::insert_into(HmailCcMap::HmailCcMap)
                            .values(&NewCc::new(
                                hmail_id,
                                cc.address().as_str().to_string(),
                                cc.display_name().clone(),
                            ))
                            .execute(connection)?;
                    }
                }

                Ok(())
            })
            .map_err(|_| ())
    }

    pub fn get_hmails(authed_user: UserId, until: Option<i32>, limit: u32) -> Vec<GetHmailsHmail> {
        let mut connection = DB_POOL.get().unwrap();

        let results: Vec<GetHmail> = if let Some(until) = until {
            Hmails::Hmails
                .filter(Hmails::user_id.eq(authed_user))
                .filter(Hmails::hmail_id.lt(until))
                .filter(Hmails::context_for.is_null()) // Exclude context hmails
                .order_by(Hmails::hmail_id.desc())
                .limit(limit as i64)
                .load::<GetHmail>(&mut connection)
                .unwrap()
        } else {
            Hmails::Hmails
                .filter(Hmails::user_id.eq(authed_user))
                .filter(Hmails::context_for.is_null()) // Exclude context hmails
                .order_by(Hmails::hmail_id.desc())
                .limit(limit as i64)
                .load::<GetHmail>(&mut connection)
                .unwrap()
        };

        results
            .into_iter()
            .map(|e| Self::get_hmail_to_get_hmails_hmail(&mut connection, e))
            .collect_vec()
    }

    pub fn get_hmail_by_hash(authed_user: UserId, hash: &BigUintField) -> Option<GetHmailsHmail> {
        let mut connection = DB_POOL.get().unwrap();

        let result = Hmails::Hmails
            .filter(Hmails::user_id.eq(authed_user))
            .filter(Hmails::hash.eq(hash.as_str()))
            .first::<GetHmail>(&mut connection)
            .optional()
            .unwrap();

        result.map(|e| Self::get_hmail_to_get_hmails_hmail(&mut connection, e))
    }

    fn get_hmail_to_get_hmails_hmail<C: Connection<Backend = Sqlite> + LoadConnection>(
        connection: &mut C,
        get_hmail: GetHmail,
    ) -> GetHmailsHmail {
        let (
            hmail_id,
            _user_id,
            context_for,
            sender,
            sender_name,
            subject,
            sent_at,
            received_at,
            random_id,
            reply_to,
            reply_to_name,
            parent,
            body,
            hash,
            pow_classification,
        ) = get_hmail.dissolve();

        let tos: Vec<GetRecipient> = HmailRecipientsMap::HmailRecipientsMap
            .filter(HmailRecipientsMap::hmail_id.eq(hmail_id))
            .load::<GetRecipient>(connection)
            .unwrap();

        let ccs: Vec<GetCc> = HmailRecipientsMap::HmailRecipientsMap
            .filter(HmailRecipientsMap::hmail_id.eq(hmail_id))
            .load::<GetCc>(connection)
            .unwrap();

        let reply_to = reply_to
            .map(|reply_to| HmailUser::new(HmailAddress::new(&reply_to).unwrap(), reply_to_name));

        GetHmailsHmail::new(
            hmail_id,
            context_for.is_some(),
            HmailUser::new(HmailAddress::new(&sender).unwrap(), sender_name),
            tos.into_iter()
                .map(|to| {
                    let (_hmail_id, address, username) = to.dissolve();
                    HmailUser::new(HmailAddress::new(&address).unwrap(), username)
                })
                .collect_vec(),
            subject,
            SystemTimeField::new(&ms_since_epoch_to_system_time(sent_at as u128)),
            SystemTimeField::new(&ms_since_epoch_to_system_time(received_at as u128)),
            random_id as u32,
            reply_to,
            ccs.into_iter()
                .map(|cc| {
                    let (_hmail_id, address, username) = cc.dissolve();
                    HmailUser::new(HmailAddress::new(&address).unwrap(), username)
                })
                .collect_vec(),
            parent.map(BigUintField::from_raw),
            body,
            BigUintField::from_raw(hash),
            PowClassification::from_ident(&pow_classification).unwrap(),
        )
    }
}
