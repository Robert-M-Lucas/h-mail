use crate::interface::RequestMethod;
use crate::interface::auth::Authorized;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const AUTH_CHECK_AUTH_PATH: &str = "/auth/check_auth";
pub const AUTH_CHECK_AUTH_METHOD: RequestMethod = RequestMethod::Get;
pub const AUTH_CHECK_AUTH_REQUIRES_AUTH: bool = true;

/// Checks whether a user is authenticated without making a specific request
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, new, Debug)]
pub struct CheckAuthRequest {}

/// Returns the name of the user, should the user be authenticated
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, new, Debug, Getters)]
pub struct CheckAuthResponseAuthed {
    username: String,
}

pub type CheckAuthResponse = Authorized<CheckAuthResponseAuthed>;
