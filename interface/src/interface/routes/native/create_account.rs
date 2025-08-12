// use std::marker::PhantomData;
use crate::interface::pow::{PowFailureReason, PowHash, PowIters, WithPow};
use crate::shared::{RequestMethod, hash_str};
use derive_getters::Getters;
use derive_new::new;
use rsa::BigUint;
use serde::{Deserialize, Serialize};

pub const NATIVE_CREATE_ACCOUNT_PATH: &str = "/native/create_account";
pub const NATIVE_CREATE_ACCOUNT_METHOD: RequestMethod = RequestMethod::Post;
pub const NATIVE_CREATE_ACCOUNT_REQUIRES_AUTH: bool = false;

/// Requests an account be created. Requires POW burden obtained through
/// `GetCreateAccountPowPolicyRequest`.
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

pub type CreateAccountRequest = WithPow<CreateAccountPackage>;

/// Returns whether the account creation succeeded and, if not, why
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
