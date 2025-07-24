use crate::root::receiving::interface::check_ip::{CheckIpRequest, CheckIpResponse};
use crate::root::ALLOWED_IPS;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;

pub async fn check_ip(Query(ip_request): Query<CheckIpRequest>) -> (StatusCode, Json<CheckIpResponse>) {
    if ALLOWED_IPS.contains(&&**ip_request.ip()) {
        (StatusCode::OK, CheckIpResponse::Authorised.into())
    }
    else {
        (StatusCode::NOT_ACCEPTABLE, CheckIpResponse::Unauthorised.into())
    }
}
