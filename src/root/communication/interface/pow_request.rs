use serde::{Deserialize, Serialize};
use derive_getters::Getters;
use derive_new::new;
use crate::root::database::PowPolicy;

#[derive(Deserialize, Getters, new)]
pub struct PowRequest {
    destination: String,
}

#[derive(Serialize, Getters, new)]
pub struct PowResponse {
    data: Option<PowResponseData>
}

#[derive(Serialize, Getters, new)]
pub struct PowTokenSend {
    token: String,
    expires_at: u128,
}

#[derive(Serialize, Getters, new)]
pub struct PowResponseData {
    policy: PowPolicy,
    pow_token: PowTokenSend
}