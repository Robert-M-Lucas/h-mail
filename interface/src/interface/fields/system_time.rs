use crate::shared::{ms_since_epoch_to_system_time, system_time_to_ms_since_epoch};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub struct SystemTimeField(u128);

impl SystemTimeField {
    pub fn decode(&self) -> SystemTime {
        ms_since_epoch_to_system_time(self.0)
    }

    pub fn new(value: &SystemTime) -> SystemTimeField {
        SystemTimeField(system_time_to_ms_since_epoch(value))
    }
}
