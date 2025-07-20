use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct CheckPow {
    token: String,
    iters: u64,
    challenge: String,
    result: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CheckPowStatus {
    Success,
    FailureNoRetry,
    NotFoundCanRetry,
    BadRequestCanRetry,
}