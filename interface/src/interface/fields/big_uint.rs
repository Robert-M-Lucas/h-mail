#[cfg(feature = "client_implementation")]
use crate::interface::pow::{PowHashComponent, St};
#[cfg(feature = "client_implementation")]
use crate::utility::base64_to_big_uint;
#[cfg(feature = "client_implementation")]
use crate::utility::big_uint_to_base64;
#[cfg(feature = "client_implementation")]
use base64::DecodeError;
#[cfg(feature = "client_implementation")]
use rsa::BigUint;
#[cfg(feature = "client_implementation")]
use rsa::signature::digest::Digest;
use serde::{Deserialize, Serialize};

/// A base-64 (standard alphabet, with padding) little-endian encoding of a large unsigned integer
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BigUintField(String);

#[cfg(not(feature = "client_implementation"))]
impl BigUintField {
    pub fn new(inner: String) -> Self {
        BigUintField(inner)
    }
}

#[cfg(feature = "client_implementation")]
impl BigUintField {
    pub fn decode(&self) -> Result<BigUint, DecodeError> {
        base64_to_big_uint(&self.0)
    }

    pub fn new(value: &BigUint) -> BigUintField {
        BigUintField(big_uint_to_base64(value))
    }

    pub fn to_string(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_raw(s: String) -> Self {
        BigUintField(s)
    }
}

#[cfg(feature = "client_implementation")]
impl PowHashComponent for BigUintField {
    fn update_hash(&self, sha256: &mut St) {
        sha256.update(self.0.as_bytes())
    }
}
