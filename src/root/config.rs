use crate::root::receiving::interface::shared::PowPolicy;

pub const ROUGH_POW_ITER_PER_SECOND: u64 = 6_500;

pub const CREATE_ACCOUNT_POW_BURDEN: u64 = ROUGH_POW_ITER_PER_SECOND * 60;
pub const TOKEN_EXPIRY_TIME: u64 = 60 * 60 * 1000;

pub const DEFAULT_USER_POW_POLICY: PowPolicy = PowPolicy::new(
    ROUGH_POW_ITER_PER_SECOND, 
    ROUGH_POW_ITER_PER_SECOND * 10, 
    ROUGH_POW_ITER_PER_SECOND * 100
);
