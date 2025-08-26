use crate::interface::RequestMethod;
use crate::interface::fields::auth_token::AuthTokenField;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};
use crate::interface::fields::hmail_address::HmailAddress;

pub const FOREIGN_VERIFY_IP_PATH: &str = "/foreign/verify_ip";
pub const FOREIGN_VERIFY_IP_METHOD: RequestMethod = RequestMethod::Post;
pub const FOREIGN_VERIFY_IP_REQUIRES_AUTH: bool = false;

/// A `DeliverEmailRequest` will cause the target server to issue a `VerifyIpRequest` back
/// to the sender to ensure the IP is not being spoofed. The `ip_verification` token should be
/// the same as the one in `DeliverEmailRequest`.
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct VerifyIpRequest {
    ip_verification: AuthTokenField,
    recipient: HmailAddress
}

/// Returns whether the `DeliverEmailRequest` originated from this server
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum VerifyIpResponse {
    Success,
    BadRequest,
    Failure,
}
