pub mod auth;
pub mod check_pow;
pub mod foreign;
pub mod native;

pub const CHECK_ALIVE_PATH: &str = "/";
pub const CHECK_ALIVE_RESPONSE: &str = "<h1>Hello, world!</h1>";
