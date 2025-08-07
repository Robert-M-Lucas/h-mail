// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use h_mail_client::communication::check_alive as c_check_alive;
use h_mail_client::communication::check_auth as c_check_auth;
use h_mail_client::{
    get_server_address, reauthenticate as c_reauthenticate, AuthCredentials, HResult,
};
use h_mail_client::{set_server_address, AnyhowError, AuthError};
use serde::Serialize;
use tokio::fs;

#[tauri::command]
async fn check_alive() -> String {
    if c_check_alive().await.is_ok() {
        "Alive".to_string()
    } else {
        "Not Alive".to_string()
    }
}

#[derive(Serialize)]
enum InterfaceAuthResult<T> {
    Unauthorized,
    Success(T),
}

#[derive(Serialize)]
enum InterfaceResult<T> {
    Ok(T),
    Err(String),
}

impl<T> InterfaceResult<T> {
    pub fn from_error(e: AnyhowError) -> Self {
        InterfaceResult::Err(format!("{e}"))
    }

    pub fn from_hresult(h: HResult<T>) -> Self {
        match h {
            Ok(v) => InterfaceResult::Ok(v),
            Err(e) => Self::from_error(e),
        }
    }
}

#[tauri::command]
async fn check_auth() -> InterfaceResult<InterfaceAuthResult<String>> {
    match c_check_auth().await {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(v.username().clone())),
        Err(e) => match e {
            AuthError::RequireReauth => InterfaceResult::Ok(InterfaceAuthResult::Unauthorized),
            AuthError::Other(e) => InterfaceResult::from_error(e),
        },
    }
}

#[tauri::command]
async fn reauthenticate(username: String, password: String) -> InterfaceResult<String> {
    match c_reauthenticate(AuthCredentials::new(username.clone(), password.to_string())).await {
        Ok(_) => InterfaceResult::Ok(username),
        Err(e) => InterfaceResult::from_error(e),
    }
}

#[tauri::command]
async fn set_server(server: String) {
    set_server_address(&server).await;
    fs::write("server_address", server).await.unwrap();
}

#[tauri::command]
async fn get_server() -> InterfaceResult<String> {
    InterfaceResult::from_hresult(get_server_address().await)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                if let Ok(v) = fs::read_to_string("server_address").await {
                    set_server_address(v).await;
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            set_server,
            get_server,
            check_alive,
            check_auth,
            reauthenticate
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
