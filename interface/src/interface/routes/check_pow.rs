use crate::interface::RequestMethod;
use crate::interface::pow::{PowFailureReason, PowHash, WithPow};
use derive_getters::Getters;
use derive_new::new;
#[cfg(feature = "client_implementation")]
use rsa::BigUint;
#[cfg(feature = "client_implementation")]
use rsa::signature::digest::Digest;
use serde::{Deserialize, Serialize};
#[cfg(feature = "client_implementation")]
use sha2::Sha256;

pub const CHECK_POW_PATH: &str = "/check_pow";
pub const CHECK_POW_METHOD: RequestMethod = RequestMethod::Post;
pub const CHECK_POW_REQUIRES_AUTH: bool = false;

/// Utility function to check POW. Note that checking POW will invalidate the POW token,
/// preventing it from being used for other purposes.
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct CheckPowPackage {
    challenge: String,
}

impl PowHash for CheckPowPackage {
    #[cfg(feature = "client_implementation")]
    fn pow_hash(&self) -> BigUint {
        let mut s = Sha256::new();
        s.update(self.challenge.as_bytes());
        BigUint::from_bytes_le(&s.finalize())
    }
}

pub type CheckPowRequest = WithPow<CheckPowPackage>;

/// Returns whether the POW was solved correctly and, if not, why
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum CheckPowResponse {
    Success,
    Failure(PowFailureReason),
}
