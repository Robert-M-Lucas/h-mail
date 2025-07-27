use crate::root::DB;
use crate::root::receiving::auth_util::auth_header::AuthorizationHeader;
use crate::root::receiving::interface::auth::Authorized;
use crate::root::receiving::interface::routes::native::get_emails::{
    GetEmailsRequest, GetEmailsResponse, GetEmailsResponseAuthed,
};
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;

pub async fn get_emails(
    auth_header: AuthorizationHeader,
    Query(get_emails): Query<GetEmailsRequest>,
) -> (StatusCode, Json<GetEmailsResponse>) {
    let Ok(user_id) = auth_header.check_access_token().await else {
        return (StatusCode::UNAUTHORIZED, Authorized::Unauthorized.into());
    };

    let emails = DB
        .lock()
        .await
        .as_ref()
        .unwrap()
        .get_emails(user_id, get_emails.since_id());

    (
        StatusCode::OK,
        Authorized::Success(GetEmailsResponseAuthed(emails)).into(),
    )
}
