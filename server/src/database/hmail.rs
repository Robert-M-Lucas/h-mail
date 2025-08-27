use crate::database::diesel_interface::diesel_structs::{
    GetCc, GetHmail, GetRecipient, NewCc, NewHmail, NewRecipient,
};
use crate::database::diesel_interface::schema::hmail_cc_map::dsl as hmail_cc_map;
use crate::database::diesel_interface::schema::hmail_recipient_map::dsl as hmail_recipient_map;
use crate::database::diesel_interface::schema::hmails::dsl as hmails;
use crate::database::{DB_POOL, Db, HmailId, UserId};
use diesel::pg::Pg;
use diesel::result::Error;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};
use h_mail_interface::interface::fields::big_uint::BigUintField;
use h_mail_interface::interface::fields::hmail_address::HmailAddress;
use h_mail_interface::interface::fields::system_time::SystemTimeField;
use h_mail_interface::interface::hmail::{HmailPackage, HmailUser};
use h_mail_interface::interface::pow::PowClassification;
use h_mail_interface::interface::routes::native::get_hmails::GetHmailsHmail;
use h_mail_interface::reexports::BigUint;
use h_mail_interface::utility::{ms_since_epoch_to_system_time, system_time_to_ms_since_epoch};
use itertools::Itertools;
use std::time::SystemTime;

impl Db {
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
        outbox: bool,
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
