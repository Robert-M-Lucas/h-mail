use crate::interface::fields::big_uint::BigUintField;
use crate::interface::pow::{PowFailureReason, PowIters};
use crate::shared::hash_str;
use derive_getters::Getters;
use derive_new::new;
use rsa::BigUint;
use serde::{Deserialize, Serialize};

pub const NATIVE_CREATE_ACCOUNT_PATH: &str = "/native/create_account";

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct CreateAccountPackage {
    username: String,
    password: String,
}

impl CreateAccountPackage {
    pub fn hash(&self) -> BigUint {
        hash_str(&self.username)
    }
}

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct CreateAccountRequest {
    package: CreateAccountPackage,
    iters: PowIters,
    token: BigUintField,
    pow_result: BigUintField,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CreateAccountResponse {
    Success,
    BadUsername,
    UsernameInUse,
    BadPassword,
    DoesNotMeetPolicy(PowIters),
    PowFailure(PowFailureReason),
}
