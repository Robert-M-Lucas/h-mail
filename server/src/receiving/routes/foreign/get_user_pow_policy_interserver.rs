use crate::database::Db;
use crate::receiving::auth_util::spf_check::spf_check;
use crate::receiving::auth_util::verify_sender_ip;
use axum::Json;
use axum::extract::ConnectInfo;
use axum::http::StatusCode;
use h_mail_interface::interface::routes::foreign::get_user_pow_policy_interserver::{
    GetUserPowPolicyInterserverRequest, GetUserPowPolicyInterserverResponse, WhitelistedResponse,
};
use std::net::SocketAddr;

pub async fn get_user_pow_policy_interserver(
    ConnectInfo(connect_info): ConnectInfo<SocketAddr>,
    Json(is_whitelisted_interserver): Json<GetUserPowPolicyInterserverRequest>,
) -> (StatusCode, Json<GetUserPowPolicyInterserverResponse>) {
    let bad_request = || {
        (
            StatusCode::BAD_REQUEST,
            GetUserPowPolicyInterserverResponse::BadRequest.into(),
        )
    };

    let Ok(ip_verification) = is_whitelisted_interserver
        .ip_verification()
        .token()
        .decode()
    else {
        return bad_request();
    };

    // Check that IP is not spoofed
    if !verify_sender_ip::verify_sender_ip(
        connect_info,
        is_whitelisted_interserver.verify_ip_port(),
        &ip_verification,
    )
    .await
    {
        return (
            StatusCode::UNAUTHORIZED,
            GetUserPowPolicyInterserverResponse::SenderIpNotAuthed.into(),
        );
    }

    let sender = is_whitelisted_interserver.sender();

    // Check IP against DNS
    if !spf_check(connect_info, sender.username(), sender.domain()).await {
        return (
            StatusCode::UNAUTHORIZED,
            GetUserPowPolicyInterserverResponse::SenderIpNotAuthed.into(),
        );
    }

    let Some(pow_policy) = Db::get_user_pow_policy(is_whitelisted_interserver.recipient_username())
    else {
        return (
            StatusCode::OK,
            GetUserPowPolicyInterserverResponse::UserDoesNotExist.into(),
        );
    };
    if let Some(classification) = Db::user_whitelisted(
        is_whitelisted_interserver.recipient_username(),
        is_whitelisted_interserver.sender(),
    ) {
        (
            StatusCode::OK,
            GetUserPowPolicyInterserverResponse::Whitelisted(WhitelistedResponse::new(
                classification,
                pow_policy,
            ))
            .into(),
        )
    } else {
        (
            StatusCode::OK,
            GetUserPowPolicyInterserverResponse::NotWhitelisted(pow_policy).into(),
        )
    }
}
