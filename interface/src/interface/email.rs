use crate::interface::pow::{PowHash, WithPow};
use derive_getters::Getters;
use rand::{thread_rng, RngCore};
use rsa::BigUint;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::interface::fields::big_uint::BigUintField;
// #[derive(Serialize, Deserialize, Debug, new, Getters)]
// pub struct Email {
//     email: EmailPackage,
//     iters: PowIters,
//     token: BigUintField,
//     pow_result: BigUintField,
//
// }

pub type Email = WithPow<EmailPackage>;

/// Represents an email being sent
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Clone, Debug, Getters)]
pub struct EmailPackage {
    destination_user: String,
    contents: String,
    reply_to: Option<BigUintField>,
    /// If two emails are the same / have the same hash, differentiate them
    random_id: u32,
}

impl EmailPackage {
    pub fn new(destination_user: String, contents: String, reply_to: Option<BigUintField>) -> Self {
        EmailPackage {
            destination_user,
            contents,
            reply_to,
            random_id: thread_rng().next_u32()
        }
    }
}

impl PowHash for EmailPackage {
    fn pow_hash(&self) -> BigUint {
        let mut s = Sha256::new();
        s.update(self.destination_user.as_bytes());
        s.update(self.contents.as_bytes());
        if let Some(reply_to) = &self.reply_to {
            s.update(reply_to.bytes_for_hash())
        }
        s.update(self.random_id.to_be_bytes());
        BigUint::from_bytes_le(&s.finalize())
    }
}
