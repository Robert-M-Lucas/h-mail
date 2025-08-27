use crate::database::diesel_interface::schema::hmail_cc_map;
use crate::database::diesel_interface::schema::hmail_recipient_map;
use crate::database::diesel_interface::schema::hmails;
use crate::database::diesel_interface::schema::user_whitelists;
use crate::database::diesel_interface::schema::users;
use crate::database::{HmailId, UserId};
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use diesel::{Insertable, Queryable};

#[derive(Insertable, new)]
#[diesel(table_name = users)]
pub struct NewUser {
    username: String,
    password_hash: String,
    pow_minimum: i32,
    pow_accepted: i32,
    pow_personal: i32,
}

#[derive(Insertable, new)]
#[diesel(table_name = hmails)]
pub struct NewHmail {
    user_id: UserId,
    outbox: bool,
    context_for: Option<HmailId>,
    sender: String,
    sender_name: Option<String>,
    subject: String,
    sent_at: i64,
    received_at: i64,
    random_id: i64,
    reply_to: Option<String>,
    reply_to_name: Option<String>,
    parent: Option<String>,
    body: String,
    hash: String,
    pow_classification: String,
}

#[derive(Queryable, Getters, Dissolve)]
#[diesel(table_name = hmails)]
pub struct GetHmail {
    hmail_id: HmailId,
    user_id: UserId,
    outbox: bool,
    context_for: Option<HmailId>,
    sender: String,
    sender_name: Option<String>,
    subject: String,
    sent_at: i64,
    received_at: i64,
    random_id: i64,
    reply_to: Option<String>,
    reply_to_name: Option<String>,
    parent: Option<String>,
    body: String,
    hash: String,
    pow_classification: String,
}

#[derive(Insertable, new)]
#[diesel(table_name = hmail_recipient_map)]
pub struct NewRecipient {
    hmail_id: HmailId,
    address: String,
    username: Option<String>,
}

#[derive(Queryable, Dissolve)]
#[diesel(table_name = hmail_to_map)]
pub struct GetRecipient {
    hmail_id: HmailId,
    address: String,
    username: Option<String>,
}

#[derive(Insertable, new)]
#[diesel(table_name = hmail_cc_map)]
pub struct NewCc {
    hmail_id: HmailId,
    address: String,
    username: Option<String>,
}

#[derive(Queryable, Dissolve)]
#[diesel(table_name = hmail_cc_map)]
pub struct GetCc {
    hmail_id: HmailId,
    address: String,
    username: Option<String>,
}

#[derive(Insertable, new)]
#[diesel(table_name = user_whitelists)]
pub struct NewUserWhitelisted {
    user_id: UserId,
    address: String,
    place_in: String,
}
