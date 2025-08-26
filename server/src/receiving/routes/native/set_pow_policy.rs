use crate::database::Db;
use crate::receiving::auth_util::auth_header::AuthorizationHeader;
use axum::Json;
use axum::http::StatusCode;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::routes::native::set_pow_policy::{
    SetPowPolicyRequest, SetPowPolicyResponse, SetPowPolicyResponseAuthed,
};

pub async fn set_pow_policy(
    auth_header: AuthorizationHeader,
    Json(set_pow_policy): Json<SetPowPolicyRequest>,
) -> (StatusCode, Json<SetPowPolicyResponse>) {
    let Some(user_id) = auth_header.check_access_token().await else {
        return (StatusCode::OK, Authorized::Unauthorized.into());
    };

    Db::set_pow_policy(user_id, set_pow_policy.policy()).await;

    (
        StatusCode::OK,
        Authorized::Success(SetPowPolicyResponseAuthed::Success).into(),
    )
}
