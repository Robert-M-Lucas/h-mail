use crate::communication::InterfaceResult;
use directories_next::ProjectDirs;
use h_mail_client::communication::check_alive as c_check_alive;
use h_mail_client::interface::fields::hmail_address::HmailAddress;
use h_mail_client::set_server_address;
use h_mail_client::{get_data_location, get_server_address, set_data_location};
use std::path::PathBuf;
use std::sync::OnceLock;
use tauri::AppHandle;
use tokio::fs;
use tracing::{debug, error, info};

pub mod communication;
pub mod pow_manager;

#[tauri::command]
async fn check_alive() -> String {
    debug!("check_alive");
    if let Err(e) = c_check_alive().await {
        e.to_string()
    } else {
        "Alive".to_string()
    }
}

fn server_address_file() -> PathBuf {
    get_data_location().unwrap().join("server_address")
}

#[tauri::command]
async fn set_server(server: String) -> Option<String> {
    debug!("set_server");
    if let Err(e) = set_server_address(&server).await {
        return Some(format!("Failed to set server: {}", e));
    }
    let file = server_address_file();
    if fs::write(file, server).await.is_err() {
        error!("Failed to write server address to file")
    };
    None
}

#[tauri::command]
async fn get_server() -> InterfaceResult<String> {
    debug!("get_server");
    get_server_address().await.into()
}

#[tauri::command]
async fn validate_hmail(address: String) -> bool {
    debug!("validate_hmail");
    HmailAddress::new(&address).is_ok()
}

pub static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let p_dirs = ProjectDirs::from("com", "Robert", "HMailClient").unwrap();
    let data_dir = p_dirs.data_dir();
    info!("Data dir: `{}`", data_dir.to_str().unwrap());
    set_data_location(data_dir).unwrap();

    tauri::Builder::default()
        .setup(|app| {
            APP_HANDLE.set(app.handle().clone()).unwrap();
            tauri::async_runtime::block_on(async {
                if let Ok(v) = fs::read_to_string(server_address_file()).await {
                    if let Err(e) = set_server_address(&v).await {
                        error!("Error setting server address on file '{v}': {e}")
                    };
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            set_server,
            get_server,
            check_alive,
            validate_hmail,
            communication::auth::check_auth,
            communication::auth::reauthenticate,
            communication::auth::logout,
            communication::create_account::create_account,
            communication::create_account::create_account_requirement,
            communication::whitelist_management::get_whitelist,
            communication::whitelist_management::remove_whitelist,
            communication::whitelist_management::add_whitelist,
            communication::get_hmails::get_hmails,
            communication::get_hmail_by_hash::get_hmail_by_hash,
            communication::send_hmail::send_hmail,
            communication::get_foreign_pow_policy::get_foreign_pow_policy,
            communication::get_pow_policy::get_pow_policy,
            communication::set_pow_policy::set_pow_policy,
            pow_manager::estimate_performance,
            pow_manager::load_estimate,
            pow_manager::cancel_current_pow
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
