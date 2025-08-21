use crate::interface::RequestMethod;
use crate::interface::auth::Authorized;
use crate::interface::pow::PowPolicy;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const NATIVE_SET_POW_POLICY_PATH: &str = "/native/set_pow_policy";
pub const NATIVE_SET_POW_POLICY_METHOD: RequestMethod = RequestMethod::Post;
pub const NATIVE_SET_POW_POLICY_REQUIRES_AUTH: bool = true;

/// Changes the authenticated user's POW policy
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct SetPowPolicyRequest {
    policy: PowPolicy,
}

/// Returns whether the request succeeded
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum SetPowPolicyResponseAuthed {
    Success,
}

pub type SetPowPolicyResponse = Authorized<SetPowPolicyResponseAuthed>;
