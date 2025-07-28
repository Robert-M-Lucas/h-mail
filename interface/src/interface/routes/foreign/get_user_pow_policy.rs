use crate::interface::pow::PowPolicy;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct UserPowPolicyRequest {
    destination: String,
}

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct UserPowPolicyResponse {
    data: Option<PowPolicy>,
}

impl UserPowPolicyResponse {
    pub fn get(self) -> Option<PowPolicy> {
        self.data
    }
}
