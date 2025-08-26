use crate::database::Db;
use crate::receiving::auth_util::auth_header::AuthorizationHeader;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::routes::native::remove_whitelist::{
    RemoveWhitelistRequest, RemoveWhitelistResponse, RemoveWhitelistResponseAuthed,
};

pub async fn remove_whitelist(
    auth_header: AuthorizationHeader,
    Query(remove_whitelist): Query<RemoveWhitelistRequest>,
) -> (StatusCode, Json<RemoveWhitelistResponse>) {
    let Some(user_id) = auth_header.check_access_token().await else {
        return (StatusCode::OK, Authorized::Unauthorized.into());
    };

    if Db::remove_whitelist(user_id, remove_whitelist.address()).await {
        (
            StatusCode::OK,
            Authorized::Success(RemoveWhitelistResponseAuthed::Success).into(),
        )
    } else {
        (
            StatusCode::OK,
            Authorized::Success(RemoveWhitelistResponseAuthed::Failure).into(),
        )
    }
}
