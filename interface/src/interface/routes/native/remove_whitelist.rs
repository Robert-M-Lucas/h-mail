use crate::interface::auth::Authorized;
use crate::interface::pow::PowClassification;
use crate::shared::RequestMethod;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const NATIVE_REMOVE_WHITELIST_PATH: &str = "/native/remove_whitelist";
pub const NATIVE_REMOVE_WHITELIST_METHOD: RequestMethod = RequestMethod::Delete;
pub const NATIVE_REMOVE_WHITELIST_REQUIRES_AUTH: bool = true;

/// Adds an address to the authenticated user's whitelist
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct RemoveWhitelistRequest {
    address: String,
    place_into: PowClassification,
}

/// Returns whether the request succeeded
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum RemoveWhitelistResponseAuthed {
    Success,
    Failure,
}

pub type RemoveWhitelistResponse = Authorized<RemoveWhitelistResponseAuthed>;
