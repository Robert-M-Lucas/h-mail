use crate::config::config_file::CONFIG;
use crate::database::Db;
use crate::receiving::auth_util::auth_header::AuthorizationHeader;
use crate::sending::send_post::send_post;
use crate::shared_resources::VERIFY_IP_TOKEN_PROVIDER;
use axum::Json;
use axum::http::StatusCode;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::fields::auth_token::AuthTokenDataField;
use h_mail_interface::interface::routes::foreign::is_whitelisted_interserver::{
    FOREIGN_IS_WHITELISTED_INTERSERVER_PATH, IsWhitelistedInterserverRequest,
    IsWhitelistedInterserverResponse,
};
use h_mail_interface::interface::routes::native::is_whitelisted::{
    IsWhitelistedRequest, IsWhitelistedResponse, IsWhitelistedResponseAuthed,
};
use h_mail_interface::shared::get_url_for_path;

pub async fn send_email(
    auth_header: AuthorizationHeader,
    Json(is_whitelisted): Json<IsWhitelistedRequest>,
) -> (StatusCode, Json<IsWhitelistedResponse>) {
    let Some(user_id) = auth_header.check_access_token().await else {
        return (StatusCode::UNAUTHORIZED, Authorized::Unauthorized.into());
    };

    let username = Db::get_username_from_id(user_id).unwrap();

    let bad_request = || {
        (
            StatusCode::BAD_REQUEST,
            Authorized::Success(IsWhitelistedResponseAuthed::BadRequest).into(),
        )
    };

    let mut recipient = is_whitelisted.recipient().split("@");
    let Some(_user) = recipient.next() else {
        return bad_request();
    };
    let Some(domain) = recipient.next() else {
        return bad_request();
    };
    if recipient.next().is_some() {
        return bad_request();
    }

    // ! Do not lock resource
    let verify_ip_token = VERIFY_IP_TOKEN_PROVIDER.write().await.get_token(());

    match send_post::<_, _, IsWhitelistedInterserverResponse>(
        get_url_for_path(domain, FOREIGN_IS_WHITELISTED_INTERSERVER_PATH),
        &IsWhitelistedInterserverRequest::new(
            is_whitelisted.recipient().clone(),
            format!("{username}@{}", CONFIG.domain()),
            AuthTokenDataField::new(&verify_ip_token),
            CONFIG.port(),
        ),
    )
    .await
    {
        Ok(r) => (
            StatusCode::OK,
            Authorized::Success(match r {
                IsWhitelistedInterserverResponse::Whitelisted => {
                    IsWhitelistedResponseAuthed::Whitelisted
                }
                IsWhitelistedInterserverResponse::NotWhitelisted(p) => {
                    IsWhitelistedResponseAuthed::NotWhitelisted(p)
                }
                IsWhitelistedInterserverResponse::SenderIpNotAuthed => {
                    IsWhitelistedResponseAuthed::RequestFailed
                }
                IsWhitelistedInterserverResponse::BadRequest => {
                    IsWhitelistedResponseAuthed::BadRequest
                }
            })
            .into(),
        ),
        Err(_) => (
            StatusCode::OK,
            Authorized::Success(IsWhitelistedResponseAuthed::RequestFailed).into(),
        ),
    }
}
