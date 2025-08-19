use crate::interface::RequestMethod;
use crate::interface::auth::Authorized;
use crate::interface::fields::hmail_address::HmailAddress;
use crate::interface::pow::{PowClassification, PowPolicy};
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const NATIVE_GET_FOREIGN_POW_POLICY_PATH: &str = "/native/get_foreign_pow_policy";
pub const NATIVE_GET_FOREIGN_POW_POLICY_METHOD: RequestMethod = RequestMethod::Post;
pub const NATIVE_GET_FOREIGN_POW_POLICY_REQUIRES_AUTH: bool = true;

/// Asks whether this authenticated user is whitelisted by the recipient
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetForeignPowPolicyRequest {
    recipient: HmailAddress,
}

/// Returns whether this authenticated user is whitelisted by the recipient (and their POW policy
/// if not)
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum GetForeignPowPolicyResponseAuthed {
    Whitelisted(PowClassification),
    NotWhitelisted(PowPolicy),
    RequestFailed,
    BadRequest,
}

pub type GetForeignPowPolicyResponse = Authorized<GetForeignPowPolicyResponseAuthed>;
