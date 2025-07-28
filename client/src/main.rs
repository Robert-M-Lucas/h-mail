use crate::send::send_get_auth;
use h_mail_interface::interface::routes::native::get_emails::{
    GetEmailsRequest, GetEmailsResponse,
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

mod auth;
mod send;
mod util;

pub static SERVER_ADDRESS: Lazy<RwLock<String>> =
    Lazy::new(|| RwLock::new("localhost:8081".to_string()));

pub async fn get_server_path(path: &str) -> String {
    format!("https://{}/{}", SERVER_ADDRESS.read().await, path)
}

#[tokio::main]
async fn main() {
    let emails: GetEmailsResponse = send_get_auth(
        get_server_path("native/get_emails").await,
        &GetEmailsRequest::new(-1),
    )
    .await
    .unwrap();
    println!("{emails:?}");
}
