use crate::interface::auth::Authorized;
use crate::interface::email::Email;
use crate::interface::routes::foreign::deliver_email::DeliverEmailResponse;
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const NATIVE_SEND_EMAIL_PATH: &str = "/native/send_email";

/// POST: Requests the server sends an email to another server
///
/// AUTH: Requires an access token as the bearer token
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, Dissolve, new, Debug)]
pub struct SendEmailRequest {
    email: Email,
    destination_domain: String,
}

/// Returns whether sending the email succeeded and, if not, why
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum SendEmailResponseAuthed {
    DeliverResponse(DeliverEmailResponse),
    SendingFailed,
}

pub type SendEmailResponse = Authorized<SendEmailResponseAuthed>;
