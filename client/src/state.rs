use h_mail_interface::error::HResult;
use h_mail_interface::interface::SERVER_PORT;
use h_mail_interface::reexports::anyhow::{Context, anyhow, bail};
use once_cell::sync::{Lazy, OnceCell};
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;

static SERVER_ADDRESS: Lazy<RwLock<Option<String>>> = Lazy::new(|| RwLock::new(None));

pub async fn get_server_address() -> HResult<String> {
    let sa = SERVER_ADDRESS.read().await;
    Ok(sa
        .as_ref()
        .context("set_server_address has not been called")?
        .clone())
}
pub async fn set_server_address<T: AsRef<str>>(addr: T) -> HResult<()> {
    let domain = addr.as_ref().trim();
    if domain.is_empty() {
        bail!("Address must not be empty");
    }
    let mut domain_port = domain.split(':');
    let domain = domain_port.next().unwrap();
    let port = if let Some(port) = domain_port.next() {
        Some(
            port.parse::<u16>()
                .map_err(|_| anyhow!("Invalid port number"))?,
        )
    } else {
        None
    };

    if domain_port.next().is_some() {
        bail!("Cannot have multiple colons in address")
    }

    if domain.len() > 253 {
        bail!("Domain bust be less than 253 characters")
    }
    if domain.is_empty() {
        bail!("Domain must not be empty");
    }

    for label in domain.split('.') {
        if label.is_empty() {
            bail!("Domain cannot start with, end with, or have two consecutive dots");
        }
        if label.len() > 63 {
            bail!("Dot-separated labels cannot be longer than 63 characters");
        }
        if !label.chars().all(|c| c.is_alphanumeric() || c == '-') {
            bail!("Domain can only have alphanumeric and - characters")
        }
        if label.starts_with('-') || label.ends_with('-') {
            bail!("Dot-separated labels cannot start or end with '-'");
        }
    }

    let domain = if let Some(port) = port {
        format!("{domain}:{port}")
    } else {
        format!("{domain}:{SERVER_PORT}")
    };

    *SERVER_ADDRESS.write().await = Some(domain);

    Ok(())
}

static DATA_LOCATION: OnceCell<PathBuf> = OnceCell::new();

pub fn get_data_location() -> HResult<&'static PathBuf> {
    DATA_LOCATION
        .get()
        .context("set_data_location has not been called")
}
pub fn set_data_location(path: &Path) -> HResult<()> {
    std::fs::create_dir_all(path)?;
    DATA_LOCATION
        .set(PathBuf::from(path))
        .map_err(|_| anyhow!("set_data_location called twice"))
}

static WIPE_OLD_TOKENS: Lazy<RwLock<bool>> = Lazy::new(|| RwLock::new(true));
pub async fn dont_wipe_old_tokens() {
    *WIPE_OLD_TOKENS.write().await = false;
}
pub async fn wipe_old_tokens() -> bool {
    *WIPE_OLD_TOKENS.read().await
}
