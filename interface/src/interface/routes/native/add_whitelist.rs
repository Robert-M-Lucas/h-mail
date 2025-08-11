use crate::interface::auth::Authorized;
use crate::interface::pow::PowClassification;
use crate::shared::RequestMethod;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const NATIVE_ADD_WHITELIST_PATH: &str = "/native/add_whitelist";
pub const NATIVE_ADD_WHITELIST_METHOD: RequestMethod = RequestMethod::Post;
pub const NATIVE_ADD_WHITELIST_REQUIRES_AUTH: bool = true;

/// Adds an address to the authenticated user's whitelist
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct AddWhitelistRequest {
    address: String,
    place_into: PowClassification
}

/// Returns whether the request succeeded
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum AddWhitelistResponse {
    Success
}

pub type IsWhitelistedResponse = Authorized<AddWhitelistResponse>;
