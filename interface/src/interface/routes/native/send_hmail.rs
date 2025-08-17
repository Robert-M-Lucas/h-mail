use crate::interface::RequestMethod;
use crate::interface::auth::Authorized;
use crate::interface::fields::hmail_address::HmailAddress;
use crate::interface::hmail::SendHmailPackage;
use crate::interface::pow::PowResult;
use crate::interface::routes::foreign::deliver_hmail::DeliverHmailResponse;
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const NATIVE_SEND_HMAIL_PATH: &str = "/native/send_hmail";
pub const NATIVE_SEND_HMAIL_METHOD: RequestMethod = RequestMethod::Post;
pub const NATIVE_SEND_HMAIL_REQUIRES_AUTH: bool = true;

/// Requests the server sends an h-mail to destinations specified in `hmail.to`,
/// `hmail.ccs` and `bccs`.
/// Requires all destinations to have a POW solved in `solved_pows`.
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, Dissolve, new, Debug)]
pub struct SendHmailRequest {
    hmail: SendHmailPackage,
    bccs: Vec<HmailAddress>,
    solved_pows: Vec<SolvedPowFor>,
}

/// Represents POW being solved for one target
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Getters, Dissolve, new, Debug)]
pub struct SolvedPowFor {
    recipient: HmailAddress,
    pow_result: Option<PowResult>,
}

/// The result of trying to send an h-ail to one recipient
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, new)]
pub struct SendHmailResultPerDestination {
    recipient: HmailAddress,
    result: SendHmailResult,
}

/// The result of trying to send an h-mail
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum SendHmailResult {
    DeliveryResult(DeliverHmailResponse),
    Failed,
}

/// Returns whether sending the h-mail succeeded and, if not, why for each recipient
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub enum SendHmailResponseAuthed {
    DeliverResponse(Vec<SendHmailResultPerDestination>),
    MissingPowFor(HmailAddress),
    DuplicateDestination,
    BadRequest,
}

pub type SendHmailResponse = Authorized<SendHmailResponseAuthed>;
