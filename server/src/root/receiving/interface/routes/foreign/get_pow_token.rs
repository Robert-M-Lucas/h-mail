use crate::root::pow_provider::PowToken;
use crate::root::receiving::interface::fields::big_uint::BigUintField;
use crate::root::receiving::interface::fields::system_time::SystemTimeField;
use base64::DecodeError;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct PowTokenRequest {}

#[derive(Serialize, Deserialize, Getters, Debug)]
pub struct PowTokenResponse {
    token: BigUintField,
    expires_at: SystemTimeField,
}

impl PowTokenResponse {
    pub fn from_token(token: &PowToken) -> PowTokenResponse {
        PowTokenResponse {
            token: BigUintField::new(token.token()),
            expires_at: SystemTimeField::new(token.expires_at()),
        }
    }

    pub fn decode(self) -> Result<PowToken, DecodeError> {
        let token = self.token.decode()?;
        let expires_at = self.expires_at.decode();

        let pow_token = PowToken::new(token, expires_at);

        Ok(pow_token)
    }
}
