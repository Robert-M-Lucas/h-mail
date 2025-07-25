use crate::root::shared::{base64_to_big_uint, big_uint_to_base64};
use base64::DecodeError;
use derive_getters::Getters;
use derive_new::new;
use rsa::BigUint;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BigUintField(String);

impl BigUintField {
    pub fn decode(&self) -> Result<BigUint, DecodeError> {
        base64_to_big_uint(&self.0)
    }

    pub fn new(value: &BigUint) -> Self {
        BigUintField(big_uint_to_base64(value))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PowFailureReason {
    FailedNoRetry,
    NotFoundCanRetry,
    BadRequestCanRetry,
}

#[derive(Getters, Serialize, Deserialize, new, Debug)]
pub struct PowPolicy {
    minimum: u64,
    accepted: u64,
    personal: u64,
}

impl PowPolicy {
    pub fn classify(&self, iters: u64) -> Option<PowClassification> {
        if iters < self.minimum {
            None
        } else if iters < self.accepted {
            Some(PowClassification::Minimum)
        } else if iters < self.personal {
            Some(PowClassification::Accepted)
        } else {
            Some(PowClassification::Personal)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum PowClassification {
    Minimum,
    Accepted,
    Personal,
}

impl PowClassification {
    pub fn to_ident(&self) -> &'static str {
        match self {
            PowClassification::Minimum => "MINIMUM",
            PowClassification::Accepted => "ACCEPTED",
            PowClassification::Personal => "PERSONAL",
        }
    }

    pub fn from_ident(ident: &str) -> Result<PowClassification, ()> {
        match ident {
            "MINIMUM" => Ok(PowClassification::Minimum),
            "ACCEPTED" => Ok(PowClassification::Accepted),
            "PERSONAL" => Ok(PowClassification::Personal),
            _ => Err(()),
        }
    }
}
