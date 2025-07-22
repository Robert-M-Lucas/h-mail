use crate::root::communication::interface::pow_request::{
    PowRequest, PowResponse, PowResponseData, PowTokenSend,
};
use crate::root::shared::{big_uint_to_base64, system_time_to_ms_since_epoch};
use crate::root::{DB, POW_PROVIDER, read_db};
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;

pub async fn pow_request(Query(pow_request): Query<PowRequest>) -> (StatusCode, Json<PowResponse>) {
    let policy = read_db!().get_user_pow_policy(pow_request.destination());
    if let Some(policy) = policy {
        let token = POW_PROVIDER.write().await.get_token();
        let token = PowTokenSend::new(
            big_uint_to_base64(token.token()),
            system_time_to_ms_since_epoch(token.expires_at()),
        );

        (
            StatusCode::FOUND,
            Json(PowResponse::new(Some(PowResponseData::new(policy, token)))),
        )
    } else {
        (StatusCode::NOT_FOUND, Json(PowResponse::new(None)))
    }
}
