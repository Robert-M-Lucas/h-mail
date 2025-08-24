use crate::interface::RequestMethod;
use crate::interface::fields::auth_token::AuthTokenDataField;
use crate::interface::fields::hmail_address::HmailAddress;
use crate::interface::hmail::{Hmail, SendHmailPackage};
use crate::interface::pow::{PowFailureReason, PowPolicy};
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const FOREIGN_DELIVER_HMAIL_PATH: &str = "/foreign/deliver_hmail";
pub const FOREIGN_DELIVER_HMAIL_METHOD: RequestMethod = RequestMethod::Post;
pub const FOREIGN_DELIVER_HMAIL_REQUIRES_AUTH: bool = false;

/// Delivers an h-mail from another server. The `ip_verification` token will be used in a
/// `VerifyIpRequest` to the `source_domain` on port `verify_ip_port`, expecting a
/// `VerifyIpResponse` to ensure that the IP is not being spoofed. Requires POW (in `hmail`) for
/// which the hash of `hmail->inner_dangerous (HmailPackage)` will be used as the POW hash. The
/// sender's IP will also be checked against the `source_domain`'s SPF records to ensure that the IP
/// is authorised by the domain to send emails.
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug, Dissolve)]
pub struct DeliverHmailRequest {
    hmail: Hmail,
    recipient_address: HmailAddress,
    ip_verification: AuthTokenDataField,
    verify_ip_port: u16,
    context: Vec<SendHmailPackage>,
}

/// Returns whether the h-mail delivery succeeded and, if not, why
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum DeliverHmailResponse {
    Success,
    UserNotFound,
    DoesNotMeetPolicy(PowPolicy),
    PowFailure(PowFailureReason),
    BadRequest,
    SenderIpNotAuthed,
}
