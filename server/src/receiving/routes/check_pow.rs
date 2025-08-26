use crate::shared_resources::POW_PROVIDER;
use axum::Json;
use axum::http::StatusCode;
use h_mail_interface::interface::pow::PowFailureReason;
use h_mail_interface::interface::routes::check_pow::{CheckPowRequest, CheckPowResponse};

pub async fn check_pow(
    Json(pow_request): Json<CheckPowRequest>,
) -> (StatusCode, Json<CheckPowResponse>) {
    let Ok(pow_request) = pow_request.decode() else {
        return (
            StatusCode::OK,
            CheckPowResponse::Failure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };

    let Some(pow_result) = pow_request.pow_result() else {
        return (
            StatusCode::OK,
            CheckPowResponse::Failure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };

    let min_iters = *pow_result.iters();

    let result = POW_PROVIDER
        .write()
        .await
        .check_pow(pow_request, min_iters)
        .await;

    match result {
        Ok(_) => (StatusCode::OK, CheckPowResponse::Success.into()),
        Err(e) => (StatusCode::OK, CheckPowResponse::Failure(e).into()),
    }
}
