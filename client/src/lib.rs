mod auth;
pub mod communication;
mod send;
mod state;

use keyring::Entry;
pub use auth::{AuthCredentials, AuthError, AuthResult, reauthenticate, reauthenticate_s};
pub use state::set_server_address;

pub use h_mail_interface::error::HResult;
pub use h_mail_interface::interface;
pub use h_mail_interface::shared::solve_pow;

// pub fn test() {
//     let service = "my-app";
//     let username = "user";
//     let token = "my-secret-token";
// 
//     let entry = Entry::new(service, username).unwrap();
// 
//     // Store the token
//     entry.set_password(token).unwrap();
// 
//     // Retrieve the token
//     let stored_token = entry.get_password().unwrap();
//     println!("Stored token: {}", stored_token);
// 
//     // Optionally delete it
//     // entry.delete_password()?;
// 
// }