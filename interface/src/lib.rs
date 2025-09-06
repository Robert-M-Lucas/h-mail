#[cfg(feature = "client_implementation")]
pub mod config;
#[cfg(feature = "client_implementation")]
pub mod error;
pub mod interface;
#[cfg(feature = "client_implementation")]
pub mod reexports;
#[cfg(feature = "client_implementation")]
pub mod server_config;
#[cfg(feature = "client_implementation")]
pub mod utility;

pub const BREAKING_INTERFACE_VERSION: &str = "1.0.0";