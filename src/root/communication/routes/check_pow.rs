use axum::http::StatusCode;
use axum::Json;
use crate::root::communication::interface::check_pow::CheckPow;
use crate::root::pow::PowCheck;
use crate::root::POW_PROVIDER;
use crate::root::shared::base64_to_big_uint;

pub async fn check_pow(Json(pow_request): Json<CheckPow>) -> (StatusCode, Json<PowCheck>) {
    let Ok(token) = base64_to_big_uint(pow_request.token()) else {
        return (StatusCode::BAD_REQUEST, PowCheck::BadRequestCanRetry.into())
    };
    let Ok(challenge) = base64_to_big_uint(pow_request.challenge()) else {
        return (StatusCode::BAD_REQUEST, PowCheck::BadRequestCanRetry.into())
    };
    let Ok(result) = base64_to_big_uint(pow_request.result()) else {
        return (StatusCode::BAD_REQUEST, PowCheck::BadRequestCanRetry.into())
    };
    
    let result = POW_PROVIDER.write().await.check_pow(
        token,
        pow_request.iters(),
        challenge,
        result,
    );
    
    match result {
        PowCheck::Success => (StatusCode::OK, result.into()),
        _ => (StatusCode::EXPECTATION_FAILED, result.into()),
    }
}

