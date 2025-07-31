use crate::interface::email::Email;
use crate::interface::fields::auth_token::AuthTokenDataField;
use crate::interface::pow::{PowFailureReason, PowPolicy};
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const FOREIGN_DELIVER_EMAIL_PATH: &str = "/foreign/deliver_email";

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct DeliverEmailRequest {
    email: Email,
    source_user: String,
    source_domain: String,
    verify_ip: AuthTokenDataField,
    verify_ip_port: u16,
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
