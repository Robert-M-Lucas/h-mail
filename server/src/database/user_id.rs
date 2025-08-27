use crate::database::diesel_interface::schema::users::dsl as users;
use crate::database::{DB_POOL, Db, UserId};
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::pg::Pg;
use diesel_async::AsyncConnection;
use diesel_async::RunQueryDsl;

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

    pub async fn get_user_id<C: AsyncConnection<Backend = Pg>>(
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
}
