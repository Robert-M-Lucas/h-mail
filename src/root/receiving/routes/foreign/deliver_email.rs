use crate::root::receiving::interface::pow::PowFailureReason;
use crate::root::receiving::interface::routes::foreign::deliver_email::{
    DeliverEmailRequest, DeliverEmailResponse,
};
use crate::root::sending;
use crate::root::shared_resources::{DB, POW_PROVIDER};
use axum::Json;
use axum::extract::ConnectInfo;
use axum::http::StatusCode;
use std::net::SocketAddr;

pub async fn deliver_email(
    ConnectInfo(connect_info): ConnectInfo<SocketAddr>,
    Json(send_email): Json<DeliverEmailRequest>,
) -> (StatusCode, Json<DeliverEmailResponse>) {
    let Ok(token) = send_email.token().decode() else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::PowFailure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };
    let Ok(pow_result) = send_email.pow_result().decode() else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::PowFailure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };
    let Ok(verify_ip_token) = send_email.verify_ip().token().decode() else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::BadRequest.into(),
        );
    };

    let Some(policy) = DB
        .lock()
        .await
        .as_ref()
        .unwrap()
        .get_user_pow_policy(send_email.destination())
    else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::UserNotFound.into(),
        );
    };

    // Check against policy
    let Some(classification) = policy.classify(send_email.iters()) else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::DoesNotMeetPolicy(policy).into(),
        );
    };

    if !sending::verify_ip::verify_ip(connect_info.ip(), &verify_ip_token).await {
        return (
            StatusCode::UNAUTHORIZED,
            DeliverEmailResponse::SenderIpNotAuthed.into(),
        );
    }

    // Check POW token
    let hash = send_email.email().hash();
    if let Err(e) = POW_PROVIDER
        .write()
        .await
        .check_pow(token, send_email.iters(), hash, pow_result)
        .await
    {
        return (
            StatusCode::EXPECTATION_FAILED,
            DeliverEmailResponse::PowFailure(e).into(),
        );
    };

    // Check IP against DNS
    #[cfg(not(feature = "no_spf"))]
    {
        let authenticator = MessageAuthenticator::new_google().unwrap();
        let sender = format!(
            "{}@{}",
            send_email.source_user(),
            send_email.source_domain()
        );
        let result = authenticator
            .verify_spf(SpfParameters::verify_mail_from(
                connect_info.ip(),
                "",
                "",
                &sender,
            ))
            .await;

        match result.result() {
            SpfResult::Pass => {}
            _ => {
                return (
                    StatusCode::UNAUTHORIZED,
                    DeliverEmailResponse::SenderIpNotAuthed.into(),
                );
            }
        }
    }

    // Try deliver email (database)
    if DB
        .lock()
        .await
        .as_ref()
        .unwrap()
        .deliver_email(
            send_email.destination(),
            send_email.source_user(),
            send_email.source_domain(),
            send_email.email(),
            classification,
        )
        .is_ok()
    {
        return (
            StatusCode::EXPECTATION_FAILED,
            DeliverEmailResponse::UserNotFound.into(),
        );
    }

    (StatusCode::OK, DeliverEmailResponse::Success.into())
}
