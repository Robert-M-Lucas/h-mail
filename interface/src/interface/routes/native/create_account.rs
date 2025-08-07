use std::marker::PhantomData;
use crate::interface::pow::{PowFailureReason, PowHash, PowIters, WithPow};
use crate::shared::hash_str;
use derive_getters::Getters;
use derive_new::new;
use rsa::BigUint;
use serde::{Deserialize, Serialize};

// pub trait PathDef<P, R> {
//     fn path() -> &'static str;
//     fn payload_type() -> PhantomData<P>;
//     fn return_type() -> PhantomData<R>;
// }
//
// struct CreateAccountPathDef;
// impl PathDef<CreateAccountPackage, CreateAccountResponse> for CreateAccountPathDef {
//     fn path() -> &'static str {
//         NATIVE_CREATE_ACCOUNT_PATH
//     }
//
//     fn payload_type() -> PhantomData<CreateAccountPackage> {
//         PhantomData::default()
//     }
//
//     fn return_type() -> PhantomData<CreateAccountResponse> {
//         PhantomData::default()
//     }
// }
//
// fn test() {
//
// }

pub const NATIVE_CREATE_ACCOUNT_PATH: &str = "/native/create_account";

/// POST: Requests an account be created. Requires POW burden obtained through
/// `GetCreateAccountPowPolicyRequest`. The hash of `username` will be used for the POW hash.
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
