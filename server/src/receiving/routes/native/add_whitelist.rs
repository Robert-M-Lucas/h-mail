use crate::database::Db;
use crate::receiving::auth_util::auth_header::AuthorizationHeader;
use axum::Json;
use axum::http::StatusCode;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::routes::native::add_whitelist::{
    AddWhitelistRequest, AddWhitelistResponse, AddWhitelistResponseAuthed,
};

pub async fn add_whitelist(
    auth_header: AuthorizationHeader,
    Json(add_whitelist): Json<AddWhitelistRequest>,
) -> (StatusCode, Json<AddWhitelistResponse>) {
    let Some(user_id) = auth_header.check_access_token().await else {
        return (StatusCode::UNAUTHORIZED, Authorized::Unauthorized.into());
    };

    Db::add_whitelist(
        user_id,
        add_whitelist.address(),
        *add_whitelist.place_into(),
    );

    (
        StatusCode::OK,
        Authorized::Success(AddWhitelistResponseAuthed::Success).into(),
    )
}
