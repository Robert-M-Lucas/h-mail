use crate::interface::auth::Authorized;
use crate::interface::fields::big_uint::BigUintField;
use crate::interface::routes::native::get_hmails::GetHmailsHmail;
use crate::interface::RequestMethod;
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const NATIVE_GET_HMAIL_BY_HASH_PATH: &str = "/native/get_hmail_by_hash";
pub const NATIVE_GET_HMAIL_BY_HASH_METHOD: RequestMethod = RequestMethod::Get;
pub const NATIVE_GET_HMAIL_BY_HASH_REQUIRES_AUTH: bool = true;

/// Requests a user's h-mails
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetHmailByHashRequest {
    hash: BigUintField
}

/// Returns the h-mails in a user's inbox
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Getters, Dissolve, new)]
pub struct GetHmailByHashResponseAuthed {
    hmail: Option<GetHmailsHmail>,
}

pub type GetHmailByHashResponse = Authorized<GetHmailByHashResponseAuthed>;
