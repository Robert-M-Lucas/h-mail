use crate::root::auth_token_provider::{AuthToken, AuthTokenData};
use crate::root::receiving::interface::fields::system_time::SystemTimeField;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthTokenField(String);

impl AuthTokenField {
    pub fn decode(&self) -> Result<AuthToken, ()> {
        AuthToken::from_string(&self.0)
    }

    pub fn new(token: &AuthToken) -> AuthTokenField {
        AuthTokenField(token.to_string())
    }
}

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
