use crate::args::ARGS;
use crate::config::DOMAIN;
use crate::database::Db;
use crate::receiving::auth_util::auth_header::AuthorizationHeader;
use crate::sending::send_post::send_post;
use crate::shared_resources::VERIFY_IP_TOKEN_PROVIDER;
use axum::Json;
use axum::http::StatusCode;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::fields::auth_token::AuthTokenDataField;
use h_mail_interface::interface::routes::foreign::deliver_email::{
    DeliverEmailRequest, DeliverEmailResponse,
};
use h_mail_interface::interface::routes::native::send_email::{
    SendEmailRequest, SendEmailResponse, SendEmailResponseAuthed,
};
use std::io::{Write, stdout};

pub async fn send_email(
    auth_header: AuthorizationHeader,
    Json(send_email): Json<SendEmailRequest>,
) -> (StatusCode, Json<SendEmailResponse>) {
    let Some(user_id) = auth_header.check_access_token().await else {
        return (StatusCode::UNAUTHORIZED, Authorized::Unauthorized.into());
    };

    let username = Db::get_username_from_id(user_id).unwrap();

    let (email, destination_domain) = send_email.dissolve();

    // ! Do not lock resource
    let verify_ip_token = VERIFY_IP_TOKEN_PROVIDER.write().await.get_token(());

    stdout().flush().ok();
    match send_post::<_, _, DeliverEmailResponse>(
        format!("https://{}/foreign/deliver_email", &destination_domain),
        &DeliverEmailRequest::new(
            email,
            username,
            DOMAIN.to_string(),
            AuthTokenDataField::new(&verify_ip_token),
            ARGS.port(),
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
