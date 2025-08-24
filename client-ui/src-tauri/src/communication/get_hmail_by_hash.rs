use crate::communication::{InterfaceAuthResult, InterfaceResult};
use h_mail_client::communication::get_hmail_by_hash as c_get_hmail_by_hash;
use h_mail_client::interface::fields::big_uint::BigUintField;
use h_mail_client::interface::routes::native::get_hmail_by_hash::GetHmailByHashRequest;
use h_mail_client::interface::routes::native::get_hmails::GetHmailsHmail;
use tracing::debug;

#[tauri::command]
pub async fn get_hmail_by_hash(
    hash: String,
) -> InterfaceResult<InterfaceAuthResult<Option<GetHmailsHmail>>> {
    debug!("get_hmail_by_hash");
    match c_get_hmail_by_hash(&GetHmailByHashRequest::new(BigUintField::from_raw(hash))).await {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(v.dissolve())),
        Err(e) => e.into(),
    }
}
