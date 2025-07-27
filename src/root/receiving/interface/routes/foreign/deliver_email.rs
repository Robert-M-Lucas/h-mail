use crate::root::receiving::interface::fields::big_uint::BigUintField;
use crate::root::receiving::interface::shared::PowFailureReason;
use crate::root::receiving::interface::shared::PowPolicy;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct DeliverEmailRequest {
    source_user: String,
    source_domain: String,
    email: String,
    iters: u64,
    token: BigUintField,
    pow_result: BigUintField,
    destination: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DeliverEmailResponse {
    Success,
    UserNotFound,
    DoesNotMeetPolicy(PowPolicy),
    PowFailure(PowFailureReason),
    SenderIpNotAuthed,
}
