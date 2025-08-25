use crate::config::args::ARGS;
use crate::config::config_file::CONFIG;
use crate::config::salt::SALT;
use crate::database::diesel_structs::{
    GetCc, GetHmail, GetRecipient, NewCc, NewHmail, NewRecipient, NewUser, NewUserWhitelisted,
};
use crate::database::schema::hmail_cc_map::dsl as hmail_cc_map;
use crate::database::schema::hmail_recipient_map::dsl as hmail_recipient_map;
use crate::database::schema::hmails::dsl as hmails;
use crate::database::schema::user_whitelists::dsl as user_whitelists;
use crate::database::schema::users::dsl as users;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use diesel_async::RunQueryDsl;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, AsyncPgConnection};
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
use std::time::SystemTime;

mod diesel_structs;
mod schema;

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

impl Db {
    pub async fn get_user_id_dangerous(user: &str) -> Option<UserId> {
        let mut connection = DB_POOL.get().await.unwrap();
        users::users
            .filter(users::username.eq(user))
            .select(users::user_id)
            .first::<UserId>(&mut connection)
            .await
            .ok()
    }

    async fn get_user_id<C: AsyncConnection<Backend = Pg>>(
        connection: &mut C,
        user: &str,
    ) -> Option<UserId> {
        users::users
            .filter(users::username.eq(user))
            .select(users::user_id)
            .first::<UserId>(connection)
            .await
            .ok()
    }

