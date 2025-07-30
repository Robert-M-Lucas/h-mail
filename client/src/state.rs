use once_cell::sync::Lazy;
use tokio::sync::RwLock;

static SERVER_ADDRESS: Lazy<RwLock<Option<String>>> = Lazy::new(|| RwLock::new(None));

pub async fn get_server_address() -> String {
    let sa = SERVER_ADDRESS.read().await;
    sa.as_ref()
        .expect("set_server_address has not been called")
        .clone()
}

pub async fn set_server_address<T: AsRef<str>>(addr: T) {
    *SERVER_ADDRESS.write().await = Some(addr.as_ref().to_string());
}
