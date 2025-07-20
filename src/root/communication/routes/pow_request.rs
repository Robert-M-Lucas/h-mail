use axum::http::StatusCode;
use axum::Json;
use crate::root::communication::interface::pow_request::{PowRequest, PowResponse, PowResponseData, PowTokenSend};
use crate::root::{DB, POW_PROVIDER};
use crate::root::shared::{big_uint_to_base64, system_time_to_ms_since_epoch};

pub async fn pow_request(Json(pow_request): Json<PowRequest>) -> (StatusCode, Json<PowResponse>) {
    let policy = DB.lock().unwrap().get_user_pow_policy(&pow_request.destination());

    if let Some(policy) = policy {
        let token = POW_PROVIDER.write().await.get_token();
        let token = PowTokenSend::new(
            big_uint_to_base64(token.token()),
            system_time_to_ms_since_epoch(token.expires_at()),
        );
        
        (StatusCode::FOUND, Json(PowResponse::new(Some(PowResponseData::new(
            policy,
            token,
            )))))
    }
    else {
        (StatusCode::NOT_FOUND, Json(PowResponse::new(None)))
    }
}

