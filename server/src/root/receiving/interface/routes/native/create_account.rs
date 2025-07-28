use crate::root::receiving::interface::fields::big_uint::BigUintField;
use crate::root::receiving::interface::pow::PowFailureReason;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct CreateAccountRequest {
    username: String,
    password: String,
    iters: u64,
    token: BigUintField,
    pow_result: BigUintField,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CreateAccountResponse {
    Success,
    BadUsername,
    UsernameInUse,
    BadPassword,
    DoesNotMeetPolicy(u64),
    PowFailure(PowFailureReason),
}
