use std::collections::HashMap;
use crate::interface::auth::Authorized;
use crate::interface::email::{Email, SendEmailPackage};
use crate::interface::routes::foreign::deliver_email::DeliverEmailResponse;
use crate::shared::RequestMethod;
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use serde::{Deserialize, Serialize};
use crate::interface::pow::PowResult;

pub const NATIVE_SEND_EMAIL_PATH: &str = "/native/send_email";
pub const NATIVE_SEND_EMAIL_METHOD: RequestMethod = RequestMethod::Post;
pub const NATIVE_SEND_EMAIL_REQUIRES_AUTH: bool = true;

/// Requests the server sends an email to destinations specified in `email`.
/// Requires all destinations to have a POW solved in `solved_pows`.
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, Dissolve, new, Debug)]
pub struct SendEmailRequest {
    email: SendEmailPackage,
    solved_pows: Vec<SolvedPowFor>
}

/// Represents POW being solved for one target
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, Dissolve, new, Debug)]
pub struct SolvedPowFor {
    target_user: String,
    pow_result: PowResult
}

/// Returns whether sending the email succeeded and, if not, why
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum SendEmailResponseAuthed {
    DeliverResponse(DeliverEmailResponse),
    SendingFailed,
}

pub type SendEmailResponse = Authorized<SendEmailResponseAuthed>;
