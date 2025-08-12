use crate::interface::auth::Authorized;
use crate::interface::pow::PowClassification;
use crate::shared::RequestMethod;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const NATIVE_GET_WHITELIST_PATH: &str = "/native/get_whitelist";
pub const NATIVE_GET_WHITELIST_METHOD: RequestMethod = RequestMethod::Get;
pub const NATIVE_GET_WHITELIST_REQUIRES_AUTH: bool = true;

/// Requests the authenticated user's whitelist
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetWhitelistRequest {}

/// Whitelist entry
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Getters, new)]
pub struct WhitelistEntry {
    address: String,
    place_in: PowClassification,
}

/// Returns the user's whitelist
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Getters, new)]
pub struct GetWhitelistResponseAuthed {
    whitelist: Vec<WhitelistEntry>,
}

pub type GetWhitelistResponse = Authorized<GetWhitelistResponseAuthed>;
