use crate::communication::{InterfaceAuthResult, InterfaceResult};
use h_mail_client::communication::{check_auth as c_check_auth, check_version};
use h_mail_client::logout as c_logout;
use h_mail_client::reauthenticate as c_reauthenticate;
use h_mail_client::AuthCredentials;
use tracing::debug;

#[tauri::command]
pub async fn check_auth() -> InterfaceResult<InterfaceAuthResult<String>> {
    debug!("check_auth");
    if let Err(e) = check_version().await {
        return InterfaceResult::Err(e.to_string())
    }
    match c_check_auth().await {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(v.username().clone())),
        Err(e) => e.into(),
    }
}

#[tauri::command]
pub async fn reauthenticate(username: String, password: String) -> InterfaceResult<String> {
    debug!("reauthenticate");
    if let Err(e) = check_version().await {
        return InterfaceResult::Err(e.to_string())
    }
    match c_reauthenticate(AuthCredentials::new(username.clone(), password.to_string())).await {
        Ok(_) => InterfaceResult::Ok(username),
        Err(e) => e.into(),
    }
}

#[tauri::command]
pub async fn logout() {
    debug!("logout");
    c_logout().await.unwrap();
}
