use crate::root::receiving::interface::routes::foreign::verify_ip::{
    VerifyIpRequest, VerifyIpResponse,
};
use crate::root::shared_resources::VERIFY_IP_TOKEN_PROVIDER;
use axum::Json;
use axum::http::StatusCode;

pub async fn verify_ip(
    Json(verify_ip_request): Json<VerifyIpRequest>,
) -> (StatusCode, Json<VerifyIpResponse>) {
    let Ok(token) = verify_ip_request.ip_verification().decode() else {
        return (StatusCode::BAD_REQUEST, VerifyIpResponse::BadRequest.into());
    };

    match VERIFY_IP_TOKEN_PROVIDER
        .write()
        .await
        .validate_token(&token)
    {
        Ok(_) => (StatusCode::OK, VerifyIpResponse::Success.into()),
        Err(_) => (StatusCode::UNAUTHORIZED, VerifyIpResponse::Failure.into()),
    }
}
