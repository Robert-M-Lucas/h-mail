use crate::interface::RequestMethod;
use crate::interface::auth::Authorized;
use crate::interface::fields::big_uint::BigUintField;
use crate::interface::fields::system_time::SystemTimeField;
use crate::interface::hmail::HmailUser;
use crate::interface::pow::PowClassification;
#[cfg(feature = "client_implementation")]
use crate::reexports::BigUint;
#[cfg(feature = "client_implementation")]
use base64::DecodeError;
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use serde::{Deserialize, Serialize};
#[cfg(feature = "client_implementation")]
use std::time::SystemTime;

pub const NATIVE_GET_HMAILS_PATH: &str = "/native/get_hmails";
pub const NATIVE_GET_HMAILS_METHOD: RequestMethod = RequestMethod::Get;
pub const NATIVE_GET_HMAILS_REQUIRES_AUTH: bool = true;

/// Requests a user's h-mails
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetHmailsRequest {
    until: Option<i64>,
    limit: u32,
    outbox: bool,
}

/// An individual h-mail in a user's inbox
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug, Dissolve)]
pub struct GetHmailsHmail {
    incrementing_id: i64,
    is_context: bool,
    sender: HmailUser,
    recipients: Vec<HmailUser>,
    subject: String,
    sent_at: SystemTimeField,
    received_at: SystemTimeField,
    random_id: u32,
    reply_to: Option<HmailUser>,
    ccs: Vec<HmailUser>,
    parent: Option<BigUintField>,
    body: String,
    hash: BigUintField,
    pow_classification: PowClassification,
}

#[cfg(feature = "client_implementation")]
impl GetHmailsHmail {
    pub fn decode(self) -> Result<GetHmailsHmailDecoded, DecodeError> {
        let (
            is_context,
            sender,
            recipients,
            subject,
            sent_at,
            random_id,
            received_at,
            reply_to,
            ccs,
            parent,
            body,
            hash,
            pow_classification,
        ) = (
            self.is_context,
            self.sender,
            self.recipients,
            self.subject,
            self.sent_at,
            self.random_id,
            self.received_at,
            self.reply_to,
            self.ccs,
            self.parent,
            self.body,
            self.hash,
            self.pow_classification,
        );

        let parent = if let Some(parent) = parent {
            Some(parent.decode()?)
        } else {
            None
        };

        Ok(GetHmailsHmailDecoded {
            is_context,
            sender,
            recipients,
            subject,
            sent_at: sent_at.decode(),
            received_at: received_at.decode(),
            random_id,
            reply_to,
            ccs,
            parent,
            body,
            hash: hash.decode()?,
            pow_classification,
        })
    }
}

#[cfg(feature = "client_implementation")]
#[derive(Getters, new, Debug, Dissolve)]
pub struct GetHmailsHmailDecoded {
    is_context: bool,
    sender: HmailUser,
    recipients: Vec<HmailUser>,
    subject: String,
    sent_at: SystemTime,
    received_at: SystemTime,
    random_id: u32,
    reply_to: Option<HmailUser>,
    ccs: Vec<HmailUser>,
    parent: Option<BigUint>,
    body: String,
    hash: BigUint,
    pow_classification: PowClassification,
}

/// Returns the h-mails in a user's inbox
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Getters, Dissolve, new)]
pub struct GetHmailsResponseAuthed {
    hmails: Vec<GetHmailsHmail>,
}

pub type GetHmailsResponse = Authorized<GetHmailsResponseAuthed>;
