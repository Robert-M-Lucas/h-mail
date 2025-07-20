use derive_getters::Getters;
use serde::Deserialize;

#[derive(Deserialize, Getters)]
pub struct CheckPow {
    token: String,
    iters: u64,
    challenge: String,
    result: String,
}
