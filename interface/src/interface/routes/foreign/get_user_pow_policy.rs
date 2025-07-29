use crate::interface::pow::PowPolicy;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const FOREIGN_GET_USER_POW_POLICY_PATH: &str = "/foreign/get_user_pow_policy";

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetUserPowPolicyRequest {
    destination: String,
}

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetUserPowPolicyResponse {
    data: Option<PowPolicy>,
}

impl GetUserPowPolicyResponse {
    pub fn get(self) -> Option<PowPolicy> {
        self.data
    }
}
