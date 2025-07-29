use h_mail_interface::shared::get_url_for_path as interface_get_url_for_path;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

static SERVER_ADDRESS: Lazy<RwLock<Option<String>>> = Lazy::new(|| RwLock::new(None));

pub async fn get_url_for_path<P: AsRef<str>>(path: P) -> String {
    let sa = SERVER_ADDRESS.read().await;
    let addr = sa.as_ref().expect("set_server_address has not been called");
    interface_get_url_for_path(addr, path)
}

pub async fn set_server_address<T: AsRef<str>>(addr: T) {
    *SERVER_ADDRESS.write().await = Some(addr.as_ref().to_string());
}
