use crate::database::Db;
use crate::receiving::auth_util::auth_header::AuthorizationHeader;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::routes::native::get_whitelist::{
    GetWhitelistRequest, GetWhitelistResponse, GetWhitelistResponseAuthed,
};

pub async fn get_whitelist(
    auth_header: AuthorizationHeader,
    Query(_get_whitelist): Query<GetWhitelistRequest>,
) -> (StatusCode, Json<GetWhitelistResponse>) {
    let Some(user_id) = auth_header.check_access_token().await else {
        return (StatusCode::UNAUTHORIZED, Authorized::Unauthorized.into());
    };

    (
        StatusCode::OK,
        Authorized::Success(GetWhitelistResponseAuthed::new(Db::get_whitelist(user_id))).into(),
    )
}
