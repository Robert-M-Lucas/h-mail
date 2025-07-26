use crate::root::receiving::interface::deliver_email::{DeliverEmailRequest, DeliverEmailResponse};
use crate::root::receiving::interface::shared::PowFailureReason;
use crate::root::shared::hash_str;
use crate::root::shared_resources::{DB, POW_PROVIDER};
use axum::Json;
use axum::http::StatusCode;
use mail_auth::spf::verify::SpfParameters;
use mail_auth::{MessageAuthenticator, SpfResult};

pub async fn deliver_email(
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

    // Check POW token and retrieve associated IP
    let hash = hash_str(send_email.email());
    let ip_addr = match POW_PROVIDER
        .write()
        .await
        .check_pow(token, send_email.iters(), hash, pow_result)
        .await
    {
        Ok(ip_addr) => ip_addr,
        Err(e) => {
            return (
                StatusCode::EXPECTATION_FAILED,
                DeliverEmailResponse::PowFailure(e).into(),
            );
        }
    };

    // Check IP against DNS
    let authenticator = MessageAuthenticator::new_google().unwrap();
    let sender = format!(
        "{}@{}",
        send_email.source_user(),
        send_email.source_domain()
    );
    let result = authenticator
        .verify_spf(SpfParameters::verify_mail_from(ip_addr, "", "", &sender))
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
