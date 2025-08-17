use crate::interface::auth::Authorized;
use crate::interface::hmail::HmailUser;
use crate::interface::fields::big_uint::BigUintField;
use crate::interface::fields::system_time::SystemTimeField;
use crate::interface::pow::PowClassification;
use crate::reexports::BigUint;
use crate::shared::RequestMethod;
use base64::DecodeError;
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub const NATIVE_GET_HMAILS_PATH: &str = "/native/get_hmails";
pub const NATIVE_GET_HMAILS_METHOD: RequestMethod = RequestMethod::Get;
pub const NATIVE_GET_HMAILS_REQUIRES_AUTH: bool = true;

/// Requests a user's h-mails
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetHmailsRequest {
    since: SystemTimeField,
}

/// An individual h-mail in a user's inbox
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, new, Debug)]
pub struct GetHmailsHmail {
    source: String,
    to: Vec<HmailUser>,
    subject: String,
    sent_at: SystemTimeField,
    received_at: SystemTimeField,
    reply_to: Option<HmailUser>,
    cc: Vec<HmailUser>,
    parent: Option<BigUintField>,
    body: String,
    hash: BigUintField,
    pow_classification: PowClassification,
}

impl GetHmailsHmail {
    pub fn decode(self) -> Result<GetHmailsHmailDecoded, DecodeError> {
        let (
            source,
            to,
            subject,
            sent_at,
            received_at,
            reply_to,
            cc,
            parent,
            body,
            hash,
            pow_classification,
        ) = (
            self.source,
            self.to,
            self.subject,
            self.sent_at,
            self.received_at,
            self.reply_to,
            self.cc,
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
            source,
            to,
            subject,
            sent_at: sent_at.decode(),
            received_at: received_at.decode(),
            reply_to,
            cc,
            parent,
            body,
            hash: hash.decode()?,
            pow_classification,
        })
    }
}

pub struct GetHmailsHmailDecoded {
    source: String,
    to: Vec<HmailUser>,
    subject: String,
    sent_at: SystemTime,
    received_at: SystemTime,
    reply_to: Option<HmailUser>,
    cc: Vec<HmailUser>,
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
