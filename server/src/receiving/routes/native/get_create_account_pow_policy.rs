use crate::config::CREATE_ACCOUNT_POW_BURDEN;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use h_mail_interface::interface::routes::native::get_create_account_pow_policy::{
    GetCreateAccountPowPolicyRequest, GetCreateAccountPowPolicyResponse,
};

pub async fn get_create_account_pow_policy(
    Query(_create_account_pow_policy_request): Query<GetCreateAccountPowPolicyRequest>,
) -> (StatusCode, Json<GetCreateAccountPowPolicyResponse>) {
    (
        StatusCode::OK,
        GetCreateAccountPowPolicyResponse::new(CREATE_ACCOUNT_POW_BURDEN).into(),
    )
}
