use crate::root::receiving::interface::get_user_pow_policy::{
    UserPowPolicyRequest, UserPowPolicyResponse,
};
use crate::root::shared_resources::DB;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;

pub async fn get_user_pow_policy(
    Query(pow_policy_request): Query<UserPowPolicyRequest>,
) -> (StatusCode, Json<UserPowPolicyResponse>) {
    let policy = DB
        .lock()
        .await
        .as_ref()
        .unwrap()
        .get_user_pow_policy(pow_policy_request.destination());
    (StatusCode::OK, UserPowPolicyResponse::new(policy).into())
}
