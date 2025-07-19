use std::time::{SystemTime, UNIX_EPOCH};
use axum::http::StatusCode;
use axum::Json;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use ctrlc::Error::System;
use rsa::BigUint;
use serde::{Deserialize, Serialize};
use crate::database::PowPolicy;
use crate::{DB, POW_PROVIDER};
use crate::pow::PowToken;
use crate::shared::{big_uint_to_base64, system_time_to_ms_since_epoch};

pub async fn pow_request(Json(pow_request): Json<PowRequest>) -> (StatusCode, Json<PowResponse>) {
    let policy = DB.lock().unwrap().get_user_pow_policy(&pow_request.destination);

    if let Some(policy) = policy {
        let token = POW_PROVIDER.write().await.get_token();
        let token = PowTokenSend {
            token: big_uint_to_base64(token.token()),
            expires_at: system_time_to_ms_since_epoch(token.expires_at()),
        };
        
        (StatusCode::FOUND, Json(PowResponse { data:  Some(PowResponseData {
            policy,
            pow_token: token,
        })}))
    }
    else {
        (StatusCode::NOT_FOUND, Json(PowResponse { data: None }))
    }
}

#[derive(Deserialize)]
pub struct PowRequest {
    destination: String,
}

#[derive(Serialize)]
pub struct PowResponse {
    data: Option<PowResponseData>
}

#[derive(Serialize)]
struct PowTokenSend {
    token: String,
    expires_at: u128,
}

#[derive(Serialize)]
pub struct PowResponseData {
    policy: PowPolicy,
    pow_token: PowTokenSend
}