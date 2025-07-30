use crate::database::Db;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use h_mail_interface::interface::routes::foreign::get_user_pow_policy::{
    GetUserPowPolicyRequest, GetUserPowPolicyResponse,
};

pub async fn get_user_pow_policy(
    Query(pow_policy_request): Query<GetUserPowPolicyRequest>,
) -> (StatusCode, Json<GetUserPowPolicyResponse>) {
    let policy = Db::get_user_pow_policy(pow_policy_request.destination());
    (StatusCode::OK, GetUserPowPolicyResponse::new(policy).into())
}
