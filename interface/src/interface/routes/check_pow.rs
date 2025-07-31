use crate::interface::pow::{PowFailureReason, PowHash, WithPow};
use crate::shared::hash_str;
use derive_getters::Getters;
use derive_new::new;
use rsa::BigUint;
use serde::{Deserialize, Serialize};

pub const CHECK_POW_PATH: &str = "/check_pow";

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct CheckPowPackage {
    challenge: String,
}

impl PowHash for CheckPowPackage {
    fn pow_hash(&self) -> BigUint {
        hash_str(&self.challenge)
    }
}

pub type CheckPowRequest = WithPow<CheckPowPackage>;

#[derive(Serialize, Deserialize, Debug)]
pub enum CheckPowResponse {
    Success,
    Failure(PowFailureReason),
}
