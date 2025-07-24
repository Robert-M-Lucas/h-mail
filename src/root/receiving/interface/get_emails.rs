use crate::root::receiving::interface::shared::PowClassification;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetEmailsRequest {
    username: String,
    since_id: i64,
}

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetEmailsEmail {
    source: String,
    email: String,
    pow_classification: PowClassification,
}

#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetEmailsResponse {
    emails: Option<Vec<GetEmailsEmail>>,
}

impl GetEmailsResponse {
    pub fn get_emails(self) -> Option<Vec<GetEmailsEmail>> {
        self.emails
    }
}
