use std::time::{SystemTime, UNIX_EPOCH};
use base64::{DecodeError, Engine};
use base64::prelude::BASE64_STANDARD;
use rsa::BigUint;

pub fn big_uint_to_base64(u: &BigUint) -> String {
    BASE64_STANDARD.encode(u.to_bytes_le())
}

pub fn base64_to_big_uint<T: AsRef<str>>(s: T) -> Result<BigUint, DecodeError> {
    BASE64_STANDARD.decode(s.as_ref()).map(|u| BigUint::from_bytes_le(&u))
}

pub fn system_time_to_ms_since_epoch(st: &SystemTime) -> u128 {
    st.duration_since(UNIX_EPOCH).unwrap().as_millis()
}