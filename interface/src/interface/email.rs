use crate::interface::pow::{PowHash, WithPow};
use derive_getters::Getters;
use derive_new::new;
use rsa::BigUint;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// #[derive(Serialize, Deserialize, Debug, new, Getters)]
// pub struct Email {
//     email: EmailPackage,
//     iters: PowIters,
//     token: BigUintField,
//     pow_result: BigUintField,
//
// }

pub type Email = WithPow<EmailPackage>;

/// Represents an email being sent. The hash of this will be used for POW when sending emails.
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Clone, Debug, new, Getters)]
pub struct EmailPackage {
    destination_user: String,
    contents: String,
}

impl PowHash for EmailPackage {
    fn pow_hash(&self) -> BigUint {
        let mut s = Sha256::new();
        s.update(self.contents.as_bytes());
        BigUint::from_bytes_le(&s.finalize())
    }
}
