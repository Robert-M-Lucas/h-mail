use std::num::NonZero;
use crate::config::ROUGH_POW_ITER_PER_SECOND;
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
    pub password_regex: String,
    pub password_requirement_text: String,
    pub rate_limit_burst_size: NonZero<u32>,
    pub rate_limit_refresh_ms: NonZero<u64>
}

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
            password_regex: "^.{8,}$".to_string(),
            password_requirement_text: "Password must have at least 8 characters.".to_string(),
            rate_limit_burst_size: NonZero::new(100).unwrap(),
            rate_limit_refresh_ms: NonZero::new(100).unwrap()
        }
    }
}
