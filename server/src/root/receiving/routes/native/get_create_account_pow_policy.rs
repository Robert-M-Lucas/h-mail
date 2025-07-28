use crate::root::config::CREATE_ACCOUNT_POW_BURDEN;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use h_mail_interface::interface::routes::native::get_create_account_pow_policy::{
    CreateAccountPowPolicyRequest, CreateAccountPowPolicyResponse,
};

pub async fn get_create_account_pow_policy(
    Query(_create_account_pow_policy_request): Query<CreateAccountPowPolicyRequest>,
) -> (StatusCode, Json<CreateAccountPowPolicyResponse>) {
    (
        StatusCode::OK,
        CreateAccountPowPolicyResponse::new(CREATE_ACCOUNT_POW_BURDEN).into(),
    )
}
