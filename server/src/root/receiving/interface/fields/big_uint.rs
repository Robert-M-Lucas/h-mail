use crate::root::shared::{base64_to_big_uint, big_uint_to_base64};
use base64::DecodeError;
use rsa::BigUint;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BigUintField(String);

impl BigUintField {
    pub fn decode(&self) -> Result<BigUint, DecodeError> {
        base64_to_big_uint(&self.0)
    }

    pub fn new(value: &BigUint) -> BigUintField {
        BigUintField(big_uint_to_base64(value))
    }
}
