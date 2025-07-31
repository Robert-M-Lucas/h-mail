use crate::interface::fields::auth_token::{AuthTokenDataField, AuthTokenField};
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const AUTH_REFRESH_ACCESS_PATH: &str = "/auth/refresh_access";

#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Getters, new)]
pub struct RefreshAccessRequest {
    refresh_token: AuthTokenField,
}

#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum RefreshAccessResponse {
    Failure,
    BadRequest,
    Success(AuthTokenDataField),
}
