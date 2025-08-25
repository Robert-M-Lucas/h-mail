use crate::database::Db;
use crate::receiving::auth_util::auth_header::AuthorizationHeader;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::routes::native::get_hmails::{
    GetHmailsRequest, GetHmailsResponse, GetHmailsResponseAuthed,
};

pub async fn get_hmails(
    auth_header: AuthorizationHeader,
    Query(get_hmails): Query<GetHmailsRequest>,
) -> (StatusCode, Json<GetHmailsResponse>) {
    let Some(user_id) = auth_header.check_access_token().await else {
        return (StatusCode::UNAUTHORIZED, Authorized::Unauthorized.into());
    };

    let hmails = Db::get_hmails(user_id, *get_hmails.until(), get_hmails.limit()).await;

    (
        StatusCode::OK,
        Authorized::Success(GetHmailsResponseAuthed::new(hmails)).into(),
    )
}
