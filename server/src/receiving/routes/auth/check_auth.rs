use crate::receiving::auth_util::auth_header::AuthorizationHeader;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::routes::auth::check_auth::{
    CheckAuthRequest, CheckAuthResponse, CheckAuthResponseAuthed,
};

pub async fn check_auth(
    auth_header: AuthorizationHeader,
    Query(_check_auth): Query<CheckAuthRequest>,
) -> (StatusCode, Json<CheckAuthResponse>) {
    if auth_header.check_access_token().await.is_none() {
        return (StatusCode::UNAUTHORIZED, Authorized::Unauthorized.into());
    };

    (
        StatusCode::OK,
        Authorized::Success(CheckAuthResponseAuthed).into(),
    )
}
