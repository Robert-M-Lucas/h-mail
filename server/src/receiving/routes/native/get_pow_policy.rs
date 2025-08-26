use crate::database::Db;
use crate::receiving::auth_util::auth_header::AuthorizationHeader;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::routes::native::get_pow_policy::{
    GetPowPolicyRequest, GetPowPolicyResponse, GetPowPolicyResponseAuthed,
};

pub async fn get_pow_policy(
    auth_header: AuthorizationHeader,
    Query(_get_pow_policy): Query<GetPowPolicyRequest>,
) -> (StatusCode, Json<GetPowPolicyResponse>) {
    let Some(user_id) = auth_header.check_access_token().await else {
        return (StatusCode::OK, Authorized::Unauthorized.into());
    };

    (
        StatusCode::OK,
        Authorized::Success(GetPowPolicyResponseAuthed::new(
            Db::get_pow_policy(user_id).await,
        ))
        .into(),
    )
}
