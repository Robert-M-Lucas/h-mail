use crate::root::receiving::interface::shared::PowClassification;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

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
pub enum GetEmailsResponse {
    NotAuthorized,
    Emails(Vec<GetEmailsEmail>),
}

