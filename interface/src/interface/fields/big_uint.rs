use crate::shared::{base64_to_big_uint, big_uint_to_base64};
use base64::DecodeError;
use rsa::BigUint;
use serde::{Deserialize, Serialize};

/// A base-64 (standard alphabet, with padding) little-endian encoding of a large unsigned integer
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BigUintField(String);

impl BigUintField {
    pub fn decode(&self) -> Result<BigUint, DecodeError> {
        base64_to_big_uint(&self.0)
    }

    pub fn new(value: &BigUint) -> BigUintField {
        BigUintField(big_uint_to_base64(value))
    }

    pub fn bytes_for_hash(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
