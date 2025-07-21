use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};
use crate::root::communication::interface::shared::PowFailureReason;
use crate::root::database::PowPolicy;

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct SendEmail {
    email: String,
    iters: u64,
    token: String,
    hash_result: String,
    destination: String
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SendEmailStatus {
    Success,
    UserNotFound,
    DoesNotMeetPolicy(PowPolicy),
    PowFailure(PowFailureReason)
}