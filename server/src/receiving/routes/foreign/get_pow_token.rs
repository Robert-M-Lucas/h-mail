use crate::shared_resources::POW_PROVIDER;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use h_mail_interface::interface::routes::foreign::get_pow_token::{
    GetPowTokenRequest, GetPowTokenResponse,
};

pub async fn get_pow_token(
    Query(_pow_request): Query<GetPowTokenRequest>,
) -> (StatusCode, Json<GetPowTokenResponse>) {
    let pow_token = POW_PROVIDER.write().await.get_token();
    (
        StatusCode::OK,
        GetPowTokenResponse::from_token(&pow_token).into(),
    )
}
