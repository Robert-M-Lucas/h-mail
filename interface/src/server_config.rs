use crate::interface::pow::{PowIters, PowPolicy};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

pub const MIN_SALT_BYTES: usize = 32;

#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Getters)]
pub struct ServerConfig {
    pub domain: String,
    pub port: u16,
    pub create_account_pow_burden: PowIters,
    pub pow_token_expiry_ms: u64,
    pub pow_rsa_bits: usize,
    pub refresh_token_expiry_ms: u64,
    pub access_token_expiry_ms: u64,
    pub verify_ip_token_expiry_ms: u64,
    pub default_user_pow_policy: PowPolicy,
}

pub const ROUGH_POW_ITER_PER_SECOND: PowIters = 6_500;

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            domain: "127.0.0.1".to_string(),
            port: 8081,
            create_account_pow_burden: ROUGH_POW_ITER_PER_SECOND * 60,
            pow_token_expiry_ms: 60 * 60 * 1000, // 1 hour
            pow_rsa_bits: 2048,
            refresh_token_expiry_ms: 30 * 24 * 60 * 60 * 1000, // 30 days
            access_token_expiry_ms: 60 * 60 * 1000,            // 1 hour
            verify_ip_token_expiry_ms: 60 * 1000,              // 1 minute
            default_user_pow_policy: PowPolicy::new(
                ROUGH_POW_ITER_PER_SECOND,
                ROUGH_POW_ITER_PER_SECOND * 10,
                ROUGH_POW_ITER_PER_SECOND * 100,
            ),
        }
    }
}
