use crate::root::config::AUTH_TOKEN_BYTES;
use crate::root::database::UserId;
use crate::root::shared::{base64_to_bytes, bytes_to_base64};
use derive_getters::Getters;
use rand::Rng;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, SystemTime};

pub type AuthTokenBytes = [u8; AUTH_TOKEN_BYTES];

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

pub struct AuthTokenProvider {
    current: HashMap<AuthToken, UserId>,
    expiry: VecDeque<(SystemTime, AuthToken)>,
    expiry_ms: u64,
}

impl AuthTokenProvider {
    pub fn new(expiry_ms: u64) -> AuthTokenProvider {
        AuthTokenProvider {
            current: HashMap::new(),
            expiry: VecDeque::new(),
            expiry_ms,
        }
    }

    pub fn get_token(&mut self, user_id: UserId) -> AuthTokenData {
        let auth_token = AuthTokenData::generate_new(self.expiry_ms);
        self.current.insert(auth_token.token().clone(), user_id);
        self.expiry
            .push_back((*auth_token.expires_at(), auth_token.token().clone()));
        auth_token
    }

    fn remove_expired(&mut self) {
        while self
            .expiry
            .front()
            .is_some_and(|(f, _)| f < &SystemTime::now())
        {
            let (_, t) = self.expiry.pop_front().unwrap();
            self.current.remove(&t); // Might not be present if already revoked
        }
    }

    pub fn validate_token(&mut self, auth_token: &AuthToken) -> Result<UserId, ()> {
        self.remove_expired();

        // TODO: This could be vulnerable to a timing attack
        let Some(token_user) = self.current.get(auth_token) else {
            return Err(());
        };

        Ok(*token_user)
    }
}
