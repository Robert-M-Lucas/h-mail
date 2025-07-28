use crate::interface::auth::Authorized;
use crate::interface::email::EmailPackage;
use crate::interface::routes::foreign::deliver_email::DeliverEmailResponse;
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, Dissolve, new, Debug)]
pub struct SendEmailRequest {
    package: EmailPackage,
    destination_domain: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SendEmailResponseAuthed {
    DeliverResponse(DeliverEmailResponse),
    SendingFailed,
}

pub type SendEmailResponse = Authorized<SendEmailResponseAuthed>;
