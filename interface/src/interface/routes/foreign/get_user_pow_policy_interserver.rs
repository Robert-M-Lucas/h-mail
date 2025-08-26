use crate::interface::RequestMethod;
use crate::interface::fields::auth_token::AuthTokenDataField;
use crate::interface::fields::hmail_address::HmailAddress;
use crate::interface::pow::{PowClassification, PowPolicy};
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const FOREIGN_GET_USER_POW_POLICY_INTERSERVER_PATH: &str =
    "/foreign/get_user_pow_policy_interserver";
pub const FOREIGN_GET_USER_POW_POLICY_INTERSERVER_METHOD: RequestMethod = RequestMethod::Post;
pub const FOREIGN_GET_USER_POW_POLICY_INTERSERVER_REQUIRES_AUTH: bool = false;

/// Asks whether a sender is whitelisted from POW by a user
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetUserPowPolicyInterserverRequest {
    recipient: HmailAddress,
    sender: HmailAddress,
    ip_verification: AuthTokenDataField,
    verify_ip_port: u16,
}

/// -
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, Dissolve, new, Debug)]
pub struct WhitelistedResponse {
    classification: PowClassification,
    policy: PowPolicy,
}

/// Returns whether the user is whitelisted from POW (and the POW policy if not)
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum GetUserPowPolicyInterserverResponse {
    Whitelisted(WhitelistedResponse),
    NotWhitelisted(PowPolicy),
    SenderIpNotAuthed,
    BadRequest,
    UserDoesNotExist,
}
