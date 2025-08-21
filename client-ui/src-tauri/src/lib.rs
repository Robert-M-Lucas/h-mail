use crate::communication::InterfaceResult;
use h_mail_client::communication::check_alive as c_check_alive;
use h_mail_client::get_server_address;
use h_mail_client::set_server_address;
use std::sync::OnceLock;
use tauri::AppHandle;
use tokio::fs;
use tracing::debug;

pub mod communication;
pub mod pow_manager;

#[tauri::command]
async fn check_alive() -> String {
    debug!("check_alive");
    if c_check_alive().await.is_ok() {
        "Alive".to_string()
    } else {
        "Not Alive".to_string()
    }
}

#[tauri::command]
async fn set_server(server: String) {
    debug!("set_server");
    set_server_address(&server).await;
    fs::write("server_address", server).await.unwrap();
}

#[tauri::command]
async fn get_server() -> InterfaceResult<String> {
    debug!("get_server");
    get_server_address().await.into()
}

pub static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tauri::Builder::default()
        .setup(|app| {
            APP_HANDLE.set(app.handle().clone()).unwrap();
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
            communication::auth::check_auth,
            communication::auth::reauthenticate,
            communication::auth::logout,
            communication::create_account::create_account,
            communication::create_account::create_account_requirement,
            communication::whitelist_management::get_whitelist,
            communication::whitelist_management::remove_whitelist,
            communication::whitelist_management::add_whitelist,
            communication::get_hmails::get_hmails,
            communication::send_hmail::send_hmail,
            pow_manager::estimate_performance,
            pow_manager::load_estimate,
            pow_manager::cancel_current_pow
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
