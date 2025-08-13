use crate::shared::{ms_since_epoch_to_system_time, system_time_to_ms_since_epoch};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// A timestamp represented as milliseconds since epoch
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemTimeField(u64);

impl SystemTimeField {
    pub fn decode(&self) -> SystemTime {
        ms_since_epoch_to_system_time(self.0 as u128)
    }

    pub fn new(value: &SystemTime) -> SystemTimeField {
        SystemTimeField(system_time_to_ms_since_epoch(value) as u64)
    }

    pub fn bytes_for_hash(&self) -> Vec<u8> {
        Vec::from(&self.0.to_le_bytes())
    }
}
