use crate::interface::fields::big_uint::BigUintField;
use crate::interface::pow::PowIters;
use derive_getters::Getters;
use derive_new::new;
use rsa::BigUint;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, new, Getters)]
pub struct Email {
    email: EmailPackage,
    iters: PowIters,
    token: BigUintField,
    pow_result: BigUintField,
    destination_user: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, new, Getters)]
pub struct EmailPackage {
    contents: String,
}

impl EmailPackage {
    pub fn hash(&self) -> BigUint {
        let mut s = Sha256::new();
        s.update(self.contents.as_bytes());
        BigUint::from_bytes_le(&s.finalize())
    }
}
