use crate::interface::auth::Authorized;
use crate::interface::pow::PowClassification;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};
use crate::shared::RequestMethod;

pub const NATIVE_GET_EMAILS_PATH: &str = "/native/get_emails";
pub const NATIVE_GET_EMAILS_METHOD: RequestMethod = RequestMethod::Get;
pub const NATIVE_GET_EMAILS_REQUIRES_AUTH: bool = true;

/// GET: Requests a user's emails
///
/// AUTH: Requires an access token as the bearer token
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetEmailsRequest {
    since_id: i32,
}

/// An individual email in a user's inbox
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetEmailsEmail {
    source: String,
    email: String,
    pow_classification: PowClassification,
}

/// Returns the emails in a user's inbox
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub struct GetEmailsResponseAuthed(pub Vec<GetEmailsEmail>);

pub type GetEmailsResponse = Authorized<GetEmailsResponseAuthed>;
