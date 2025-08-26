use crate::sending::send_post::send_post;
use h_mail_interface::interface::auth::AuthToken;
use h_mail_interface::interface::fields::auth_token::AuthTokenField;
use h_mail_interface::interface::routes::foreign::verify_ip::{
    FOREIGN_VERIFY_IP_PATH, VerifyIpRequest, VerifyIpResponse,
};
use h_mail_interface::utility::get_url_for_path;
use std::net::SocketAddr;
use h_mail_interface::interface::fields::hmail_address::HmailAddress;

pub async fn verify_sender_ip(
    connect_info: SocketAddr,
    verify_ip_port: u16,
    verify_ip_token: &AuthToken,
    recipient: HmailAddress
) -> bool {
    matches!(
        send_post::<_, _, VerifyIpResponse>(
            get_url_for_path(
                format!("{}:{}", connect_info.ip(), verify_ip_port),
                FOREIGN_VERIFY_IP_PATH,
            ),
            &VerifyIpRequest::new(AuthTokenField::new(verify_ip_token), recipient),
        )
        .await,
        Ok(VerifyIpResponse::Success)
    )
}
