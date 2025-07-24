use crate::root::communication::interface::shared::PowFailureReason;
use crate::root::database::PowPolicy;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct SendEmailRequest {
    source: String,
    email: String,
    iters: u64,
    token: String,
    hash_result: String,
    destination: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SendEmailResponse {
    Success,
    UserNotFound,
    DoesNotMeetPolicy(PowPolicy),
    PowFailure(PowFailureReason),
}
