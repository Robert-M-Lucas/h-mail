use crate::interface::fields::auth_token::AuthTokenDataField;
use crate::interface::pow::{PowClassification, PowPolicy};
use crate::shared::RequestMethod;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const FOREIGN_IS_WHITELISTED_INTERSERVER_PATH: &str = "/foreign/is_whitelisted";
pub const FOREIGN_IS_WHITELISTED_INTERSERVER_METHOD: RequestMethod = RequestMethod::Post;
pub const FOREIGN_IS_WHITELISTED_INTERSERVER_REQUIRES_AUTH: bool = false;

/// Asks whether a sender is whitelisted from POW by a user
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct IsWhitelistedInterserverRequest {
    recipient: String,
    sender: String,
    ip_verification: AuthTokenDataField,
    verify_ip_port: u16,
}

/// Returns whether the user is whitelisted from POW (and the POW policy if not)
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum IsWhitelistedInterserverResponse {
    Whitelisted(PowClassification),
    NotWhitelisted(PowPolicy),
    SenderIpNotAuthed,
    BadRequest,
}
