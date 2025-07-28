use crate::root::receiving::interface::email::EmailPackage;
use crate::root::receiving::interface::fields::auth_token::AuthTokenDataField;
use crate::root::receiving::interface::pow::PowFailureReason;
use crate::root::receiving::interface::pow::PowPolicy;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct DeliverEmailRequest {
    package: EmailPackage,
    source_user: String,
    source_domain: String,
    verify_ip: AuthTokenDataField,
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
