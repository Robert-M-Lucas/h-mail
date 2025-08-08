use anyhow::Context;
use h_mail_interface::error::HResult;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

static SERVER_ADDRESS: Lazy<RwLock<Option<String>>> = Lazy::new(|| RwLock::new(None));

pub async fn get_server_address() -> HResult<String> {
    let sa = SERVER_ADDRESS.read().await;
    Ok(sa
        .as_ref()
        .context("set_server_address has not been called")?
        .clone())
}

pub async fn set_server_address<T: AsRef<str>>(addr: T) {
    *SERVER_ADDRESS.write().await = Some(addr.as_ref().to_string());
}

static WIPE_OLD_TOKENS: Lazy<RwLock<bool>> = Lazy::new(|| RwLock::new(true));
pub async fn dont_wipe_old_tokens() {
    *WIPE_OLD_TOKENS.write().await = false;
}

pub async fn wipe_old_tokens() -> bool {
    *WIPE_OLD_TOKENS.read().await
}
