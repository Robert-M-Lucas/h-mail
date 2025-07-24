use crate::root::receiving::interface::send_email::{DeliverEmailRequest, DeliverEmailResponse};
use crate::root::receiving::interface::shared::PowFailureReason;
use crate::root::shared::base64_to_big_uint;
use crate::root::shared::hash_email;
use axum::Json;
use axum::http::StatusCode;
use crate::root::shared_resources::{DB, POW_PROVIDER};

pub async fn deliver_email(Json(send_email): Json<DeliverEmailRequest>) -> (StatusCode, Json<DeliverEmailResponse>) {
    let Ok(token) = send_email.token().decode() else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::PowFailure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };
    let Ok(hash_result) = send_email.hash_result().decode() else {
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

    let Some(classification) = policy.classify(send_email.iters()) else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::DoesNotMeetPolicy(policy).into(),
        );
    };

    let hash = hash_email(send_email.email());

    if let Err(e) = POW_PROVIDER
        .write()
        .await
        .check_pow(token, send_email.iters(), hash, hash_result)
        .await
    {
        return (
            StatusCode::EXPECTATION_FAILED,
            DeliverEmailResponse::PowFailure(e).into(),
        );
    }

    if !DB.lock().await.as_ref().unwrap().deliver_email(
        send_email.destination(),
        send_email.source(),
        send_email.email(),
        classification,
    ) {
        return (
            StatusCode::EXPECTATION_FAILED,
            DeliverEmailResponse::UserNotFound.into(),
        );
    }

    (StatusCode::OK, DeliverEmailResponse::Success.into())
}
