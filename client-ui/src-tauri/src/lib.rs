// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use serde::{Deserialize, Serialize};
use h_mail_client::communication::check_alive as c_check_alive;
use h_mail_client::communication::check_auth as c_check_auth;
use h_mail_client::{reauthenticate as c_reauthenticate, AuthCredentials};
use h_mail_client::{set_server_address, AnyhowError, AuthError};


#[tauri::command]
async fn check_alive() -> String {
    if c_check_alive().await.is_ok() {
        "Alive".to_string()
    }
    else {
        "Not Alive".to_string()
    }
}

#[derive(Serialize)]
enum InterfaceAuthResult<T> {
    Unauthorized,
    Success(T)
}

#[derive(Serialize)]
enum InterfaceResult<T> {
    Ok(T),
    Err(String)
}

impl<T> InterfaceResult<T> {
    pub fn from_error(e: AnyhowError) -> Self {
        InterfaceResult::Err(format!("{}", e))
    }
}


#[tauri::command]
async fn check_auth() -> InterfaceResult<InterfaceAuthResult<String>> {
    match c_check_auth().await {
        Ok(v) => {
            InterfaceResult::Ok(InterfaceAuthResult::Success(v.username().clone()))
        }
        Err(e) => {
            match e {
                AuthError::RequireReauth => {
                    InterfaceResult::Ok(InterfaceAuthResult::Unauthorized)
                }
                AuthError::Other(e) => {
                    InterfaceResult::from_error(e)
                }
            }
        }
    }
}

#[tauri::command]
async fn reauthenticate(username: String, password: String) -> InterfaceResult<String> {
    match c_reauthenticate(AuthCredentials::new(username.clone(), password.to_string())).await {
        Ok(_) => {
            InterfaceResult::Ok(username)
        }
        Err(e) => {
            InterfaceResult::from_error(e)
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                set_server_address("127.0.0.1:8081").await
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![check_alive, check_auth, reauthenticate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
