use crate::database::diesel_interface::schema::users::dsl as users;
use crate::database::{DB_POOL, Db, UserId};
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use h_mail_interface::interface::pow::{PowIters, PowPolicy};

impl Db {
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
}
