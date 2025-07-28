use crate::root::receiving::interface::fields::auth_token::AuthTokenField;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct VerifyIpRequest {
    ip_verification: AuthTokenField,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VerifyIpResponse {
    Success,
    BadRequest,
    Failure,
}
