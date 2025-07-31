use crate::interface::pow::PowIters;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_PATH: &str = "/native/get_create_account_pow_policy";

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetCreateAccountPowPolicyRequest {}

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetCreateAccountPowPolicyResponse {
    required: PowIters,
}
