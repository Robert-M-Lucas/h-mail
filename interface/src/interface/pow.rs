use derive_getters::Getters;
use derive_new::new;
use rsa::BigUint;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub type PowIters = u32;

#[derive(Getters, new, Debug)]
pub struct PowToken {
    token: BigUint,
    expires_at: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PowFailureReason {
    FailedNoRetry,
    NotFoundCanRetry,
    BadRequestCanRetry,
    BadIPCanRetry,
}

#[derive(Getters, Serialize, Deserialize, Debug)]
pub struct PowPolicy {
    minimum: PowIters,
    accepted: PowIters,
    personal: PowIters,
}

impl PowPolicy {
    pub const fn new(minimum: PowIters, accepted: PowIters, personal: PowIters) -> PowPolicy {
        PowPolicy {
            minimum,
            accepted,
            personal,
        }
    }
}

impl PowPolicy {
    pub fn classify(&self, iters: PowIters) -> Option<PowClassification> {
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
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
