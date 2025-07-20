use std::collections::{HashMap, VecDeque};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use derive_getters::Getters;
use rsa::{BigUint, RsaPrivateKey};
use rsa::traits::{PrivateKeyParts, PublicKeyParts};
use serde::Serialize;
use tokio::time::Instant;

#[derive(Serialize)]
pub enum PowCheck {
    Success,
    FailureNoRetry,
    NotFoundCanRetry,
    BadRequestCanRetry,
}

#[derive(Getters)]
pub struct PowToken {
    token: BigUint,
    expires_at: SystemTime,
}

const TOKEN_EXPIRY_TIME: u64 = 10 * 60 * 1000;

pub struct PowProvider {
    current: HashMap<BigUint, (BigUint, BigUint)>,
    expiry: VecDeque<(SystemTime, BigUint)>,
}

impl PowProvider {
    pub fn new() -> Self {
        PowProvider {
            current: HashMap::new(),
            expiry: VecDeque::new()
        }
    }

    pub fn get_token(&mut self) -> PowToken {
        let mut rng = rand::thread_rng(); // rand@0.8
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
        let p = priv_key.primes()[0].clone();
        let q = priv_key.primes()[1].clone();
        let n = priv_key.n().clone();

        let expires_at = SystemTime::now().checked_add(Duration::from_millis(TOKEN_EXPIRY_TIME)).unwrap();
        
        let pow_token = PowToken {
            token: n.clone(),
            expires_at,
        };
        
        self.current.insert(n.clone(), (p, q));
        self.expiry.push_back((expires_at, n));
        
        pow_token
    }
    
    pub fn check_pow(&mut self, token: BigUint, iters: u64, challenge: BigUint, result: BigUint) -> PowCheck {
        while self.expiry.front().is_some_and(|(f, _)| f < &SystemTime::now()) {
            let (_, n) = self.expiry.pop_front().unwrap();
            self.current.remove(&n).unwrap();
        }
        
        let Some((p, q)) = self.current.remove(&token) else {
            return PowCheck::NotFoundCanRetry;
        };
        let n = token;

        let t = BigUint::from(iters);
        let phi = &(p - 1u32) * &(q - 1u32);
        let e = BigUint::from(2usize).modpow(&t, &phi);
        let actual = challenge.modpow(&e, &n);
        
        if actual == result {
            PowCheck::Success
        }
        else {
            PowCheck::FailureNoRetry
        }
    }
}