use crate::root::receiving::interface::get_emails::{GetEmailsRequest, GetEmailsResponse};
use crate::root::DB;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;

pub async fn get_emails(
    Query(get_emails): Query<GetEmailsRequest>,
) -> (StatusCode, Json<GetEmailsResponse>) {
    let emails = DB
        .lock()
        .await
        .as_ref()
        .unwrap()
        .get_emails(get_emails.username(), get_emails.since_id());

    if emails.is_some() {
        (StatusCode::OK, GetEmailsResponse::new(emails).into())
    } else {
        (StatusCode::NOT_FOUND, GetEmailsResponse::new(emails).into())
    }
}
