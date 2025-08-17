use crate::config::config_file::CONFIG;
use crate::database::Db;
use crate::receiving::auth_util::auth_header::AuthorizationHeader;
use crate::sending::send_post::send_post;
use crate::shared_resources::VERIFY_IP_TOKEN_PROVIDER;
use axum::Json;
use axum::http::StatusCode;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::fields::auth_token::AuthTokenDataField;
use h_mail_interface::interface::fields::hmail_address::HmailAddress;
use h_mail_interface::interface::routes::foreign::get_user_pow_policy_interserver::{
    FOREIGN_GET_USER_POW_POLICY_INTERSERVER_PATH, GetUserPowPolicyInterserverRequest,
    GetUserPowPolicyInterserverResponse,
};
use h_mail_interface::interface::routes::native::get_user_pow_policy::{
    GetUserPowPolicyRequest, GetUserPowPolicyResponse, GetUserPowPolicyResponseAuthed,
};
use h_mail_interface::utility::get_url_for_path;

pub async fn get_user_pow_policy(
    auth_header: AuthorizationHeader,
    Json(is_whitelisted): Json<GetUserPowPolicyRequest>,
) -> (StatusCode, Json<GetUserPowPolicyResponse>) {
    let Some(user_id) = auth_header.check_access_token().await else {
        return (StatusCode::UNAUTHORIZED, Authorized::Unauthorized.into());
    };

    let username = Db::get_username_from_id(user_id).unwrap();

    let recipient = is_whitelisted.recipient();

    // ! Do not lock resource
    let verify_ip_token = VERIFY_IP_TOKEN_PROVIDER.write().await.get_token(());

    match send_post::<_, _, GetUserPowPolicyInterserverResponse>(
        get_url_for_path(
            recipient.domain(),
            FOREIGN_GET_USER_POW_POLICY_INTERSERVER_PATH,
        ),
        &GetUserPowPolicyInterserverRequest::new(
            recipient.username().to_string(),
            HmailAddress::from_username_domain(&username, &CONFIG.domain).unwrap(),
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
