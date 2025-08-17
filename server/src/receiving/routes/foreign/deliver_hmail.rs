use crate::config::config_file::CONFIG;
use crate::database::Db;
use crate::receiving::auth_util::spf_check::spf_check;
use crate::receiving::auth_util::verify_sender_ip;
use crate::shared_resources::POW_PROVIDER;
use axum::Json;
use axum::extract::ConnectInfo;
use axum::http::StatusCode;
use h_mail_interface::interface::pow::{PowFailureReason, PowHash};
use h_mail_interface::interface::routes::foreign::deliver_hmail::{
    DeliverHmailRequest, DeliverHmailResponse,
};
use std::net::SocketAddr;

pub async fn deliver_hmail(
    ConnectInfo(connect_info): ConnectInfo<SocketAddr>,
    Json(deliver_hmail): Json<DeliverHmailRequest>,
) -> (StatusCode, Json<DeliverHmailResponse>) {
    let (hmail_package, sender_address, recipient_address, verify_ip, verify_ip_port) =
        deliver_hmail.dissolve();

    if recipient_address.domain() != CONFIG.domain() {
        return (
            StatusCode::BAD_REQUEST,
            DeliverHmailResponse::WrongDomain.into(),
        );
    }

    let Ok(hmail_package) = hmail_package.decode() else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverHmailResponse::PowFailure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };

    let hash = hmail_package.pow_hash();

    let Ok(verify_ip_token) = verify_ip.token().decode() else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverHmailResponse::BadRequest.into(),
        );
    };

    let (classification, policy_minimum) = if let Some(whitelist_classification) =
        Db::user_whitelisted(recipient_address.username(), &sender_address)
    {
        (whitelist_classification, 0)
    } else {
        let Some(policy) = Db::get_user_pow_policy(recipient_address.username()) else {
            return (
                StatusCode::BAD_REQUEST,
                DeliverHmailResponse::UserNotFound.into(),
            );
        };

        // Check against policy
        let Some(classification) = policy.classify(
            hmail_package
                .pow_result()
                .as_ref()
                .map_or(0, |p| *p.iters()),
        ) else {
            return (
                StatusCode::BAD_REQUEST,
                DeliverHmailResponse::DoesNotMeetPolicy(policy).into(),
            );
        };
        (classification, *policy.minimum())
    };

    // Check POW token
    let hmail_package = match POW_PROVIDER
        .write()
        .await
        .check_pow(hmail_package, policy_minimum)
        .await
    {
        Ok(hmail_package) => {
            let Ok(hmail_package) = hmail_package.decode() else {
                return (
                    StatusCode::BAD_REQUEST,
                    DeliverHmailResponse::BadRequest.into(),
                );
            };
            hmail_package
        }
        Err(e) => {
            return (
                StatusCode::EXPECTATION_FAILED,
                DeliverHmailResponse::PowFailure(e).into(),
            );
        }
    };

    // Check that IP is not spoofed
    if !verify_sender_ip::verify_sender_ip(connect_info, verify_ip_port, &verify_ip_token).await {
        return (
            StatusCode::UNAUTHORIZED,
            DeliverHmailResponse::SenderIpNotAuthed.into(),
        );
    }

    // Check IP against DNS
    if !spf_check(
        connect_info,
        sender_address.username(),
        sender_address.domain(),
    )
    .await
    {
        return (
            StatusCode::UNAUTHORIZED,
            DeliverHmailResponse::SenderIpNotAuthed.into(),
        );
    }

    // Try deliver hmail (database)
    if Db::deliver_hmail(
        recipient_address.username(),
        &sender_address,
        hmail_package,
        &hash,
        classification,
    )
    .is_err()
    {
        return (
            StatusCode::EXPECTATION_FAILED,
            DeliverHmailResponse::UserNotFound.into(),
        );
    }

    (StatusCode::OK, DeliverHmailResponse::Success.into())
}
