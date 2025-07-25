use crate::root::receiving::interface::shared::{BigUintField, PowFailureReason};
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct CheckPowRequest {
    token: BigUintField,
    iters: u64,
    challenge: BigUintField,
    result: BigUintField,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CheckPowResponse {
    Success,
    Failure(PowFailureReason),
}
