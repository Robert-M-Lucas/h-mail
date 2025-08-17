use crate::config::config_file::CONFIG;
use h_mail_interface::interface::pow::{
    PowFailureReason, PowHash, PowIters, PowToken, WithPowDecoded,
};
use h_mail_interface::reexports::BigUint;
use h_mail_interface::reexports::rsa::RsaPrivateKey;
use h_mail_interface::reexports::rsa::traits::{PrivateKeyParts, PublicKeyParts};
use h_mail_interface::utility::shortcut_solve_pow;
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
        let priv_key = RsaPrivateKey::new(&mut rng, CONFIG.pow_rsa_bits()).unwrap();
        let p = priv_key.primes()[0].clone();
        let q = priv_key.primes()[1].clone();
        let n = priv_key.n().clone();

        let expires_at = SystemTime::now()
            .checked_add(Duration::from_millis(CONFIG.pow_token_expiry_ms()))
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

    pub async fn check_pow<T: PowHash>(
        &mut self,
        with_pow: WithPowDecoded<T>,
        min_iters: PowIters,
    ) -> Result<T, PowFailureReason> {
        self.remove_expired();

        let hash = with_pow.pow_hash();
        let (inner, pow_result) = with_pow.dissolve();
        let Some(pow_result) = pow_result else {
            return if min_iters == 0 {
                Ok(inner)
            } else {
                Err(PowFailureReason::DoesNotMeetPolicyMinimum(min_iters))
            };
        };
        if *pow_result.iters() < min_iters {
            return Err(PowFailureReason::DoesNotMeetPolicyMinimum(min_iters));
        }
        let (iters, token, pow_result) = pow_result.dissolve();

        // TODO: This could be vulnerable to a timing attack
        let Some((p, q)) = self.current.remove(&token) else {
            return Err(PowFailureReason::NotFoundCanRetry);
        };

        tokio::task::spawn_blocking(move || {
            let actual = shortcut_solve_pow(&p, &q, iters, &hash);

            if actual == pow_result {
                Ok(())
            } else {
                Err(PowFailureReason::FailedNoRetry)
            }
        })
        .await
        .unwrap()?;
        Ok(inner)
    }
}
