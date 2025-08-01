use crate::interface::fields::auth_token::AuthTokenDataField;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const AUTH_AUTHENTICATE_PATH: &str = "/auth/authenticate";

/// POST: Requests an access token using a username and password
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct AuthenticateRequest {
    username: String,
    password: String,
}

/// Returns a refresh token if successful
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum AuthenticateResponse {
    Failure,
    Success(AuthTokenDataField),
}
