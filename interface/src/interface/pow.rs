use crate::interface::fields::big_uint::BigUintField;
#[cfg(feature = "client_implementation")]
use base64::DecodeError;
#[cfg(feature = "client_implementation")]
use derive_getters::Dissolve;
use derive_getters::Getters;
use derive_new::new;
#[cfg(feature = "client_implementation")]
use rsa::BigUint;
#[cfg(feature = "client_implementation")]
use rsa::signature::digest::consts::U32;
#[cfg(feature = "client_implementation")]
use rsa::signature::digest::core_api::{CoreWrapper, CtVariableCoreWrapper};
use serde::{Deserialize, Serialize};
#[cfg(feature = "client_implementation")]
use sha2::Sha256VarCore;
use std::fmt::Debug;
#[cfg(feature = "client_implementation")]
use std::time::SystemTime;

pub type PowIters = u32;

/// A wrapper around a request requiring a proof-of-work (POW). The `token` is obtained from a
/// `GetPowTokenRequest`. The hash of `inner` (`inner.pow_hash()`) is squared `pow_result.iters` times (modulo `pow_result.token`) to obtain
/// `pow_result.pow_result`.
///
/// See `inner`'s value for the underlying type.
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, new, Getters, Debug)]
pub struct WithPow<T: PowHash> {
    inner_dangerous: T,
    pow_result: Option<PowResult>,
}

impl<T: PowHash> PowHash for WithPow<T> {
    #[cfg(feature = "client_implementation")]
    fn pow_hash(&self) -> BigUint {
        self.inner_dangerous.pow_hash()
    }
}

/// The result of solving a POW token. Used in `WithPow`.
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, new, Getters, Debug)]
pub struct PowResult {
    iters: PowIters,
    token: BigUintField,
    pow_result: BigUintField,
}

#[cfg(feature = "client_implementation")]
impl PowResult {
    pub fn decode(self) -> Result<PowResultDecoded, DecodeError> {
        let (iters, token, pow_result) = (self.iters, self.token, self.pow_result);
        Ok(PowResultDecoded {
            iters,
            token: token.decode()?,
            pow_result: pow_result.decode()?,
        })
    }
}

#[cfg(feature = "client_implementation")]
#[derive(Debug, Getters, new, Dissolve)]
pub struct PowResultDecoded {
    iters: PowIters,
    token: BigUint,
    pow_result: BigUint,
}

#[cfg(feature = "client_implementation")]
impl PowResultDecoded {
    pub fn encode(&self) -> PowResult {
        let (iters, token, pow_result) = (self.iters, &self.token, &self.pow_result);
        PowResult {
            iters,
            token: BigUintField::new(token),
            pow_result: BigUintField::new(pow_result),
        }
    }
}

impl<T: PowHash> WithPow<T> {
    #[cfg(feature = "client_implementation")]
    pub fn decode(self) -> Result<WithPowDecoded<T>, DecodeError> {
        let (inner, pow_result) = (self.inner_dangerous, self.pow_result);

        let pow_result = if let Some(pow_result) = pow_result {
            Some(pow_result.decode()?)
        } else {
            None
        };

        Ok(WithPowDecoded {
            inner_dangerous: inner,
            pow_result,
        })
    }
}

#[cfg(feature = "client_implementation")]
#[derive(Getters, Debug, Dissolve)]
pub struct WithPowDecoded<T: PowHash> {
    inner_dangerous: T,
    pow_result: Option<PowResultDecoded>,
}

#[cfg(feature = "client_implementation")]
impl<T: PowHash> PowHash for WithPowDecoded<T> {
    fn pow_hash(&self) -> BigUint {
        self.inner_dangerous.pow_hash()
    }
}

pub trait PowHash {
    #[cfg(feature = "client_implementation")]
    fn pow_hash(&self) -> BigUint;
}

#[cfg(feature = "client_implementation")]
pub type St = CoreWrapper<CtVariableCoreWrapper<Sha256VarCore, U32, sha2::OidSha256>>;

#[cfg(feature = "client_implementation")]
pub trait PowHashComponent {
    fn update_hash(&self, sha256: &mut St);
}

#[cfg(feature = "client_implementation")]
#[derive(Getters, new, Debug, Dissolve)]
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

/// Represents a user's pow policy that dictates how an incoming h-mail is categorised
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

    #[cfg(feature = "client_implementation")]

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

    pub fn iters_from_classification(&self, classification: PowClassification) -> PowIters {
        match classification {
            PowClassification::Minimum => self.minimum,
            PowClassification::Accepted => self.accepted,
            PowClassification::Personal => self.personal,
        }
    }
}

/// Represents a classification in the `PowPolicy`
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
