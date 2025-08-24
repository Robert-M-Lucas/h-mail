use h_mail_client::reexports::AnyhowError;
use h_mail_client::{AuthError, AuthResult, HResult};
use serde::Serialize;

pub mod auth;
pub mod create_account;
pub mod get_hmail_by_hash;
pub mod get_hmails;
pub mod get_foreign_pow_policy;
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
    pub fn todo() -> InterfaceResult<T> {
        InterfaceResult::Err("todo".to_string())
    }

    pub fn from_error(e: AnyhowError) -> Self {
        InterfaceResult::Err(format!("{e}"))
    }
}

impl<T> From<AuthError> for InterfaceResult<InterfaceAuthResult<T>> {
    fn from(value: AuthError) -> Self {
        match value {
            AuthError::RequireReauth => InterfaceResult::Ok(InterfaceAuthResult::Unauthorized),
            AuthError::Other(e) => Self::from_error(e),
        }
    }
}

impl<T> From<AuthResult<T>> for InterfaceResult<InterfaceAuthResult<T>> {
    fn from(value: AuthResult<T>) -> Self {
        match value {
            Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(v)),
            Err(e) => e.into(),
        }
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

impl<T> From<AnyhowError> for InterfaceResult<T> {
    fn from(value: AnyhowError) -> Self {
        Self::from_error(value)
    }
}
