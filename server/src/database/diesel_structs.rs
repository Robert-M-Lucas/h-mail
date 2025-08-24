use crate::database::schema::HmailCcMap;
use crate::database::schema::HmailRecipientsMap;
use crate::database::schema::Hmails;
use crate::database::schema::UserWhitelists;
use crate::database::schema::Users;
use crate::database::{HmailId, UserId};
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use diesel::{Insertable, Queryable};

#[derive(Insertable, new)]
#[diesel(table_name = Users)]
pub struct NewUser {
    username: String,
    password_hash: String,
    pow_minimum: i32,
    pow_accepted: i32,
    pow_personal: i32,
}

#[derive(Insertable, new)]
#[diesel(table_name = Hmails)]
pub struct NewHmail {
    user_id: UserId,
    sender: String,
    sender_name: Option<String>,
    subject: String,
    sent_at: i64,
    received_at: i64,
    reply_to: Option<String>,
    reply_to_name: Option<String>,
    parent: Option<String>,
    body: String,
    hash: String,
    pow_classification: String,
}

#[derive(Queryable, Getters, Dissolve)]
#[diesel(table_name = Hmails)]
pub struct GetHmail {
    hmail_id: HmailId,
    user_id: UserId,
    sender: String,
    sender_name: Option<String>,
    subject: String,
    sent_at: i64,
    received_at: i64,
    reply_to: Option<String>,
    reply_to_name: Option<String>,
    parent: Option<String>,
    body: String,
    hash: String,
    pow_classification: String,
}

#[derive(Insertable, new)]
#[diesel(table_name = HmailRecipientsMap)]
pub struct NewRecipient {
    hmail_id: HmailId,
    address: String,
    username: Option<String>,
}

#[derive(Queryable, Dissolve)]
#[diesel(table_name = HmailToMap)]
pub struct GetRecipient {
    hmail_id: HmailId,
    address: String,
    username: Option<String>,
}

#[derive(Insertable, new)]
#[diesel(table_name = HmailCcMap)]
pub struct NewCc {
    hmail_id: HmailId,
    address: String,
    username: Option<String>,
}

#[derive(Queryable, Dissolve)]
#[diesel(table_name = HmailCcMap)]
pub struct GetCc {
    hmail_id: HmailId,
    address: String,
    username: Option<String>,
}

#[derive(Insertable, new)]
#[diesel(table_name = UserWhitelists)]
pub struct NewUserWhitelisted {
    user_id: UserId,
    address: String,
    place_in: String,
}
