use crate::interface::fields::auth_token::AuthTokenDataField;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const AUTH_AUTHENTICATE_PATH: &str = "/auth/authenticate";

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct AuthenticateRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthenticateResponse {
    Failure,
    Success(AuthTokenDataField),
}
