use crate::root::database::PowPolicy;
use crate::root::pow::PowToken;
use base64::DecodeError;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};
use crate::root::shared::{base64_to_big_uint, ms_since_epoch_to_system_time};

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct PowRequest {
    destination: String,
}

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct PowResponse {
    data: Option<PowResponseData>,
}

impl PowResponse {
    pub fn get(self) -> Option<PowResponseData> {
        self.data
    }
}

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct PowTokenSend {
    token: String,
    expires_at: u128,
}

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct PowResponseData {
    policy: PowPolicy,
    pow_token: PowTokenSend,
}

impl PowResponseData {
    pub fn decode(self) -> Result<PowResponseDataDecoded, DecodeError> {
        let token = base64_to_big_uint(self.pow_token.token())?;

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
