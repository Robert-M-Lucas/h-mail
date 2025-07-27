use crate::root::receiving::interface::routes::check_pow::CheckPowRequest;
use crate::root::receiving::interface::routes::check_pow::CheckPowResponse;
use crate::root::receiving::interface::shared::PowFailureReason;
use crate::root::shared_resources::POW_PROVIDER;
use axum::Json;
use axum::http::StatusCode;

pub async fn check_pow(
    Json(pow_request): Json<CheckPowRequest>,
) -> (StatusCode, Json<CheckPowResponse>) {
    let Ok(token) = pow_request.token().decode() else {
        return (
            StatusCode::BAD_REQUEST,
            CheckPowResponse::Failure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };
    let Ok(challenge) = pow_request.challenge().decode() else {
        return (
            StatusCode::BAD_REQUEST,
            CheckPowResponse::Failure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };
    let Ok(result) = pow_request.result().decode() else {
        return (
            StatusCode::BAD_REQUEST,
            CheckPowResponse::Failure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };

    let result = POW_PROVIDER
        .write()
        .await
        .check_pow(token, pow_request.iters(), challenge, result)
        .await;

    match result {
        Ok(_) => (StatusCode::OK, CheckPowResponse::Success.into()),
        Err(e) => (
            StatusCode::EXPECTATION_FAILED,
            CheckPowResponse::Failure(e).into(),
        ),
    }
}
