use crate::database::Db;
use crate::receiving::auth_util::spf_check::spf_check;
use crate::receiving::auth_util::verify_sender_ip;
use axum::Json;
use axum::extract::ConnectInfo;
use axum::http::StatusCode;
use h_mail_interface::interface::routes::foreign::is_whitelisted_interserver::{
    IsWhitelistedInterserverRequest, IsWhitelistedInterserverResponse,
};
use std::net::SocketAddr;

pub async fn is_whitelisted_interserver(
    ConnectInfo(connect_info): ConnectInfo<SocketAddr>,
    Json(is_whitelisted_interserver): Json<IsWhitelistedInterserverRequest>,
) -> (StatusCode, Json<IsWhitelistedInterserverResponse>) {
    let bad_request = || {
        (
            StatusCode::BAD_REQUEST,
            IsWhitelistedInterserverResponse::BadRequest.into(),
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
            IsWhitelistedInterserverResponse::SenderIpNotAuthed.into(),
        );
    }

    let mut sender = is_whitelisted_interserver.sender().split("@");
    let Some(user) = sender.next() else {
        return bad_request();
    };
    let Some(domain) = sender.next() else {
        return bad_request();
    };
    if sender.next().is_some() {
        return bad_request();
    }

    // Check IP against DNS
    if !spf_check(connect_info, user, domain).await {
        return (
            StatusCode::UNAUTHORIZED,
            IsWhitelistedInterserverResponse::SenderIpNotAuthed.into(),
        );
    }

    if let Some(classification) = Db::user_whitelisted(
        is_whitelisted_interserver.recipient(),
        is_whitelisted_interserver.sender(),
    ) {
        (
            StatusCode::OK,
            IsWhitelistedInterserverResponse::Whitelisted(classification).into(),
        )
    } else {
        let pow_policy = Db::get_user_pow_policy(user).unwrap();

        (
            StatusCode::OK,
            IsWhitelistedInterserverResponse::NotWhitelisted(pow_policy).into(),
        )
    }
}
