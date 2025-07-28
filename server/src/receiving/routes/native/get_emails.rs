use crate::DB;
use crate::receiving::auth_util::auth_header::AuthorizationHeader;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::routes::native::get_emails::{
    GetEmailsRequest, GetEmailsResponse, GetEmailsResponseAuthed,
};

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
