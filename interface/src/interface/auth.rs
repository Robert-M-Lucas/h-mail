#[cfg(feature = "client_implementation")]
use crate::config::AUTH_TOKEN_BYTES;
#[cfg(feature = "client_implementation")]
use crate::error::HResult;
#[cfg(feature = "client_implementation")]
use crate::utility::{base64_to_bytes, bytes_to_base64};
#[cfg(feature = "client_implementation")]
use anyhow::{Context, anyhow};
#[cfg(feature = "client_implementation")]
use derive_getters::Getters;
#[cfg(feature = "client_implementation")]
use rand::Rng;
use serde::{Deserialize, Serialize};
#[cfg(feature = "client_implementation")]
use std::time::Duration;
#[cfg(feature = "client_implementation")]
use std::time::SystemTime;

/// A wrapper around a response indicating whether a request that requires authorisation was
/// successful.
///
/// See `Success`'s value for the underlying type.
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum Authorized<T> {
    Success(T),
    Unauthorized,
}

#[cfg(feature = "client_implementation")]
#[derive(Getters, Clone, PartialOrd, PartialEq, Eq, Hash, Debug)]
pub struct AuthToken {
    token: AuthTokenBytes,
}

#[cfg(feature = "client_implementation")]
impl AuthToken {
    pub fn from_bytes(bytes: AuthTokenBytes) -> AuthToken {
        AuthToken { token: bytes }
    }

    pub fn from_string<T: AsRef<str>>(s: T) -> HResult<AuthToken> {
        Ok(AuthToken {
            token: base64_to_bytes(s)
                .context("Failed to decode auth token")?
                .try_into()
                .map_err(|_| anyhow!("Failed to decode auth token"))?,
        })
    }

    pub fn encode(&self) -> String {
        bytes_to_base64(&self.token)
    }
}

#[cfg(feature = "client_implementation")]
#[derive(Getters)]
pub struct AuthTokenData {
    token: AuthToken,
    expires_at: SystemTime,
}

#[cfg(feature = "client_implementation")]
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

#[cfg(feature = "client_implementation")]
pub type AuthTokenBytes = [u8; AUTH_TOKEN_BYTES];
