use crate::communication::{InterfaceAuthResult, InterfaceResult};
use h_mail_client::communication::get_foreign_pow_policy as c_get_foreign_pow_policy;
use h_mail_client::interface::fields::hmail_address::HmailAddress;
use h_mail_client::interface::routes::native::get_foreign_pow_policy::{
    GetForeignPowPolicyRequest, GetForeignPowPolicyResponseAuthed,
};
use tracing::debug;

#[tauri::command]
pub async fn get_foreign_pow_policy(
    recipient: HmailAddress,
) -> InterfaceResult<InterfaceAuthResult<GetForeignPowPolicyResponseAuthed>> {
    debug!("get_foreign_pow_policy");
    match c_get_foreign_pow_policy(&GetForeignPowPolicyRequest::new(recipient)).await {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(v)),
        Err(e) => e.into(),
    }
}
