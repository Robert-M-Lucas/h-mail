use crate::interface::auth::Authorized;
use crate::interface::pow::PowClassification;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const NATIVE_GET_EMAILS_PATH: &str = "/native/get_emails";

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetEmailsRequest {
    since_id: i64,
}

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetEmailsEmail {
    source: String,
    email: String,
    pow_classification: PowClassification,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetEmailsResponseAuthed(pub Vec<GetEmailsEmail>);

pub type GetEmailsResponse = Authorized<GetEmailsResponseAuthed>;
