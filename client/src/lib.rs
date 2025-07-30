mod auth;
pub mod communication;
mod send;
mod state;
mod util;

pub use auth::{AuthCredentials, AuthError, AuthResult, reauthenticate, reauthenticate_s};
pub use h_mail_interface::*;
pub use state::set_server_address;
