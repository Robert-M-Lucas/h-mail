use crate::config::{POW_RSA_BITS, POW_TOKEN_EXPIRY_MS};
use h_mail_interface::interface::pow::{PowFailureReason, PowToken};
use rsa::traits::{PrivateKeyParts, PublicKeyParts};
use rsa::{BigUint, RsaPrivateKey};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, SystemTime};

pub struct PowProvider {
    current: HashMap<BigUint, (BigUint, BigUint)>,
    expiry: VecDeque<(SystemTime, BigUint)>,
}

impl Default for PowProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl PowProvider {
    pub fn new() -> Self {
        PowProvider {
            current: HashMap::new(),
            expiry: VecDeque::new(),
        }
    }

    pub fn get_token(&mut self) -> PowToken {
        let mut rng = rand::thread_rng(); // rand@0.8
        let priv_key = RsaPrivateKey::new(&mut rng, POW_RSA_BITS).unwrap();
        let p = priv_key.primes()[0].clone();
        let q = priv_key.primes()[1].clone();
        let n = priv_key.n().clone();

        let expires_at = SystemTime::now()
            .checked_add(Duration::from_millis(POW_TOKEN_EXPIRY_MS))
            .unwrap();

        let pow_token = PowToken::new(n.clone(), expires_at);

        self.current.insert(n.clone(), (p, q));
        self.expiry.push_back((expires_at, n));

        pow_token
    }

    fn remove_expired(&mut self) {
        while self
            .expiry
            .front()
            .is_some_and(|(f, _)| f < &SystemTime::now())
        {
            let (_, n) = self.expiry.pop_front().unwrap();
            self.current.remove(&n); // Might not be present if already used
        }
    }

    pub async fn check_pow(
        &mut self,
        token: BigUint,
        iters: u64,
        hash: BigUint,
        pow_result: BigUint,
    ) -> Result<(), PowFailureReason> {
        self.remove_expired();

        // TODO: This could be vulnerable to a timing attack
        let Some((p, q)) = self.current.remove(&token) else {
            return Err(PowFailureReason::NotFoundCanRetry);
        };
        let n = token;

        tokio::task::spawn_blocking(move || {
            let t = BigUint::from(iters);
            let phi = &(p - 1u32) * &(q - 1u32);
            let e = BigUint::from(2usize).modpow(&t, &phi);
            let actual = hash.modpow(&e, &n);

            if actual == pow_result {
                Ok(())
            } else {
                Err(PowFailureReason::FailedNoRetry)
            }
        })
        .await
        .unwrap()
    }
}
