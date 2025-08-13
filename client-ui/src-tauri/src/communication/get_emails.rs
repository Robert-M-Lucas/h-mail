use crate::communication::{InterfaceAuthResult, InterfaceResult};
use h_mail_client::communication::get_emails as c_get_emails;
use h_mail_client::interface::fields::system_time::SystemTimeField;
use h_mail_client::interface::routes::native::get_emails::{GetEmailsEmail, GetEmailsRequest};
use h_mail_client::{ms_since_epoch_to_system_time, AuthError};
use tracing::debug;

#[tauri::command]
pub async fn get_emails(since: u64) -> InterfaceResult<InterfaceAuthResult<Vec<GetEmailsEmail>>> {
    debug!("get_emails");
    match c_get_emails(&GetEmailsRequest::new(SystemTimeField::new(
        &ms_since_epoch_to_system_time(since as u128),
    )))
    .await
    {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(v.dissolve())),
        Err(e) => match e {
            AuthError::RequireReauth => InterfaceResult::Ok(InterfaceAuthResult::Unauthorized),
            AuthError::Other(e) => InterfaceResult::from_error(e),
        },
    }
}
