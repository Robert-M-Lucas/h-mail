use crate::config::AUTH_TOKEN_BYTES;
use crate::error::HResult;
use crate::shared::{base64_to_bytes, bytes_to_base64};
use anyhow::{Context, anyhow};
use derive_getters::Getters;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
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
