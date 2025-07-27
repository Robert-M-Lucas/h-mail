use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Authorized<T> {
    Success(T),
    Unauthorized,
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
    minimum: u64,
    accepted: u64,
    personal: u64,
}

impl PowPolicy {
    pub const fn new(minimum: u64, accepted: u64, personal: u64) -> PowPolicy {
        PowPolicy {
            minimum,
            accepted,
            personal,
        }
    }
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
