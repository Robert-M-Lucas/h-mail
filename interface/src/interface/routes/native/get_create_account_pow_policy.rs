use crate::interface::pow::PowIters;
use crate::shared::RequestMethod;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_PATH: &str = "/native/get_create_account_pow_policy";
pub const NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_METHOD: RequestMethod = RequestMethod::Get;
pub const NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_REQUIRES_AUTH: bool = false;

/// Requests the number of POW iterations required to create an account
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetCreateAccountPowPolicyRequest {}

/// Returns the number of POW iterations required to create an account
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetCreateAccountPowPolicyResponse {
    required: PowIters,
}
