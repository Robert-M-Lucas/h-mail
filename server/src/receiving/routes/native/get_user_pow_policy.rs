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
    FOREIGN_IS_WHITELISTED_INTERSERVER_PATH, GetUserPowPolicyInterserverRequest,
    GetUserPowPolicyInterserverResponse,
};
use h_mail_interface::interface::routes::native::get_user_pow_policy::{GetUserPowPolicyRequest, GetUserPowPolicyResponse, GetUserPowPolicyResponseAuthed};
use h_mail_interface::interface::routes::native::is_whitelisted::{
    GetUserPowPolicyRequest, GetUserPowPolicyResponse, GetUserPowPolicyResponseAuthed,
};
use h_mail_interface::shared::get_url_for_path;

pub async fn get_user_pow_policy(
    auth_header: AuthorizationHeader,
    Json(is_whitelisted): Json<GetUserPowPolicyRequest>,
) -> (StatusCode, Json<GetUserPowPolicyResponse>) {
    let Some(user_id) = auth_header.check_access_token().await else {
        return (StatusCode::UNAUTHORIZED, Authorized::Unauthorized.into());
    };

    let username = Db::get_username_from_id(user_id).unwrap();

    let bad_request = || {
        (
            StatusCode::BAD_REQUEST,
            Authorized::Success(GetUserPowPolicyResponseAuthed::BadRequest).into(),
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

    match send_post::<_, _, GetUserPowPolicyInterserverResponse>(
        get_url_for_path(domain, FOREIGN_IS_WHITELISTED_INTERSERVER_PATH),
        &GetUserPowPolicyInterserverRequest::new(
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
                GetUserPowPolicyInterserverResponse::Whitelisted(c) => {
                    GetUserPowPolicyResponseAuthed::Whitelisted(c)
                }
                GetUserPowPolicyInterserverResponse::NotWhitelisted(p) => {
                    GetUserPowPolicyResponseAuthed::NotWhitelisted(p)
                }
                GetUserPowPolicyInterserverResponse::SenderIpNotAuthed => {
                    GetUserPowPolicyResponseAuthed::RequestFailed
                }
                GetUserPowPolicyInterserverResponse::BadRequest => {
                    GetUserPowPolicyResponseAuthed::BadRequest
                }
            })
            .into(),
        ),
        Err(_) => (
            StatusCode::OK,
            Authorized::Success(GetUserPowPolicyResponseAuthed::RequestFailed).into(),
        ),
    }
}
