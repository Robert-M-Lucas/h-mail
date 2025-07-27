use crate::root::config::CREATE_ACCOUNT_POW_BURDEN;
use crate::root::receiving::interface::routes::native::get_create_account_pow_policy::{
    CreateAccountPowPolicyRequest, CreateAccountPowPolicyResponse,
};
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;

pub async fn get_create_account_pow_policy(
    Query(_create_account_pow_policy_request): Query<CreateAccountPowPolicyRequest>,
) -> (StatusCode, Json<CreateAccountPowPolicyResponse>) {
    (
        StatusCode::OK,
        CreateAccountPowPolicyResponse::new(CREATE_ACCOUNT_POW_BURDEN).into(),
    )
}