    pub async fn create_user(username: &str, password: &str) -> Result<(), ()> {
        let mut connection = DB_POOL.get().await.unwrap();

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
            Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Err(()),
            Err(e) => panic!("{e:?}"),
            Ok(_) => Ok(()),
        }
    }

    pub async fn authenticate(username: &str, password: &str) -> Result<UserId, ()> {
        let mut connection = DB_POOL.get().await.unwrap();

        let salt_string = get_salt();
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt_string)
            .unwrap();

        let user_result: UserId = users::users
            .filter(users::username.eq(username))
            .filter(users::password_hash.eq(password_hash.to_string()))
            .select(users::user_id)
            .first(&mut connection)
            .await
            .map_err(|_| ())?;

        Ok(user_result)
    }

    #[allow(dead_code)]
    pub async fn has_user(user: &str) -> bool {
        let mut connection = DB_POOL.get().await.unwrap();
        Self::get_user_id(&mut connection, user).await.is_some()
    }

    pub async fn user_whitelisted(
        our_user: &str,
        address: &HmailAddress,
    ) -> Option<PowClassification> {
        let mut connection = DB_POOL.get().await.unwrap();
        let user_id = Self::get_user_id(&mut connection, our_user).await?;
        let mut connection = DB_POOL.get().await.unwrap();
        user_whitelists::user_whitelists
            .filter(user_whitelists::user_id.eq(user_id))
            .filter(user_whitelists::address.eq(address.as_str().to_string()))
            .select(user_whitelists::place_in)
            .limit(1)
            .first::<String>(&mut connection)
            .await
            .optional()
            .unwrap()
            .map(|s| PowClassification::from_ident(&s).unwrap())
    }

    pub async fn add_whitelist(
        user_id: UserId,
        address: &HmailAddress,
        classification: PowClassification,
    ) {
        let mut connection = DB_POOL.get().await.unwrap();

        diesel::insert_into(user_whitelists::user_whitelists)
            .values(NewUserWhitelisted::new(
                user_id,
                address.as_str().to_string(),
                classification.to_ident().to_string(),
            ))
            .on_conflict((user_whitelists::user_id, user_whitelists::address))
            .do_update()
            .set(user_whitelists::place_in.eq(classification.to_ident().to_string()))
            .execute(&mut connection)
            .await
            .unwrap();
    }

    pub async fn remove_whitelist(user_id: UserId, address: &str) -> bool {
        let mut connection = DB_POOL.get().await.unwrap();

        let deleted = diesel::delete(
            user_whitelists::user_whitelists
                .filter(user_whitelists::user_id.eq(user_id))
                .filter(user_whitelists::address.eq(address)),
        )
        .execute(&mut connection)
        .await
        .unwrap();

        deleted > 0
    }

    pub async fn get_whitelist(user_id: UserId) -> Vec<WhitelistEntry> {
        let mut connection = DB_POOL.get().await.unwrap();

        let whitelist: Vec<(String, String)> = user_whitelists::user_whitelists
            .filter(user_whitelists::user_id.eq(user_id))
            .select((user_whitelists::address, user_whitelists::place_in))
            .load::<(String, String)>(&mut connection)
            .await
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

    pub async fn get_username_from_id(id: UserId) -> Option<String> {
        let mut connection = DB_POOL.get().await.unwrap();

        users::users
            .filter(users::user_id.eq(id))
            .select(users::username)
            .first::<String>(&mut connection)
            .await
            .optional()
            .unwrap()
    }

    pub async fn get_user_pow_policy(user_name: &str) -> Option<PowPolicy> {
        let mut connection = DB_POOL.get().await.unwrap();

        let result = users::users
            .filter(users::username.eq(user_name))
            .select((users::pow_minimum, users::pow_accepted, users::pow_personal))
            .first::<(i32, i32, i32)>(&mut connection)
            .await
            .optional()
            .expect("Error querying user pow policy");

        result.map(|(min, accepted, personal)| {
            PowPolicy::new(min as PowIters, accepted as PowIters, personal as PowIters)
        })
    }

    pub async fn get_pow_policy(user_id: UserId) -> PowPolicy {
        let mut connection = DB_POOL.get().await.unwrap();

        let (minimum, accepted, personal) = users::users
            .filter(users::user_id.eq(user_id))
            .select((users::pow_minimum, users::pow_accepted, users::pow_personal))
            .first::<(i32, i32, i32)>(&mut connection)
            .await
            .expect("Error querying user pow policy");

        PowPolicy::new(
            minimum as PowIters,
            accepted as PowIters,
            personal as PowIters,
        )
    }

    pub async fn set_pow_policy(user_id: UserId, new_policy: &PowPolicy) {
        let mut connection = DB_POOL.get().await.unwrap();

        diesel::update(users::users.filter(users::user_id.eq(user_id)))
            .set((
                users::pow_minimum.eq(*new_policy.minimum() as i32),
                users::pow_accepted.eq(*new_policy.accepted() as i32),
                users::pow_personal.eq(*new_policy.personal() as i32),
            ))
            .execute(&mut connection)
            .await
            .expect("Error updating user POW policy");
    }

    pub async fn deliver_hmail(
        user: &str,
        hmail: HmailPackage,
        hash: &BigUint,
        classification: PowClassification,
        context: Vec<(HmailPackage, BigUint)>,
        outbox: bool,
    ) -> Result<(), ()> {
        let mut connection = DB_POOL.get().await.unwrap();
        let Some(user_id) = Self::get_user_id(&mut connection, user).await else {
            return Err(());
        };
        Self::deliver_hmail_to_id(user_id, hmail, hash, classification, context, outbox).await
    }

    pub async fn deliver_hmail_to_id(
        user_id: UserId,
        hmail: HmailPackage,
        hash: &BigUint,
        classification: PowClassification,
        context: Vec<(HmailPackage, BigUint)>,
        outbox: bool
    ) -> Result<(), ()> {
        let mut connection = DB_POOL.get().await.unwrap();

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
                async move {
                    let hmail_id = diesel::insert_into(hmails::hmails)
                        .values(&NewHmail::new(
                            user_id,
                            outbox,
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
                        .returning(hmails::hmail_id)
                        .get_result(connection)
                        .await
                        .unwrap();

                    for recipient in recipients {
                        diesel::insert_into(hmail_recipient_map::hmail_recipient_map)
                            .values(&NewRecipient::new(
                                hmail_id,
                                recipient.address().as_str().to_string(),
                                recipient.display_name().clone(),
                            ))
                            .execute(connection)
                            .await?;
                    }

                    for cc in ccs {
                        diesel::insert_into(hmail_cc_map::hmail_cc_map)
                            .values(&NewCc::new(
                                hmail_id,
                                cc.address().as_str().to_string(),
                                cc.display_name().clone(),
                            ))
                            .execute(connection)
                            .await?;
                    }

                    for (context, hash) in context {
                        let (
                            sender,
                            recipients,
                            subject,
                            sent_at,
                            random_id,
                            reply_to,
                            ccs,
                            parent,
                            body,
                        ) = context.dissolve();

                        let (reply_to, reply_to_name) = if let Some(reply_to) = reply_to {
                            let (reply_to, reply_to_name) = reply_to.dissolve();
                            (Some(reply_to), reply_to_name)
                        } else {
                            (None, None)
                        };

                        let hmail_id = diesel::insert_into(hmails::hmails)
                            .values(&NewHmail::new(
                                user_id,
                                false,
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
                            .returning(hmails::hmail_id)
                            .get_result(connection)
                            .await
                            .unwrap();

                        for recipient in recipients {
                            diesel::insert_into(hmail_recipient_map::hmail_recipient_map)
                                .values(&NewRecipient::new(
                                    hmail_id,
                                    recipient.address().as_str().to_string(),
                                    recipient.display_name().clone(),
                                ))
                                .execute(connection)
                                .await?;
                        }

                        for cc in ccs {
                            diesel::insert_into(hmail_cc_map::hmail_cc_map)
                                .values(&NewCc::new(
                                    hmail_id,
                                    cc.address().as_str().to_string(),
                                    cc.display_name().clone(),
                                ))
                                .execute(connection)
                                .await?;
                        }
                    }

                    Ok(())
                }
                .scope_boxed()
            })
            .await
            .map_err(|_| ())
    }

    pub async fn get_hmails(
        authed_user: UserId,
        until: Option<HmailId>,
        limit: u32,
        outbox: bool,
    ) -> Vec<GetHmailsHmail> {
        let mut connection = DB_POOL.get().await.unwrap();

        connection
            .transaction::<_, Error, _>(|connection| {
                async move {
                    let results: Vec<GetHmail> = if let Some(until) = until {
                        hmails::hmails
                            .filter(hmails::user_id.eq(authed_user))
                            .filter(hmails::hmail_id.lt(until))
                            .filter(hmails::outbox.eq(outbox))
                            .filter(hmails::context_for.is_null()) // Exclude context hmails
                            .order_by(hmails::hmail_id.desc())
                            .limit(limit as i64)
                            .load::<GetHmail>(connection)
                            .await
                            .unwrap()
                    } else {
                        hmails::hmails
                            .filter(hmails::user_id.eq(authed_user))
                            .filter(hmails::outbox.eq(outbox))
                            .filter(hmails::context_for.is_null()) // Exclude context hmails
                            .order_by(hmails::hmail_id.desc())
                            .limit(limit as i64)
                            .load::<GetHmail>(connection)
                            .await
                            .unwrap()
                    };

                    let mut processed = Vec::new();

                    for result in results {
                        processed
                            .push(Self::get_hmail_to_get_hmails_hmail(connection, result).await)
                    }

                    Ok(processed)
                }
                .scope_boxed()
            })
            .await
            .unwrap()
    }

    pub async fn get_hmail_by_hash(
        authed_user: UserId,
        hash: &BigUintField,
    ) -> Option<GetHmailsHmail> {
        let mut connection = DB_POOL.get().await.unwrap();

        connection
            .transaction::<_, Error, _>(|connection| {
                async move {
                    let result = hmails::hmails
                        .filter(hmails::user_id.eq(authed_user))
                        .filter(hmails::hash.eq(hash.as_str()))
                        .first::<GetHmail>(connection)
                        .await
                        .optional()
                        .unwrap();

                    Ok(if let Some(result) = result {
                        Some(Self::get_hmail_to_get_hmails_hmail(connection, result).await)
                    } else {
                        None
                    })
                }
                .scope_boxed()
            })
            .await
            .unwrap()
    }

    async fn get_hmail_to_get_hmails_hmail<C: AsyncConnection<Backend = Pg>>(
        connection: &mut C,
        get_hmail: GetHmail,
    ) -> GetHmailsHmail {
        let (
            hmail_id,
            _user_id,
            _outbox,
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

        let tos: Vec<GetRecipient> = hmail_recipient_map::hmail_recipient_map
            .filter(hmail_recipient_map::hmail_id.eq(hmail_id))
            .load::<GetRecipient>(connection)
            .await
            .unwrap();

        let ccs: Vec<GetCc> = hmail_cc_map::hmail_cc_map
            .filter(hmail_cc_map::hmail_id.eq(hmail_id))
            .load::<GetCc>(connection)
            .await
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
