use crate::database::diesel_interface::schema::users::dsl as users;
use crate::database::{DB_POOL, Db, UserId, get_salt};
use argon2::{Argon2, PasswordHasher};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;

impl Db {
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
}
