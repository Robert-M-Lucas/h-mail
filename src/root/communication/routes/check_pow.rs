use crate::root::POW_PROVIDER;
use crate::root::communication::interface::check_pow::CheckPow;
use crate::root::communication::interface::check_pow::CheckPowStatus;
use crate::root::communication::interface::shared::PowFailureReason;
use crate::root::shared::base64_to_big_uint;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;

pub async fn check_pow(Query(pow_request): Query<CheckPow>) -> (StatusCode, Json<CheckPowStatus>) {
    let Ok(token) = base64_to_big_uint(pow_request.token()) else {
        return (
            StatusCode::BAD_REQUEST,
            CheckPowStatus::Failure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };
    let Ok(challenge) = base64_to_big_uint(pow_request.challenge()) else {
        return (
            StatusCode::BAD_REQUEST,
            CheckPowStatus::Failure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };
    let Ok(result) = base64_to_big_uint(pow_request.result()) else {
        return (
            StatusCode::BAD_REQUEST,
            CheckPowStatus::Failure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };

    let result =
        POW_PROVIDER
            .write()
            .await
            .check_pow(token, pow_request.iters(), challenge, result);

    match result {
        Ok(_) => (StatusCode::OK, CheckPowStatus::Success.into()),
        Err(e) => (
            StatusCode::EXPECTATION_FAILED,
            CheckPowStatus::Failure(e).into(),
        ),
    }
}
