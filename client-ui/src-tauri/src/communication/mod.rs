use h_mail_client::reexports::AnyhowError;
use h_mail_client::HResult;
use serde::Serialize;

pub mod auth;
pub mod create_account;
pub mod get_hmails;
pub mod pow_requests;
pub mod send_hmail;
pub mod whitelist_management;

#[derive(Serialize)]
pub enum InterfaceAuthResult<T> {
    Unauthorized,
    Success(T),
}

#[derive(Serialize)]
pub enum InterfaceResult<T> {
    Ok(T),
    Err(String),
}

impl<T> InterfaceResult<T> {
    pub fn from_error(e: AnyhowError) -> Self {
        InterfaceResult::Err(format!("{e}"))
    }
}

impl<T> From<HResult<T>> for InterfaceResult<T> {
    fn from(value: HResult<T>) -> Self {
        match value {
            Ok(v) => InterfaceResult::Ok(v),
            Err(e) => Self::from_error(e),
        }
    }
}
