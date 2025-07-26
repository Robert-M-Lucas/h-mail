use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct CreateAccountPowPolicyRequest {}

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct CreateAccountPowPolicyResponse {
    required: u64
}
