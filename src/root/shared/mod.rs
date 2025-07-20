use base64::prelude::BASE64_STANDARD;
use base64::{DecodeError, Engine};
use rsa::BigUint;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn big_uint_to_base64(u: &BigUint) -> String {
    BASE64_STANDARD.encode(u.to_bytes_le())
}

pub fn base64_to_big_uint<T: AsRef<str>>(s: T) -> Result<BigUint, DecodeError> {
    BASE64_STANDARD
        .decode(s.as_ref())
        .map(|u| BigUint::from_bytes_le(&u))
}

pub fn system_time_to_ms_since_epoch(st: &SystemTime) -> u128 {
    st.duration_since(UNIX_EPOCH).unwrap().as_millis()
}

pub fn ms_since_epoch_to_system_time(ms: u128) -> SystemTime {
    UNIX_EPOCH + Duration::from_millis(ms as u64)
}
