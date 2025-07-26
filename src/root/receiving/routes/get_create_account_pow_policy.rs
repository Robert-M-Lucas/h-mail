use crate::root::receiving::interface::get_create_account_pow_policy::{CreateAccountPowPolicyRequest, CreateAccountPowPolicyResponse};
use crate::root::config::CREATE_ACCOUNT_POW_BURDEN;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;

pub async fn get_create_account_pow_policy(
    Query(_create_account_pow_policy_request): Query<CreateAccountPowPolicyRequest>
) -> (StatusCode, Json<CreateAccountPowPolicyResponse>) {
    (StatusCode::OK, CreateAccountPowPolicyResponse::new(CREATE_ACCOUNT_POW_BURDEN).into())
}
