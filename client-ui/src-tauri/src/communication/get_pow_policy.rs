use crate::communication::{InterfaceAuthResult, InterfaceResult};
use h_mail_client::communication::get_pow_policy as c_get_pow_policy;
use tracing::debug;
use h_mail_client::interface::routes::native::get_pow_policy::{GetPowPolicyResponseAuthed};

#[tauri::command]
pub async fn get_pow_policy(
) -> InterfaceResult<InterfaceAuthResult<GetPowPolicyResponseAuthed>> {
    debug!("get_pow_policy");
    match c_get_pow_policy().await {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(v)),
        Err(e) => e.into(),
    }
}
