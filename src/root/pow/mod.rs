use crate::root::config::TOKEN_EXPIRY_TIME;
use crate::root::receiving::interface::shared::PowFailureReason;
use derive_getters::Getters;
use derive_new::new;
use rsa::traits::{PrivateKeyParts, PublicKeyParts};
use rsa::{BigUint, RsaPrivateKey};
use std::collections::{HashMap, VecDeque};
use std::net::IpAddr;
use std::time::{Duration, SystemTime};

#[derive(Getters, new, Debug)]
pub struct PowToken {
    token: BigUint,
    expires_at: SystemTime,
}

pub struct PowProvider {
    current: HashMap<BigUint, (IpAddr, BigUint, BigUint)>,
    expiry: VecDeque<(SystemTime, BigUint)>,
}

impl PowProvider {
    pub fn new() -> Self {
        PowProvider {
            current: HashMap::new(),
            expiry: VecDeque::new(),
        }
    }

    pub fn get_token(&mut self, addr: IpAddr) -> PowToken {
        let mut rng = rand::thread_rng(); // rand@0.8
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
        let p = priv_key.primes()[0].clone();
        let q = priv_key.primes()[1].clone();
        let n = priv_key.n().clone();

        let expires_at = SystemTime::now()
            .checked_add(Duration::from_millis(TOKEN_EXPIRY_TIME))
            .unwrap();

        let pow_token = PowToken {
            token: n.clone(),
            expires_at,
        };

        self.current.insert(n.clone(), (addr, p, q));
        self.expiry.push_back((expires_at, n));

        pow_token
    }

    pub async fn check_pow(
        &mut self,
        token: BigUint,
        iters: u64,
        hash: BigUint,
        pow_result: BigUint,
    ) -> Result<IpAddr, PowFailureReason> {
        while self
            .expiry
            .front()
            .is_some_and(|(f, _)| f < &SystemTime::now())
        {
            let (_, n) = self.expiry.pop_front().unwrap();
            self.current.remove(&n); // Might not be present if already used
        }

        let Some((ip_addr, p, q)) = self.current.remove(&token) else {
            return Err(PowFailureReason::NotFoundCanRetry);
        };
        let n = token;

        tokio::task::spawn_blocking(move || {
            let t = BigUint::from(iters);
            let phi = &(p - 1u32) * &(q - 1u32);
            let e = BigUint::from(2usize).modpow(&t, &phi);
            let actual = hash.modpow(&e, &n);

            if actual == pow_result {
                Ok(ip_addr)
            } else {
                Err(PowFailureReason::FailedNoRetry)
            }
        })
        .await
        .unwrap()
    }
}
