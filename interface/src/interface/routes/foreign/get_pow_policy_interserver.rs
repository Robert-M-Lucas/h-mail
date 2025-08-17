use crate::interface::fields::auth_token::AuthTokenDataField;
use crate::interface::pow::{PowClassification, PowPolicy};
use crate::shared::RequestMethod;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};
use crate::interface::fields::hmail_address::HmailAddress;

pub const FOREIGN_GET_POW_POLICY_INTERSERVER_PATH: &str = "/foreign/get_pow_policy_interserver";
pub const FOREIGN_GET_POW_POLICY_INTERSERVER_METHOD: RequestMethod = RequestMethod::Post;
pub const FOREIGN_GET_POW_POLICY_INTERSERVER_REQUIRES_AUTH: bool = false;

/// Asks whether a sender is whitelisted from POW by a user
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetPowPolicyInterserverRequest {
    recipient_username: String,
    sender: HmailAddress,
    ip_verification: AuthTokenDataField,
    verify_ip_port: u16,
}

/// Returns whether the user is whitelisted from POW (and the POW policy if not)
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum GetPowPolicyInterserverResponse {
    Whitelisted(PowClassification),
    NotWhitelisted(PowPolicy),
    SenderIpNotAuthed,
    BadRequest,
}
