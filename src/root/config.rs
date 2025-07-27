use crate::root::receiving::interface::pow::PowPolicy;

pub const ROUGH_POW_ITER_PER_SECOND: u64 = 6_500;

pub const CREATE_ACCOUNT_POW_BURDEN: u64 = ROUGH_POW_ITER_PER_SECOND * 60;
pub const POW_TOKEN_EXPIRY_MS: u64 = 60 * 60 * 1000; // 1 hour

pub const POW_RSA_BITS: usize = 2048;

pub const AUTH_TOKEN_BYTES: usize = 256;
pub const REFRESH_TOKEN_EXPIRY_MS: u64 = 30 * 24 * 60 * 60 * 1000; // 30 days
pub const ACCESS_TOKEN_EXPIRY_MS: u64 = 60 * 60 * 1000; // 1 hour
pub const VERIFY_IP_TOKEN_EXPIRY_MS: u64 = 60 * 1000; // 1 minute

pub const DEFAULT_USER_POW_POLICY: PowPolicy = PowPolicy::new(
    ROUGH_POW_ITER_PER_SECOND,
    ROUGH_POW_ITER_PER_SECOND * 10,
    ROUGH_POW_ITER_PER_SECOND * 100,
);
