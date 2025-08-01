use crate::interface::auth::Authorized;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const AUTH_CHECK_AUTH_PATH: &str = "/auth/check_auth";

/// GET: Checks whether a refresh token is valid
///
/// AUTH: Requires an access token as the bearer token
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, new, Debug)]
pub struct CheckAuthRequest;

/// Returns the name of the user, should the user be authorised
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, new, Debug, Getters)]
pub struct CheckAuthResponseAuthed {
    username: String,
}

pub type CheckAuthResponse = Authorized<CheckAuthResponseAuthed>;
