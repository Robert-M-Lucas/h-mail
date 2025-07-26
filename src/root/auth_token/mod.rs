use std::collections::{HashMap, VecDeque};
use std::time::SystemTime;
use rsa::BigUint;
use crate::root::database::UserId;

pub struct AuthTokenProvider {
    current: HashMap<String, UserId>,
    expiry: VecDeque<(SystemTime, BigUint)>,
}



impl AuthTokenProvider {
    pub fn new() -> AuthTokenProvider {
        AuthTokenProvider {
            current: HashMap::new(),
            expiry: VecDeque::new(),
        }
    }
    
    pub fn get_token(&mut self, user_id: UserId) {
        
    }
}
