use crate::communication::{InterfaceAuthResult, InterfaceResult};
use h_mail_client::communication::get_hmails as c_get_hmails;
use h_mail_client::interface::routes::native::get_hmails::{GetHmailsHmail, GetHmailsRequest};
use tracing::debug;

#[tauri::command]
pub async fn get_hmails(
    until: Option<i64>,
    limit: u32,
    outbox: bool,
) -> InterfaceResult<InterfaceAuthResult<Vec<GetHmailsHmail>>> {
    debug!("get_hmails");
    match c_get_hmails(&GetHmailsRequest::new(until, limit, outbox)).await {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(v.dissolve())),
        Err(e) => e.into(),
    }
}
