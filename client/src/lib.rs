mod auth;
pub mod communication;
mod send;
mod state;

pub use auth::{AuthCredentials, AuthError, AuthResult, reauthenticate, reauthenticate_s};
pub use state::get_server_address;
pub use state::set_server_address;

pub use h_mail_interface::error::HResult;
pub use h_mail_interface::interface;
pub use h_mail_interface::shared::solve_pow;
pub use h_mail_interface::shared::solve_pow_iter;
pub use h_mail_interface::AnyhowError;
pub use h_mail_interface::anyhow as anyhow;
pub use h_mail_interface::shared::ROUGH_POW_ITER_PER_SECOND;
