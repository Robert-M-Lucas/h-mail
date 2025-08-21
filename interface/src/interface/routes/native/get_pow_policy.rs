use crate::interface::RequestMethod;
use crate::interface::auth::Authorized;
use crate::interface::pow::PowPolicy;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const NATIVE_GET_POW_POLICY_PATH: &str = "/native/get_pow_policy";
pub const NATIVE_GET_POW_POLICY_METHOD: RequestMethod = RequestMethod::Get;
pub const NATIVE_GET_POW_POLICY_REQUIRES_AUTH: bool = true;

/// Gets the authenticated user's POW policy (note that the data returned is not secret)
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetPowPolicyRequest {}

/// Returns the authenticated user's POW policy (note that the data returned is not secret)
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetPowPolicyResponseAuthed {
    policy: PowPolicy,
}

pub type GetPowPolicyResponse = Authorized<GetPowPolicyResponseAuthed>;
