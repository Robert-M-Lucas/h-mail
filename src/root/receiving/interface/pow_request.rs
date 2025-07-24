use crate::root::receiving::interface::shared::PowPolicy;
use crate::root::pow::PowToken;
use crate::root::shared::{base64_to_big_uint, ms_since_epoch_to_system_time};
use base64::DecodeError;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};
use crate::root::receiving::interface::shared::BigUintField;

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct PowTokenRequest {
    destination: String,
}

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct PowTokenResponse {
    data: Option<PowResponseData>,
}

impl PowTokenResponse {
    pub fn get(self) -> Option<PowResponseData> {
        self.data
    }
}

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct PowTokenSendable {
    token: BigUintField,
    expires_at: u128,
}

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct PowResponseData {
    policy: PowPolicy,
    pow_token: PowTokenSendable,
}

impl PowResponseData {
    pub fn decode(self) -> Result<PowResponseDataDecoded, DecodeError> {
        let token = self.pow_token.token().decode()?;

        let pow_token = PowToken::new(
            token,
            ms_since_epoch_to_system_time(self.pow_token.expires_at()),
        );

        Ok(PowResponseDataDecoded {
            policy: self.policy,
            pow_token,
        })
    }
}

#[derive(new, Debug, Getters)]
pub struct PowResponseDataDecoded {
    policy: PowPolicy,
    pow_token: PowToken,
}
