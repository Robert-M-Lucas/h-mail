use crate::interface::auth::Authorized;
use crate::interface::pow::PowPolicy;
use crate::shared::RequestMethod;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const NATIVE_IS_WHITELISTED_PATH: &str = "/native/is_whitelisted";
pub const NATIVE_IS_WHITELISTED_METHOD: RequestMethod = RequestMethod::Post;
pub const NATIVE_IS_WHITELISTED_REQUIRES_AUTH: bool = true;

/// Asks whether this authenticated user is whitelisted by the recipient
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct IsWhitelistedRequest {
    recipient: String,
}

/// Returns whether this authenticated user is whitelisted by the recipient (and pow policy if not)
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum IsWhitelistedResponseAuthed {
    Whitelisted,
    NotWhitelisted(PowPolicy),
    RequestFailed,
    BadRequest,
}

pub type IsWhitelistedResponse = Authorized<IsWhitelistedResponseAuthed>;
