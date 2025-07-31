use crate::interface::pow::{PowFailureReason, PowHash, PowIters, WithPow};
use crate::shared::hash_str;
use derive_getters::Getters;
use derive_new::new;
use rsa::BigUint;
use serde::{Deserialize, Serialize};

pub const NATIVE_CREATE_ACCOUNT_PATH: &str = "/native/create_account";

#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct CreateAccountPackage {
    username: String,
    password: String,
}

impl PowHash for CreateAccountPackage {
    fn pow_hash(&self) -> BigUint {
        hash_str(&self.username)
    }
}

// #[derive(Serialize, Deserialize, Getters, new, Debug)]
// pub struct CreateAccountRequest {
//     package: CreateAccountPackage,
//     iters: PowIters,
//     token: BigUintField,
//     pow_result: BigUintField,
// }

pub type CreateAccountRequest = WithPow<CreateAccountPackage>;

#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum CreateAccountResponse {
    Success,
    BadUsername,
    UsernameInUse,
    BadPassword,
    DoesNotMeetPolicy(PowIters),
    PowFailure(PowFailureReason),
}
