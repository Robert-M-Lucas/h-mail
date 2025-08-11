use crate::database::schema::EmailCcMap;
use crate::database::schema::EmailToMap;
use crate::database::schema::Emails;
use crate::database::schema::UserWhitelists;
use crate::database::schema::Users;
use crate::database::{EmailId, UserId};
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
#[diesel(table_name = Emails)]
pub struct NewEmail {
    user_id: UserId,
    source: String,
    subject: String,
    sent_at: i64,
    received_at: i64,
    mime_version: String,
    content_type: String,
    reply_to: Option<String>,
    reply_to_name: Option<String>,
    parent: Option<String>,
    body: String,
    hash: String,
    pow_classification: String,
}

#[derive(Queryable, Getters, Dissolve)]
#[diesel(table_name = Emails)]
pub struct GetEmail {
    email_id: EmailId,
    user_id: UserId,
    source: String,
    subject: String,
    sent_at: i64,
    received_at: i64,
    mime_version: String,
    content_type: String,
    reply_to: Option<String>,
    reply_to_name: Option<String>,
    parent: Option<String>,
    body: String,
    hash: String,
    pow_classification: String,
}

#[derive(Insertable, new)]
#[diesel(table_name = EmailToMap)]
pub struct NewTo {
    email_id: EmailId,
    email: String,
    name: Option<String>,
}

#[derive(Queryable, Dissolve)]
#[diesel(table_name = EmailToMap)]
pub struct GetTo {
    email_id: EmailId,
    email: String,
    name: Option<String>,
}

#[derive(Insertable, new)]
#[diesel(table_name = EmailCcMap)]
pub struct NewCc {
    email_id: EmailId,
    email: String,
    name: Option<String>,
}

#[derive(Queryable, Dissolve)]
#[diesel(table_name = EmailCcMap)]
pub struct GetCc {
    email_id: EmailId,
    email: String,
    name: Option<String>,
}

#[derive(Insertable, new)]
#[diesel(table_name = UserWhitelists)]
pub struct NewUserWhitelisted {
    user_id: UserId,
    whitelisted: String,
    place_in: String,
}
