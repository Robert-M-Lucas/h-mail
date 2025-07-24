use crate::root::communication::interface::pow_request::{
    PowTokenRequest, PowTokenResponse, PowResponseData, PowTokenSendable,
};
use crate::root::shared::{big_uint_to_base64, system_time_to_ms_since_epoch};
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use crate::root::shared_resources::{DB, POW_PROVIDER};

pub async fn pow_request(Query(pow_request): Query<PowTokenRequest>) -> (StatusCode, Json<PowTokenResponse>) {
    let policy = DB
        .lock()
        .unwrap()
        .get_user_pow_policy(pow_request.destination());
    if let Some(policy) = policy {
        let token = POW_PROVIDER.write().await.get_token();
        let token = PowTokenSendable::new(
            big_uint_to_base64(token.token()),
            system_time_to_ms_since_epoch(token.expires_at()),
        );

        (
            StatusCode::FOUND,
            Json(PowTokenResponse::new(Some(PowResponseData::new(policy, token)))),
        )
    } else {
        (StatusCode::NOT_FOUND, Json(PowTokenResponse::new(None)))
    }
}
