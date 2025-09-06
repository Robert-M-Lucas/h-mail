#[cfg(feature = "client_interface")]
pub mod auth;
#[cfg(feature = "client_interface")]
pub mod check_pow;
pub mod foreign;
#[cfg(feature = "client_interface")]
pub mod get_pow_token;
#[cfg(feature = "client_interface")]
pub mod native;

#[cfg(feature = "client_interface")]
pub const CHECK_ALIVE_PATH: &str = "/";
#[cfg(feature = "client_interface")]
pub const CHECK_ALIVE_RESPONSE: &str = "<h1>Hello, world!</h1>";
#[cfg(feature = "client_interface")]
pub const GET_BREAKING_VERSION_PATH: &str = "/version";