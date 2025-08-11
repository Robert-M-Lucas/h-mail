use std::time::SystemTime;
use base64::DecodeError;
use crate::interface::auth::Authorized;
use crate::interface::pow::PowClassification;
use crate::shared::RequestMethod;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};
use crate::interface::email::EmailUser;
use crate::interface::fields::big_uint::BigUintField;
use crate::interface::fields::system_time::SystemTimeField;
use crate::reexports::BigUint;

pub const NATIVE_GET_EMAILS_PATH: &str = "/native/get_emails";
pub const NATIVE_GET_EMAILS_METHOD: RequestMethod = RequestMethod::Get;
pub const NATIVE_GET_EMAILS_REQUIRES_AUTH: bool = true;

/// Requests a user's emails
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetEmailsRequest {
    since: SystemTimeField,
}

/// An individual email in a user's inbox
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetEmailsEmail {
    from: String,
    to: Vec<EmailUser>,
    subject: String,
    sent_at: SystemTimeField,
    received_at: SystemTimeField,
    mime_version: String,
    content_type: String,
    reply_to: Option<EmailUser>,
    cc: Vec<EmailUser>,
    parent: Option<BigUintField>,
    body: String,
    hash: BigUintField,
    pow_classification: PowClassification,
}

impl GetEmailsEmail {
    pub fn decode(self) -> Result<GetEmailsEmailDecoded, DecodeError> {
        let (from, to, subject, sent_at, received_at, mime_version, content_type, reply_to, cc, parent, body, hash, pow_classification) = (self.from, self.to, self.subject, self.sent_at, self.received_at, self.mime_version, self.content_type, self.reply_to, self.cc, self.parent, self.body, self.hash, self.pow_classification);

        let parent = if let Some(parent) = parent {
            Some(parent.decode()?)
        }
        else {
            None
        };

        Ok(GetEmailsEmailDecoded {
            from,
            to,
            subject,
            sent_at: sent_at.decode(),
            received_at: received_at.decode(),
            mime_version,
            content_type,
            reply_to,
            cc,
            parent,
            body,
            hash: hash.decode()?,
            pow_classification,
        })
    }
}

pub struct GetEmailsEmailDecoded {
    from: String,
    to: Vec<EmailUser>,
    subject: String,
    sent_at: SystemTime,
    received_at: SystemTime,
    mime_version: String,
    content_type: String,
    reply_to: Option<EmailUser>,
    cc: Vec<EmailUser>,
    parent: Option<BigUint>,
    body: String,
    hash: BigUint,
    pow_classification: PowClassification,
}


/// Returns the emails in a user's inbox
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub struct GetEmailsResponseAuthed(pub Vec<GetEmailsEmail>);

pub type GetEmailsResponse = Authorized<GetEmailsResponseAuthed>;
