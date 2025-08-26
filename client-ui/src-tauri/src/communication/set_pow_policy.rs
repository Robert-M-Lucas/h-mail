use crate::communication::{InterfaceAuthResult, InterfaceResult};
use h_mail_client::communication::set_pow_policy as c_set_pow_policy;
use tracing::debug;
use h_mail_client::interface::pow::{PowIters, PowPolicy};
use h_mail_client::interface::routes::native::set_pow_policy::{SetPowPolicyRequest, SetPowPolicyResponseAuthed};

#[tauri::command]
pub async fn set_pow_policy(
    minimum: PowIters,
    accepted: PowIters,
    personal: PowIters
) -> InterfaceResult<InterfaceAuthResult<SetPowPolicyResponseAuthed>> {
    debug!("set_pow_policy");
    match c_set_pow_policy(&SetPowPolicyRequest::new(PowPolicy::new(minimum, accepted, personal))).await {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(v)),
        Err(e) => e.into(),
    }
}
