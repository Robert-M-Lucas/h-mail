use std::time::SystemTime;
use base64::DecodeError;
use crate::interface::fields::big_uint::BigUintField;
use crate::interface::pow::{PowHash, WithPow};
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use rand::{RngCore, thread_rng};
use rsa::BigUint;
use rsa::signature::digest::consts::U32;
use rsa::signature::digest::core_api::{CoreWrapper, CtVariableCoreWrapper};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha256VarCore};
use crate::interface::fields::system_time::SystemTimeField;
// #[derive(Serialize, Deserialize, Debug, new, Getters)]
// pub struct Email {
//     email: EmailPackage,
//     iters: PowIters,
//     token: BigUintField,
//     pow_result: BigUintField,
//
// }

/// Represents an email address, with an optional display name
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Clone, Debug, Getters, new)]
pub struct EmailUser {
    email: String,
    display_name: Option<String>
}

#[derive(Clone, Debug, Getters, Dissolve)]
pub struct EmailPackage {
    to: Vec<EmailUser>,
    subject: String,
    sent_at: SystemTime,
    /// If two emails are the same / have the same hash, differentiate them
    random_id: u32,
    mime_version: String,
    content_type: String,
    reply_to: Option<EmailUser>,
    cc: Vec<EmailUser>,
    parent: Option<BigUint>,
    body: String,
}

impl EmailPackage {
    pub fn new<T: Iterator<Item = EmailUser>, S: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>, T2: Iterator<Item = EmailUser>, S4: AsRef<str>>(
        to: T,
        subject: S,
        mime_version: S2,
        content_type: S3,
        reply_to: Option<EmailUser>,
        cc: T2,
        parent: Option<BigUint>,
        body: S4
    ) -> Self {
        Self {
            to: to.collect(),
            subject: subject.as_ref().to_string(),
            sent_at: SystemTime::now(),
            random_id: thread_rng().next_u32(),
            mime_version: mime_version.as_ref().to_string(),
            content_type: content_type.as_ref().to_string(),
            reply_to,
            cc: cc.collect(),
            parent,
            body: body.as_ref().to_string()
        }
    }

    pub fn encode(self) -> SendEmailPackage {
        let (to, subject, sent_at, random_id, mime_version, content_type, reply_to, cc, parent, body) = self.dissolve();
        SendEmailPackage {
            to,
            subject,
            sent_at: SystemTimeField::new(&sent_at),
            random_id,
            mime_version,
            content_type,
            reply_to,
            cc,
            parent: parent.map(|p| BigUintField::new(&p)),
            body,
        }
    }
}

pub type Email = WithPow<SendEmailPackage>;

/// Represents an email being sent. The email's hash is used to identify an email uniquely (for
/// replying to emails), with the `random_id` being used to differentiate two exactly identical
/// emails. As the `random_id` is client-chosen, the hash of the email should not be used as a UID
/// for servers as a client can easily construct two emails with identical hashes.
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Clone, Debug, Getters, Dissolve)]
pub struct SendEmailPackage {
    to: Vec<EmailUser>,
    subject: String,
    sent_at: SystemTimeField,
    /// If two emails are the same / have the same hash, differentiate them
    random_id: u32,
    mime_version: String,
    content_type: String,
    reply_to: Option<EmailUser>,
    cc: Vec<EmailUser>,
    parent: Option<BigUintField>,
    body: String,
}

impl SendEmailPackage {
    pub fn decode(self) -> Result<EmailPackage, DecodeError> {
        let (to, subject, sent_at, random_id, mime_version, content_type, reply_to, cc, parent, body) = self.dissolve();

        let parent = if let Some(parent) = parent {
            Some(parent.decode()?)
        } else { None };

        Ok(EmailPackage {
            to,
            subject,
            sent_at: sent_at.decode(),
            random_id,
            mime_version,
            content_type,
            reply_to,
            cc,
            parent,
            body,
        })
    }
}

impl PowHash for SendEmailPackage {
    fn pow_hash(&self) -> BigUint {
        type St = CoreWrapper<CtVariableCoreWrapper<Sha256VarCore, U32, sha2::OidSha256>>;
        let mut s: St = Sha256::new();

        let update_with_email_user = |s: &mut St, email_user: &EmailUser| {
            s.update(&email_user.email);
            if let Some(display_name) = &email_user.display_name {
                s.update(&[1u8]);
                s.update(display_name.as_bytes());
            }
            else {
                s.update(&[0u8]);
            }
        };

        s.update(&self.to.len().to_le_bytes());
        for user in &self.to {
            update_with_email_user(&mut s, user);
        }

        s.update(&self.sent_at.bytes_for_hash());
        s.update(&self.random_id.to_le_bytes());
        s.update(self.mime_version.as_bytes());
        s.update(&self.content_type.as_bytes());
        if let Some(reply_to) = &self.reply_to {
            s.update(&[1u8]);
            update_with_email_user(&mut s, reply_to);
        }
        else {
            s.update(&[0u8]);
        }

        s.update(&self.cc.len().to_le_bytes());
        for user in &self.cc {
            update_with_email_user(&mut s, user);
        }

        if let Some(parent) = &self.parent {
            s.update(&[1u8]);
            s.update(parent.bytes_for_hash());
        }
        else {
            s.update(&[0u8]);
        }

        s.update(&self.body.as_bytes());

        BigUint::from_bytes_le(&s.finalize())
    }
}
