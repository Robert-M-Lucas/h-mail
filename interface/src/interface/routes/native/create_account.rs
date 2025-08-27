use crate::interface::RequestMethod;
use crate::interface::pow::{PowFailureReason, PowHash, PowIters, WithPow};
use derive_getters::Getters;
use derive_new::new;
#[cfg(feature = "client_implementation")]
use rsa::BigUint;
#[cfg(feature = "client_implementation")]
use rsa::signature::digest::Digest;
use serde::{Deserialize, Serialize};
#[cfg(feature = "client_implementation")]
use sha2::Sha256;

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
    #[cfg(feature = "client_implementation")]
    fn pow_hash(&self) -> BigUint {
        let mut s = Sha256::new();
        s.update(self.username.as_bytes());
        BigUint::from_bytes_le(&s.finalize())
    }
}

pub type CreateAccountRequest = WithPow<CreateAccountPackage>;

/// Returns whether the account creation succeeded and, if not, why
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum CreateAccountResponse {
    Success(String),
    BadUsername(String),
    UsernameInUse,
    BadPassword(String),
    DoesNotMeetPolicy(PowIters),
    PowFailure(PowFailureReason),
}
