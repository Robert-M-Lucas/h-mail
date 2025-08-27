use crate::database::diesel_interface::diesel_structs::NewUserWhitelisted;
use crate::database::diesel_interface::schema::user_whitelists::dsl as user_whitelists;
use crate::database::{DB_POOL, Db, UserId};
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use h_mail_interface::interface::fields::hmail_address::HmailAddress;
use h_mail_interface::interface::pow::PowClassification;
use h_mail_interface::interface::routes::native::get_whitelist::WhitelistEntry;
use itertools::Itertools;

impl Db {
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
}
