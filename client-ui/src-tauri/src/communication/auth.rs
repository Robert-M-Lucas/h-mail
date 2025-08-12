use crate::communication::{InterfaceAuthResult, InterfaceResult};
use h_mail_client::communication::check_auth as c_check_auth;
use h_mail_client::logout as c_logout;
use h_mail_client::reauthenticate as c_reauthenticate;
use h_mail_client::{AuthCredentials, AuthError};
use tracing::debug;

#[tauri::command]
pub async fn check_auth() -> InterfaceResult<InterfaceAuthResult<String>> {
    debug!("check_auth");
    match c_check_auth().await {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(v.username().clone())),
        Err(e) => match e {
            AuthError::RequireReauth => InterfaceResult::Ok(InterfaceAuthResult::Unauthorized),
            AuthError::Other(e) => InterfaceResult::from_error(e),
        },
    }
}

#[tauri::command]
pub async fn reauthenticate(username: String, password: String) -> InterfaceResult<String> {
    debug!("reauthenticate");
    match c_reauthenticate(AuthCredentials::new(username.clone(), password.to_string())).await {
        Ok(_) => InterfaceResult::Ok(username),
        Err(e) => InterfaceResult::from_error(e),
    }
}

#[tauri::command]
pub async fn logout() {
    debug!("logout");
    c_logout().await.unwrap();
}
