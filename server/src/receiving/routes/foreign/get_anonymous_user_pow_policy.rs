use crate::database::Db;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use h_mail_interface::interface::routes::foreign::get_anonymous_user_pow_policy::{
    GetAnonymousUserPowPolicyRequest, GetAnonymousUserPowPolicyResponse,
};

pub async fn get_anonymous_user_pow_policy(
    Query(pow_policy_request): Query<GetAnonymousUserPowPolicyRequest>,
) -> (StatusCode, Json<GetAnonymousUserPowPolicyResponse>) {
    let policy = Db::get_user_pow_policy(pow_policy_request.recipient_username()).await;
    (
        StatusCode::OK,
        GetAnonymousUserPowPolicyResponse::new(policy).into(),
    )
}
