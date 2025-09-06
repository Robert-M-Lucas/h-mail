mod auth;
pub mod communication;
mod send;
mod state;

pub use auth::{AuthCredentials, AuthError, AuthResult, logout, reauthenticate, reauthenticate_s};
pub use state::dont_wipe_old_tokens;
pub use state::get_server_address;
pub use state::set_server_address;
pub use state::get_data_location;
pub use state::set_data_location;

pub use h_mail_interface::config::ROUGH_POW_ITER_PER_SECOND;
pub use h_mail_interface::error::HResult;
pub use h_mail_interface::interface;
pub use h_mail_interface::reexports;
pub use h_mail_interface::utility::*;
