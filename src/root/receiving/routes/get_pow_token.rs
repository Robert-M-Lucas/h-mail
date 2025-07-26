use crate::root::receiving::interface::get_pow_token::{
    PowTokenRequest, PowTokenResponse,
};
use crate::root::shared_resources::POW_PROVIDER;
use axum::extract::{ConnectInfo, Query};
use axum::http::StatusCode;
use axum::Json;
use std::net::SocketAddr;

pub async fn pow_request(
    Query(_pow_request): Query<PowTokenRequest>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> (StatusCode, Json<PowTokenResponse>) {
    let pow_token = POW_PROVIDER.write().await.get_token(addr.ip());
    (StatusCode::OK, PowTokenResponse::from_token(&pow_token).into())
}
