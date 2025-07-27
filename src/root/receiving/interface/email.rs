use derive_getters::Getters;
use derive_new::new;
use rsa::BigUint;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Clone, Debug, new, Getters)]
pub struct Email {
    contents: String,
}

impl Email {
    pub fn hash(&self) -> BigUint {
        let mut s = Sha256::new();
        s.update(self.contents.as_bytes());
        BigUint::from_bytes_le(&s.finalize())
    }
}
