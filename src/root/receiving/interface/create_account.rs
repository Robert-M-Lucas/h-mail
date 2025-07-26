
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};
use crate::root::receiving::interface::shared::PowFailureReason;
use crate::root::receiving::interface::shared::BigUintField;

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
    BadUserName,
    BadPassword,
    DoesNotMeetPolicy(u64),
    PowCheckFailed(PowFailureReason)
}
