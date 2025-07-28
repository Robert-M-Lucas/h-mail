use h_mail_interface::interface::auth::{AuthToken, AuthTokenData};
use std::collections::{HashMap, VecDeque};
use std::time::SystemTime;

pub struct AuthTokenProvider<T> {
    current: HashMap<AuthToken, T>,
    expiry: VecDeque<(SystemTime, AuthToken)>,
    expiry_ms: u64,
}

impl<T> AuthTokenProvider<T>
where
    T: Clone,
{
    pub fn new(expiry_ms: u64) -> AuthTokenProvider<T> {
        AuthTokenProvider {
            current: HashMap::new(),
            expiry: VecDeque::new(),
            expiry_ms,
        }
    }

    pub fn get_token(&mut self, data: T) -> AuthTokenData {
        let auth_token = AuthTokenData::generate_new(self.expiry_ms);
        self.current.insert(auth_token.token().clone(), data);
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

    pub fn validate_token(&mut self, auth_token: &AuthToken) -> Result<T, ()> {
        self.remove_expired();

        // TODO: This could be vulnerable to a timing attack
        let Some(data) = self.current.get(auth_token) else {
            return Err(());
        };

        Ok(data.clone())
    }
}
