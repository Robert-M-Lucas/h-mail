use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct CheckIpRequest {
    ip: String
}

#[derive(Serialize, Deserialize, new, Debug)]
pub enum CheckIpResponse {
    Authorised,
    Unauthorised,
}
