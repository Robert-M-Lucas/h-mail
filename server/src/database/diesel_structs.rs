use crate::database::UserId;
use crate::database::schema::Emails;
use crate::database::schema::Users;
use derive_new::new;
use diesel::Insertable;

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
    email: String,
    pow_classification: String,
}
