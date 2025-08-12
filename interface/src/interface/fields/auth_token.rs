use crate::error::HResult;
use crate::interface::auth::{AuthToken, AuthTokenData};
use crate::interface::fields::system_time::SystemTimeField;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/// Represents a base-64 encoded authentication token - you will not need to decode this.
/// Used in bearer tokens for authentication and for sender IP verification.
///
/// Note that the length of this token is server-implementation-dependent. As the token will only
/// ever be decoded/checked/used by the server it issued from, there is no need for standardisation.
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthTokenField(pub String);

impl AuthTokenField {
    pub fn decode(&self) -> HResult<AuthToken> {
        AuthToken::from_string(&self.0)
    }

    pub fn new(token: &AuthToken) -> AuthTokenField {
        AuthTokenField(token.encode())
    }
}

/// An `AuthToken` with attached expiry time
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, Debug)]
pub struct AuthTokenDataField {
    token: AuthTokenField,
    expires_at: SystemTimeField,
}

impl AuthTokenDataField {
    pub fn new(data: &AuthTokenData) -> AuthTokenDataField {
        AuthTokenDataField {
            token: AuthTokenField::new(data.token()),
            expires_at: SystemTimeField::new(data.expires_at()),
        }
    }
}
