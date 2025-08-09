use crate::interface::fields::auth_token::AuthTokenDataField;
use crate::shared::RequestMethod;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const AUTH_AUTHENTICATE_PATH: &str = "/auth/authenticate";
pub const AUTH_AUTHENTICATE_METHOD: RequestMethod = RequestMethod::Get;
pub const AUTH_AUTHENTICATE_REQUIRES_AUTH: bool = true;

/// Requests an access token using a username and password
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
