use base64::prelude::BASE64_STANDARD;
use base64::{DecodeError, Engine};
use rsa::BigUint;
use sha2::{Digest, Sha256};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn big_uint_to_base64(u: &BigUint) -> String {
    bytes_to_base64(&u.to_bytes_le())
}

pub fn bytes_to_base64(bytes: &[u8]) -> String {
    BASE64_STANDARD.encode(bytes)
}

pub fn base64_to_big_uint<T: AsRef<str>>(s: T) -> Result<BigUint, DecodeError> {
    base64_to_bytes(s).map(|u| BigUint::from_bytes_le(&u))
}

pub fn base64_to_bytes<T: AsRef<str>>(s: T) -> Result<Vec<u8>, DecodeError> {
    BASE64_STANDARD.decode(s.as_ref())
}

pub fn system_time_to_ms_since_epoch(st: &SystemTime) -> u128 {
    st.duration_since(UNIX_EPOCH).unwrap().as_millis()
}

pub fn ms_since_epoch_to_system_time(ms: u128) -> SystemTime {
    UNIX_EPOCH + Duration::from_millis(ms as u64)
}

pub fn hash_str<T: AsRef<str>>(string: T) -> BigUint {
    let mut s = Sha256::new();
    s.update(string.as_ref().as_bytes());
    BigUint::from_bytes_le(&s.finalize())
}

pub fn get_url_for_path<S: AsRef<str>, P: AsRef<str>>(server: S, path: P) -> String {
    format!("https://{}{}", server.as_ref(), path.as_ref())
}
