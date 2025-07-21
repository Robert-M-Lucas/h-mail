use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PowFailureReason {
    FailedNoRetry,
    NotFoundCanRetry,
    BadRequestCanRetry
}