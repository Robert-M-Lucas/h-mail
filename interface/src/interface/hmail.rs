use crate::interface::fields::big_uint::BigUintField;
use crate::interface::fields::hmail_address::HmailAddress;
use crate::interface::fields::system_time::SystemTimeField;
use crate::interface::pow::{PowHash, WithPow};
#[cfg(feature = "client_implementation")]
use crate::interface::pow::{PowHashComponent, St};
#[cfg(feature = "client_implementation")]
use base64::DecodeError;
use derive_getters::{Dissolve, Getters};
use derive_new::new;
#[cfg(feature = "client_implementation")]
use rand::{RngCore, thread_rng};
#[cfg(feature = "client_implementation")]
use rsa::BigUint;
use serde::{Deserialize, Serialize};
#[cfg(feature = "client_implementation")]
use sha2::{Digest, Sha256};
#[cfg(feature = "client_implementation")]
use std::time::SystemTime;

/// Represents a h-mail address, with an optional display name
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Clone, Debug, Getters, new, Dissolve)]
pub struct HmailUser {
    address: HmailAddress,
    display_name: Option<String>,
}

#[cfg(feature = "client_implementation")]
#[derive(Clone, Debug, Getters, Dissolve)]
pub struct HmailPackage {
    sender: HmailUser,
    recipients: Vec<HmailUser>,
    subject: String,
    sent_at: SystemTime,
    /// If two h-mails are the same / have the same hash, differentiate them
    random_id: u32,
    reply_to: Option<HmailUser>,
    ccs: Vec<HmailUser>,
    parent: Option<BigUint>,
    body: String,
}

#[cfg(feature = "client_implementation")]
impl HmailPackage {
    pub fn new<
        T: Iterator<Item = HmailUser>,
        S: AsRef<str>,
        T2: Iterator<Item = HmailUser>,
        S4: AsRef<str>,
    >(
        sender: HmailUser,
        recipients: T,
        subject: S,
        reply_to: Option<HmailUser>,
        ccs: T2,
        parent: Option<BigUint>,
        body: S4,
    ) -> Self {
        Self {
            sender,
            recipients: recipients.collect(),
            subject: subject.as_ref().to_string(),
            sent_at: SystemTime::now(),
            random_id: thread_rng().next_u32(),
            reply_to,
            ccs: ccs.collect(),
            parent,
            body: body.as_ref().to_string(),
        }
    }

    pub fn encode(self) -> SendHmailPackage {
        let (sender, recipients, subject, sent_at, random_id, reply_to, ccs, parent, body) =
            self.dissolve();
        SendHmailPackage {
            sender,
            recipients,
            subject,
            sent_at: SystemTimeField::new(&sent_at),
            random_id,
            reply_to,
            ccs,
            parent: parent.map(|p| BigUintField::new(&p)),
            body,
        }
    }
}

pub type Hmail = WithPow<SendHmailPackage>;

/// Represents an email being sent. The email's hash is used to identify an email uniquely (for
/// replying to emails), with the `random_id` being used to differentiate two exactly identical
/// emails. As the `random_id` is client-chosen, the hash of the email should not be used as a UID
/// for servers as a client can easily construct two emails with identical hashes.
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Clone, Debug, Getters, Dissolve, new)]
pub struct SendHmailPackage {
    sender: HmailUser,
    recipients: Vec<HmailUser>,
    subject: String,
    sent_at: SystemTimeField,
    /// If two emails are the same / have the same hash, differentiate them
    random_id: u32,
    reply_to: Option<HmailUser>,
    ccs: Vec<HmailUser>,
    parent: Option<BigUintField>,
    body: String,
}

#[cfg(feature = "client_implementation")]
impl SendHmailPackage {
    pub fn decode(self) -> Result<HmailPackage, DecodeError> {
        let (sender, recipients, subject, sent_at, random_id, reply_to, cc, parent, body) = self.dissolve();

        let parent = if let Some(parent) = parent {
            Some(parent.decode()?)
        } else {
            None
        };

        Ok(HmailPackage {
            sender,
            recipients,
            subject,
            sent_at: sent_at.decode(),
            random_id,
            reply_to,
            ccs: cc,
            parent,
            body,
        })
    }
}

impl PowHash for SendHmailPackage {
    #[cfg(feature = "client_implementation")]
    fn pow_hash(&self) -> BigUint {
        let mut s: St = Sha256::new();

        let update_with_email_user = |s: &mut St, email_user: &HmailUser| {
            email_user.address.update_hash(s);
            if let Some(display_name) = &email_user.display_name {
                s.update([1u8]);
                s.update(display_name.as_bytes());
            } else {
                s.update([0u8]);
            }
        };

        s.update(self.recipients.len().to_le_bytes());
        for user in &self.recipients {
            update_with_email_user(&mut s, user);
        }

        self.sent_at.update_hash(&mut s);
        s.update(self.random_id.to_le_bytes());
        if let Some(reply_to) = &self.reply_to {
            s.update([1u8]);
            update_with_email_user(&mut s, reply_to);
        } else {
            s.update([0u8]);
        }

        s.update(self.ccs.len().to_le_bytes());
        for user in &self.ccs {
            update_with_email_user(&mut s, user);
        }

        if let Some(parent) = &self.parent {
            s.update([1u8]);
            parent.update_hash(&mut s);
        } else {
            s.update([0u8]);
        }

        s.update(self.body.as_bytes());

        BigUint::from_bytes_le(&s.finalize())
    }
}
