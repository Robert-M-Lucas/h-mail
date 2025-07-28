use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use derive_getters::Getters;
use rand::Rng;
use crate::root::config::AUTH_TOKEN_BYTES;
use crate::root::shared::{base64_to_bytes, bytes_to_base64};

#[derive(Serialize, Deserialize, Debug)]
pub enum Authorized<T> {
    Success(T),
    Unauthorized,
}

#[derive(Getters, Clone, PartialOrd, PartialEq, Eq, Hash, Debug)]
pub struct AuthToken {
    token: AuthTokenBytes,
}

impl AuthToken {
    pub fn from_bytes(bytes: AuthTokenBytes) -> AuthToken {
        AuthToken { token: bytes }
    }

    pub fn from_string<T: AsRef<str>>(s: T) -> Result<AuthToken, ()> {
        Ok(AuthToken {
            token: base64_to_bytes(s)
                .map_err(|_| ())?
                .try_into()
                .map_err(|_| ())?,
        })
    }

    pub fn to_string(&self) -> String {
        bytes_to_base64(&self.token)
    }
}

#[derive(Getters)]
pub struct AuthTokenData {
    token: AuthToken,
    expires_at: SystemTime,
}

impl AuthTokenData {
    pub fn generate_new(expiry_ms: u64) -> AuthTokenData {
        let mut rng = rand::thread_rng();
        let mut token: AuthTokenBytes = [0; AUTH_TOKEN_BYTES];
        rng.fill(&mut token[..]);
        let expires_at = SystemTime::now()
            .checked_add(Duration::from_millis(expiry_ms))
            .unwrap();
        AuthTokenData {
            token: AuthToken::from_bytes(token),
            expires_at,
        }
    }
}

pub type AuthTokenBytes = [u8; AUTH_TOKEN_BYTES];