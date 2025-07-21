use crate::root::communication::interface::send_email::{SendEmail, SendEmailStatus};
use crate::root::communication::interface::shared::PowFailureReason;
use crate::root::shared::base64_to_big_uint;
use crate::root::{DB, POW_PROVIDER};
use crate::root::shared::hash_email;
use axum::http::StatusCode;
use axum::Json;

pub async fn send_email(Json(send_email): Json<SendEmail>) -> (StatusCode, Json<SendEmailStatus>) {
    let Ok(token) = base64_to_big_uint(send_email.token()) else {
        return (StatusCode::BAD_REQUEST, SendEmailStatus::PowFailure(PowFailureReason::BadRequestCanRetry).into());
    };
    let Ok(hash_result) = base64_to_big_uint(send_email.hash_result()) else {
        return (StatusCode::BAD_REQUEST, SendEmailStatus::PowFailure(PowFailureReason::BadRequestCanRetry).into());
    };
    
    let Some(policy) = DB.lock().unwrap().get_user_pow_policy(send_email.destination()) else {
        return (StatusCode::BAD_REQUEST, SendEmailStatus::UserNotFound.into());
    };
    
    if policy.minimum() > send_email.iters() {
        return (StatusCode::BAD_REQUEST, SendEmailStatus::DoesNotMeetPolicy(policy).into());
    }
    
    let hash = hash_email(send_email.email());

    if let Err(e) = POW_PROVIDER
            .write()
            .await
            .check_pow(token, send_email.iters(), hash, hash_result) {
        return (StatusCode::EXPECTATION_FAILED, SendEmailStatus::PowFailure(e).into());
    }
    
    if !DB.lock().unwrap().deliver_email(send_email.destination(), send_email.email(), send_email.iters(), policy) {
        return (StatusCode::EXPECTATION_FAILED, SendEmailStatus::UserNotFound.into());
    }

    (StatusCode::OK, SendEmailStatus::Success.into())
}
