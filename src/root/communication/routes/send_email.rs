use crate::root::communication::interface::send_email::{SendEmailRequest, SendEmailResponse};
use crate::root::communication::interface::shared::PowFailureReason;
use crate::root::shared::base64_to_big_uint;
use crate::root::shared::hash_email;
use axum::Json;
use axum::http::StatusCode;
use crate::root::shared_resources::{DB, POW_PROVIDER};

pub async fn send_email(Json(send_email): Json<SendEmailRequest>) -> (StatusCode, Json<SendEmailResponse>) {
    let Ok(token) = base64_to_big_uint(send_email.token()) else {
        return (
            StatusCode::BAD_REQUEST,
            SendEmailResponse::PowFailure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };
    let Ok(hash_result) = base64_to_big_uint(send_email.hash_result()) else {
        return (
            StatusCode::BAD_REQUEST,
            SendEmailResponse::PowFailure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };

    let Some(policy) = DB
        .lock()
        .unwrap()
        .get_user_pow_policy(send_email.destination())
    else {
        return (
            StatusCode::BAD_REQUEST,
            SendEmailResponse::UserNotFound.into(),
        );
    };

    if policy.minimum() > send_email.iters() {
        return (
            StatusCode::BAD_REQUEST,
            SendEmailResponse::DoesNotMeetPolicy(policy).into(),
        );
    }

    let hash = hash_email(send_email.email());

    if let Err(e) =
        POW_PROVIDER
            .write()
            .await
            .check_pow(token, send_email.iters(), hash, hash_result)
    {
        return (
            StatusCode::EXPECTATION_FAILED,
            SendEmailResponse::PowFailure(e).into(),
        );
    }

    if !DB.lock().unwrap().deliver_email(
        send_email.destination(),
        send_email.email(),
        send_email.iters(),
        policy,
    ) {
        return (
            StatusCode::EXPECTATION_FAILED,
            SendEmailResponse::UserNotFound.into(),
        );
    }

    (StatusCode::OK, SendEmailResponse::Success.into())
}
