use crate::root::receiving::interface::shared::PowFailureReason;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};
use crate::root::receiving::interface::shared::BigUintField;
use crate::root::receiving::interface::shared::PowPolicy;

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct DeliverEmailRequest {
    source: String,
    email: String,
    iters: u64,
    token: BigUintField,
    hash_result: BigUintField,
    destination: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DeliverEmailResponse {
    Success,
    UserNotFound,
    DoesNotMeetPolicy(PowPolicy),
    PowFailure(PowFailureReason),
    SourceVerificationFailed
}
