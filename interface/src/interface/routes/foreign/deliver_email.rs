use crate::interface::email::Email;
use crate::interface::fields::auth_token::AuthTokenDataField;
use crate::interface::pow::{PowFailureReason, PowPolicy};
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const FOREIGN_DELIVER_EMAIL_PATH: &str = "/foreign/deliver_email";

/// POST: Delivers an email from another server. The `ip_verification` token will be used in a
/// `VerifyIpRequest` to the `source_domain` on port `verify_ip_port` to ensure that the IP
/// is not being spoofed. Requires POW (in `email`) for which the hash of
/// `Email->inner (EmailPackage)` will be used as the POW hash.
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug, Dissolve)]
pub struct DeliverEmailRequest {
    email: Email,
    source_user: String,
    source_domain: String,
    ip_verification: AuthTokenDataField,
    verify_ip_port: u16,
}

/// Returns whether the email delivery succeeded and, if not, why
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum DeliverEmailResponse {
    Success,
    UserNotFound,
    DoesNotMeetPolicy(PowPolicy),
    PowFailure(PowFailureReason),
    BadRequest,
    SenderIpNotAuthed,
}
