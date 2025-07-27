use crate::root::config::DOMAIN;
use crate::root::receiving::auth_util::auth_header::AuthorizationHeader;
use crate::root::receiving::interface::auth::Authorized;
use crate::root::receiving::interface::fields::auth_token::AuthTokenDataField;
use crate::root::receiving::interface::routes::foreign::deliver_email::{
    DeliverEmailRequest, DeliverEmailResponse,
};
use crate::root::receiving::interface::routes::native::send_email::{
    SendEmailRequest, SendEmailResponse, SendEmailResponseAuthed,
};
use crate::root::sending::send_post::send_post;
use crate::root::shared_resources::{DB, VERIFY_IP_TOKEN_PROVIDER};
use axum::Json;
use axum::http::StatusCode;

pub async fn send_email(
    auth_header: AuthorizationHeader,
    Json(send_email): Json<SendEmailRequest>,
) -> (StatusCode, Json<SendEmailResponse>) {
    let Ok(user_id) = auth_header.check_access_token().await else {
        return (StatusCode::UNAUTHORIZED, Authorized::Unauthorized.into());
    };

    let username = DB
        .lock()
        .await
        .as_ref()
        .unwrap()
        .get_username_from_id(user_id)
        .unwrap();

    let (package, destination_domain) = send_email.dissolve();

    match send_post::<_, _, DeliverEmailResponse>(
        format!("https://{}:8081/foreign/deliver_email", &destination_domain),
        &DeliverEmailRequest::new(
            package,
            username,
            DOMAIN.to_string(),
            AuthTokenDataField::new(&VERIFY_IP_TOKEN_PROVIDER.write().await.get_token(())),
        ),
    )
    .await
    {
        Ok(r) => (
            StatusCode::OK,
            Authorized::Success(SendEmailResponseAuthed::DeliverResponse(r)).into(),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Authorized::Success(SendEmailResponseAuthed::SendingFailed).into(),
        ),
    }
}
