use crate::interface::RequestMethod;
use crate::interface::pow::PowPolicy;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const FOREIGN_GET_ANONYMOUS_USER_POW_POLICY_PATH: &str =
    "/foreign/get_anonymous_user_pow_policy";
pub const FOREIGN_GET_ANONYMOUS_USER_POW_POLICY_METHOD: RequestMethod = RequestMethod::Get;
pub const FOREIGN_GET_ANONYMOUS_USER_POW_POLICY_REQUIRES_AUTH: bool = false;

/// Requests a user's POW policy. Use your servers `IsWhitelistedRequest` to get the POW policy,
/// also checking whether the sender is whitelisted and, therefore, does not need to complete POW.
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetAnonymousUserPowPolicyRequest {
    recipient_username: String,
}

/// Returns the users POW policy, if they exist
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetAnonymousUserPowPolicyResponse {
    data: Option<PowPolicy>,
}

#[cfg(feature = "client_implementation")]
impl GetAnonymousUserPowPolicyResponse {
    pub fn get(self) -> Option<PowPolicy> {
        self.data
    }
}
