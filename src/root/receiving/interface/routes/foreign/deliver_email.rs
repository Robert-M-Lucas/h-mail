use crate::root::receiving::interface::email::Email;
use crate::root::receiving::interface::fields::auth_token::AuthTokenDataField;
use crate::root::receiving::interface::fields::big_uint::BigUintField;
use crate::root::receiving::interface::pow::PowFailureReason;
use crate::root::receiving::interface::pow::PowPolicy;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct DeliverEmailRequest {
    source_user: String,
    source_domain: String,
    email: Email,
    iters: u64,
    token: BigUintField,
    verify_ip: AuthTokenDataField,
    pow_result: BigUintField,
    destination: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DeliverEmailResponse {
    Success,
    UserNotFound,
    DoesNotMeetPolicy(PowPolicy),
    PowFailure(PowFailureReason),
    BadRequest,
    SenderIpNotAuthed,
}
