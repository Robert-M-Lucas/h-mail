#[cfg(feature = "client_implementation")]
use crate::interface::pow::{PowHashComponent, St};
#[cfg(feature = "client_implementation")]
use crate::utility::ms_since_epoch_to_system_time;
#[cfg(feature = "client_implementation")]
use crate::utility::system_time_to_ms_since_epoch;
#[cfg(feature = "client_implementation")]
use rsa::signature::digest::Digest;
use serde::{Deserialize, Serialize};
#[cfg(feature = "client_implementation")]
use std::time::SystemTime;

/// A timestamp represented as milliseconds since epoch
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemTimeField(u64);

#[cfg(not(feature = "client_implementation"))]
impl SystemTimeField {
    pub fn new(inner: u64) -> Self {
        SystemTimeField(inner)
    }
}

#[cfg(feature = "client_implementation")]
impl SystemTimeField {
    pub fn decode(&self) -> SystemTime {
        ms_since_epoch_to_system_time(self.0 as u128)
    }

    pub fn new(value: &SystemTime) -> SystemTimeField {
        SystemTimeField(system_time_to_ms_since_epoch(value) as u64)
    }
}

#[cfg(feature = "client_implementation")]
impl PowHashComponent for SystemTimeField {
    fn update_hash(&self, sha256: &mut St) {
        sha256.update(self.0.to_le_bytes())
    }
}
