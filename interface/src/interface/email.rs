use crate::interface::fields::big_uint::BigUintField;
use crate::interface::pow::{PowHash, WithPow};
use derive_getters::Getters;
use rand::{RngCore, thread_rng};
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

/// Represents an email being sent. The email's hash is used to identify an email uniquely (for
/// replying to emails), with the `random_id` being used to differentiate two exactly identical
/// emails. As the `random_id` is client-chosen, the hash of the email should not be used as a UID
/// for servers as a client can easily construct two emails with identical hashes.
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
            random_id: thread_rng().next_u32(),
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
