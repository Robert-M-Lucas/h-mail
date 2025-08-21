use crate::communication::{InterfaceAuthResult, InterfaceResult};
use h_mail_client::communication::get_hmails as c_get_hmails;
use h_mail_client::interface::fields::system_time::SystemTimeField;
use h_mail_client::interface::routes::native::get_hmails::{GetHmailsHmail, GetHmailsRequest};
use h_mail_client::ms_since_epoch_to_system_time;
use tracing::debug;

#[tauri::command]
pub async fn get_hmails(until: Option<i32>, limit: u32) -> InterfaceResult<InterfaceAuthResult<Vec<GetHmailsHmail>>> {
    debug!("get_hmails");
    match c_get_hmails(&GetHmailsRequest::new(until, limit))
    .await
    {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(v.dissolve())),
        Err(e) => e.into(),
    }
}
