use crate::root::receiving::interface::pow_request::{
    PowResponseData, PowTokenRequest, PowTokenResponse, PowTokenSendable,
};
use crate::root::receiving::interface::shared::BigUintField;
use crate::root::shared::system_time_to_ms_since_epoch;
use crate::root::shared_resources::{DB, POW_PROVIDER};
use axum::Json;
use axum::extract::{ConnectInfo, Query};
use axum::http::StatusCode;
use std::net::SocketAddr;

pub async fn pow_request(
    Query(pow_request): Query<PowTokenRequest>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> (StatusCode, Json<PowTokenResponse>) {
    let policy = DB
        .lock()
        .await
        .as_ref()
        .unwrap()
        .get_user_pow_policy(pow_request.destination());

    if let Some(policy) = policy {
        let token = POW_PROVIDER.write().await.get_token(addr.ip());
        let token = PowTokenSendable::new(
            BigUintField::new(token.token()),
            system_time_to_ms_since_epoch(token.expires_at()),
        );

        (
            StatusCode::FOUND,
            Json(PowTokenResponse::new(Some(PowResponseData::new(
                policy, token,
            )))),
        )
    } else {
        (StatusCode::NOT_FOUND, Json(PowTokenResponse::new(None)))
    }
}
