use crate::interface::fields::big_uint::BigUintField;
use base64::DecodeError;
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use rsa::BigUint;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::time::SystemTime;

pub type PowIters = u32;

/// A wrapper around a request requiring a proof-of-work (POW). The `token` is obtained from a
/// `GetPowTokenRequest`. Some hash of `inner` is squared `iters` times (modulo `token`) to obtain
/// `pow_result`.
///
/// See `inner`'s value for the underlying type.
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, new, Getters, Debug)]
pub struct WithPow<T: PowHash> {
    inner: T,
    iters: PowIters,
    token: BigUintField,
    pow_result: BigUintField,
}

impl<T: PowHash> PowHash for WithPow<T> {
    fn pow_hash(&self) -> BigUint {
        self.inner.pow_hash()
    }
}

impl<T: PowHash> WithPow<T> {
    pub fn decode(self) -> Result<WithPowDecoded<T>, DecodeError> {
        let (inner, iters, token, pow_result) =
            (self.inner, self.iters, self.token, self.pow_result);

        Ok(WithPowDecoded {
            inner_dangerous: inner,
            iters,
            token: token.decode()?,
            pow_result: pow_result.decode()?,
        })
    }
}

#[derive(Getters, Debug, Dissolve)]
pub struct WithPowDecoded<T: PowHash> {
    inner_dangerous: T,
    iters: PowIters,
    token: BigUint,
    pow_result: BigUint,
}

impl<T: PowHash> PowHash for WithPowDecoded<T> {
    fn pow_hash(&self) -> BigUint {
        self.inner_dangerous.pow_hash()
    }
}

pub trait PowHash {
    fn pow_hash(&self) -> BigUint;
}

#[derive(Getters, new, Debug)]
pub struct PowToken {
    token: BigUint,
    expires_at: SystemTime,
}

/// Reason for a POW check failing
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum PowFailureReason {
    FailedNoRetry,
    NotFoundCanRetry,
    BadRequestCanRetry,
    DoesNotMeetPolicyMinimum(PowIters),
}

/// Represents a user's pow policy that dictates how an incoming email is categorised
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Getters, Serialize, Deserialize, Debug, Clone)]
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

/// Represents a classification in the `PowPolicy`
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
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

    pub fn from_ident(ident: &str) -> Option<PowClassification> {
        match ident {
            "MINIMUM" => Some(PowClassification::Minimum),
            "ACCEPTED" => Some(PowClassification::Accepted),
            "PERSONAL" => Some(PowClassification::Personal),
            _ => None,
        }
    }
}
