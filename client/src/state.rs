use std::path::{Path, PathBuf};
use h_mail_interface::error::HResult;
use h_mail_interface::reexports::anyhow::{anyhow, Context};
use once_cell::sync::{Lazy, OnceCell};
use tokio::fs;
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

static DATA_LOCATION: OnceCell<PathBuf> = OnceCell::new();

pub fn get_data_location() -> HResult<&'static PathBuf> {
    DATA_LOCATION.get().context("set_data_location has not been called")
}
pub fn set_data_location(path: &Path) -> HResult<()> {
    std::fs::create_dir_all(path)?;
    DATA_LOCATION.set(PathBuf::from(path)).map_err(|_| anyhow!("set_data_location called twice"))
}

static WIPE_OLD_TOKENS: Lazy<RwLock<bool>> = Lazy::new(|| RwLock::new(true));
pub async fn dont_wipe_old_tokens() {
    *WIPE_OLD_TOKENS.write().await = false;
}
pub async fn wipe_old_tokens() -> bool {
    *WIPE_OLD_TOKENS.read().await
}
