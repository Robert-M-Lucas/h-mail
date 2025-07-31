use crate::shared_resources::VERIFY_IP_TOKEN_PROVIDER;
use axum::Json;
use axum::http::StatusCode;
use h_mail_interface::interface::routes::foreign::verify_ip::{VerifyIpRequest, VerifyIpResponse};

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
        Some(_) => (StatusCode::OK, VerifyIpResponse::Success.into()),
        None => (StatusCode::UNAUTHORIZED, VerifyIpResponse::Failure.into()),
    }
}
