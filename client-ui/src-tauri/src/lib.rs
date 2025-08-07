// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use h_mail_client::communication::check_alive as c_check_alive;
use h_mail_client::set_server_address;

#[tauri::command]
async fn check_alive() -> String {
    if c_check_alive().await.is_ok() {
        "Alive".to_string()
    }
    else {
        "Not Alive".to_string()
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
        .invoke_handler(tauri::generate_handler![check_alive])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
