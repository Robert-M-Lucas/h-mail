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
    let (hmail_package, recipient_address, verify_ip, verify_ip_port, context) =
        deliver_hmail.dissolve();

    let Ok(hmail_package) = hmail_package.decode() else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverHmailResponse::PowFailure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };

    let mut context_decoded = Vec::new();
    for context in context {
        let hash = context.pow_hash();
        let Ok(context) = context.decode() else {
            return (
                StatusCode::BAD_REQUEST,
                DeliverHmailResponse::PowFailure(PowFailureReason::BadRequestCanRetry).into(),
            );
        };
        context_decoded.push((context, hash));
    }

    let hash = hmail_package.pow_hash();

    let Ok(verify_ip_token) = verify_ip.token().decode() else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverHmailResponse::BadRequest.into(),
        );
    };

    let whitelist_classification = Db::user_whitelisted(
        recipient_address.username(),
        hmail_package.inner_dangerous().sender().address(),
    )
    .await;

    let Some(policy) = Db::get_user_pow_policy(recipient_address.username()).await else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverHmailResponse::UserNotFound.into(),
        );
    };

    let (classification, policy_minimum) = if let Some(policy_classification) = policy.classify(
        hmail_package
            .pow_result()
            .as_ref()
            .map_or(0, |p| *p.iters()),
    ) {
        if let Some(whitelist_classification) = whitelist_classification {
            if policy_classification > whitelist_classification {
                (policy_classification, *policy.minimum())
            } else {
                (whitelist_classification, 0)
            }
        } else {
            (policy_classification, *policy.minimum())
        }
    } else if let Some(whitelist_classification) = whitelist_classification {
        (whitelist_classification, 0)
    } else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverHmailResponse::DoesNotMeetPolicy(policy).into(),
        );
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

    let sender_user = hmail_package.sender();

    // Check IP against DNS
    if !spf_check(
        connect_info,
        sender_user.address().username(),
        sender_user.address().domain(),
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
        hmail_package,
        &hash,
        classification,
        context_decoded,
        false
    )
    .await
    .is_err()
    {
        return (
            StatusCode::EXPECTATION_FAILED,
            DeliverHmailResponse::UserNotFound.into(),
        );
    }

    (StatusCode::OK, DeliverHmailResponse::Success.into())
}
