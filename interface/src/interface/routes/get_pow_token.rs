use crate::interface::fields::big_uint::BigUintField;
use crate::interface::fields::system_time::SystemTimeField;
use crate::interface::pow::PowToken;
use crate::shared::RequestMethod;
use base64::DecodeError;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const GET_POW_TOKEN_PATH: &str = "/get_pow_token";
pub const GET_POW_TOKEN_METHOD: RequestMethod = RequestMethod::Get;
pub const GET_POW_TOKEN_REQUIRES_AUTH: bool = false;

/// GET: Request a POW token used for various purposes
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetPowTokenRequest {}

/// Returns a POW token
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, Debug)]
pub struct GetPowTokenResponse {
    token: BigUintField,
    expires_at: SystemTimeField,
}

impl GetPowTokenResponse {
    pub fn from_token(token: &PowToken) -> GetPowTokenResponse {
        GetPowTokenResponse {
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
