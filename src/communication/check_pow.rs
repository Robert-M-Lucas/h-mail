use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::database::PowPolicy;
use crate::{DB, POW_PROVIDER};
use crate::pow::{PowCheck, PowToken};
use crate::shared::base64_to_big_uint;

pub async fn check_pow(Json(pow_request): Json<CheckPow>) -> (StatusCode, Json<PowCheck>) {
    let Ok(token) = base64_to_big_uint(&pow_request.token) else {
        return (StatusCode::BAD_REQUEST, PowCheck::BadRequestCanRetry.into())
    };
    let Ok(challenge) = base64_to_big_uint(&pow_request.challenge) else {
        return (StatusCode::BAD_REQUEST, PowCheck::BadRequestCanRetry.into())
    };
    let Ok(result) = base64_to_big_uint(&pow_request.result) else {
        return (StatusCode::BAD_REQUEST, PowCheck::BadRequestCanRetry.into())
    };
    
    let result = POW_PROVIDER.write().await.check_pow(
        token,
        pow_request.iters,
        challenge,
        result,
    );
    
    match result {
        PowCheck::Success => (StatusCode::OK, result.into()),
        _ => (StatusCode::EXPECTATION_FAILED, result.into()),
    }
}

#[derive(Deserialize)]
pub struct CheckPow {
    token: String,
    iters: u64,
    challenge: String,
    result: String,
}
