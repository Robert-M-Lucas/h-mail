use crate::interface::auth::Authorized;
use crate::interface::pow::PowClassification;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const NATIVE_GET_EMAILS_PATH: &str = "/native/get_emails";

#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetEmailsRequest {
    since_id: i32,
}

#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetEmailsEmail {
    source: String,
    email: String,
    pow_classification: PowClassification,
}

#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub struct GetEmailsResponseAuthed(pub Vec<GetEmailsEmail>);

pub type GetEmailsResponse = Authorized<GetEmailsResponseAuthed>;
