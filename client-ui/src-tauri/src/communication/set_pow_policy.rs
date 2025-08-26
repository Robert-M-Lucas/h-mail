use crate::communication::{InterfaceAuthResult, InterfaceResult};
use h_mail_client::communication::set_pow_policy as c_set_pow_policy;
use h_mail_client::interface::pow::PowPolicy;
use h_mail_client::interface::routes::native::set_pow_policy::{
    SetPowPolicyRequest, SetPowPolicyResponseAuthed,
};
use tracing::debug;

#[tauri::command]
pub async fn set_pow_policy(
    policy: PowPolicy,
) -> InterfaceResult<InterfaceAuthResult<SetPowPolicyResponseAuthed>> {
    debug!("set_pow_policy");
    match c_set_pow_policy(&SetPowPolicyRequest::new(policy)).await {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(v)),
        Err(e) => e.into(),
    }
}
