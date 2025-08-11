use crate::config::config_file::CONFIG;
use crate::database::Db;
use crate::receiving::auth_util::spf_check::spf_check;
use crate::receiving::auth_util::verify_sender_ip;
use crate::shared_resources::POW_PROVIDER;
use axum::Json;
use axum::extract::ConnectInfo;
use axum::http::StatusCode;
use h_mail_interface::interface::pow::{PowFailureReason, PowHash};
use h_mail_interface::interface::routes::foreign::deliver_email::{
    DeliverEmailRequest, DeliverEmailResponse,
};
use std::net::SocketAddr;

pub async fn deliver_email(
    ConnectInfo(connect_info): ConnectInfo<SocketAddr>,
    Json(deliver_email): Json<DeliverEmailRequest>,
) -> (StatusCode, Json<DeliverEmailResponse>) {
    let (
        email_package,
        source_user,
        source_domain,
        destination_user,
        destination_domain,
        verify_ip,
        verify_ip_port,
    ) = deliver_email.dissolve();

    if &destination_domain != CONFIG.domain() {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::WrongDomain.into(),
        );
    }

    let Ok(email_package) = email_package.decode() else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::PowFailure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };

    let hash = email_package.pow_hash();

    let Ok(verify_ip_token) = verify_ip.token().decode() else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::BadRequest.into(),
        );
    };

    let Some(policy) = Db::get_user_pow_policy(&destination_user) else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::UserNotFound.into(),
        );
    };

    // Check against policy
    let Some(classification) = policy.classify(*email_package.pow_result().iters()) else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::DoesNotMeetPolicy(policy).into(),
        );
    };

    // Check POW token
    let email_package = match POW_PROVIDER
        .write()
        .await
        .check_pow(email_package, *policy.minimum())
        .await
    {
        Ok(email_package) => {
            let Ok(email_package) = email_package.decode() else {
                return (
                    StatusCode::BAD_REQUEST,
                    DeliverEmailResponse::BadRequest.into(),
                );
            };
            email_package
        }
        Err(e) => {
            return (
                StatusCode::EXPECTATION_FAILED,
                DeliverEmailResponse::PowFailure(e).into(),
            );
        }
    };

    // Check that IP is not spoofed
    if !verify_sender_ip::verify_sender_ip(connect_info, verify_ip_port, &verify_ip_token).await {
        return (
            StatusCode::UNAUTHORIZED,
            DeliverEmailResponse::SenderIpNotAuthed.into(),
        );
    }

    // Check IP against DNS
    if !spf_check(connect_info, &source_user, &source_domain).await {
        return (
            StatusCode::UNAUTHORIZED,
            DeliverEmailResponse::SenderIpNotAuthed.into(),
        );
    }

    // Try deliver email (database)
    if Db::deliver_email(
        &destination_user,
        &source_user,
        &source_domain,
        email_package,
        &hash,
        classification,
    )
    .is_err()
    {
        return (
            StatusCode::EXPECTATION_FAILED,
            DeliverEmailResponse::UserNotFound.into(),
        );
    }

    (StatusCode::OK, DeliverEmailResponse::Success.into())
}
