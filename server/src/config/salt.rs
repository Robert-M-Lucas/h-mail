use crate::config::args::ARGS;
use argon2::password_hash::{Salt, SaltString};
use h_mail_interface::server_config::MIN_SALT_BYTES;
use once_cell::sync::Lazy;
use rand::{Rng, thread_rng};
use std::env;
use std::env::VarError;
use tracing::debug;
use tracing::log::warn;

pub static SALT: Lazy<Option<SaltString>> = Lazy::new(|| {
    let set;
    let s = match env::var("SECRET_SALT") {
        Ok(salt) => {
            set = true;
            Some(salt)
        }
        Err(VarError::NotPresent) => {
            set = false;
            None
        }
        Err(VarError::NotUnicode(_)) => {
            set = true;
            if !ARGS.no_salt() {
                panic!("SECRET_SALT is not valid unicode")
            };
            None
        }
    };

    if !ARGS.no_salt() && s.is_none() {
        panic!("SECRET_SALT is not set");
    }
    if ARGS.no_salt() && set {
        warn!("SECRET_SALT is set but is being ignored");
    }

    let s = match s {
        Some(s) => s,
        None => return None,
    };

    let mut b = [0u8; Salt::MAX_LENGTH];
    let s = SaltString::from_b64(&s).unwrap();
    let len = s.decode_b64(&mut b).unwrap().len();
    debug!("Salt length: {}", len);
    if len < MIN_SALT_BYTES {
        panic!("SECRET_SALT too short (minimum {MIN_SALT_BYTES} bytes)");
    }

    Some(s)
});

pub fn generate_salt() -> SaltString {
    let mut salt = [0u8; MIN_SALT_BYTES];
    thread_rng().fill(&mut salt[..]);
    SaltString::encode_b64(&salt).unwrap()
}
