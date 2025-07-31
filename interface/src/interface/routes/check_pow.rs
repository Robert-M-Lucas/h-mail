use crate::interface::fields::big_uint::BigUintField;
use crate::interface::pow::{PowFailureReason, PowIters};
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const CHECK_POW_PATH: &str = "/check_pow";

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct CheckPowRequest {
    token: BigUintField,
    iters: PowIters,
    challenge: BigUintField,
    result: BigUintField,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CheckPowResponse {
    Success,
    Failure(PowFailureReason),
}
