use crate::interface::pow::PowPolicy;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};
use crate::shared::RequestMethod;

pub const FOREIGN_GET_USER_POW_POLICY_PATH: &str = "/foreign/get_user_pow_policy";
pub const FOREIGN_GET_USER_POW_POLICY_METHOD: RequestMethod = RequestMethod::Get;
pub const FOREIGN_GET_USER_POW_POLICY_REQUIRES_AUTH: bool = false;

/// GET: Requests a users POW policy
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetUserPowPolicyRequest {
    destination: String,
}

/// Returns the users POW policy, if they exist
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetUserPowPolicyResponse {
    data: Option<PowPolicy>,
}

impl GetUserPowPolicyResponse {
    pub fn get(self) -> Option<PowPolicy> {
        self.data
    }
}
