use std::collections::HashMap;
use tracing::debug;
use h_mail_client::interface::fields::system_time::SystemTimeField;
use h_mail_client::interface::routes::native::get_emails::GetEmailsRequest;
use h_mail_client::{ms_since_epoch_to_system_time, solve_pow, AuthError, AuthResult};
use h_mail_client::communication::{check_is_whitelisted, get_pow_token};
use h_mail_client::interface::email::{EmailUser, SendEmailPackage};
use h_mail_client::interface::pow::PowHash;
use h_mail_client::interface::routes::native::is_whitelisted::{IsWhitelistedRequest, IsWhitelistedResponseAuthed};
use h_mail_client::interface::routes::native::send_email::{SendEmailRequest, SendEmailResponseAuthed, SolvedPowFor};
use h_mail_client::reexports::anyhow::bail;
use crate::communication::{InterfaceAuthResult, InterfaceResult};

#[tauri::command]
pub async fn get_pow_req(address: String) -> InterfaceResult<InterfaceAuthResult<IsWhitelistedResponseAuthed>> {
    debug!("get_pow_req");
    match check_is_whitelisted(&IsWhitelistedRequest::new(address))
        .await
    {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(v)),
        Err(e) => match e {
            AuthError::RequireReauth => InterfaceResult::Ok(InterfaceAuthResult::Unauthorized),
            AuthError::Other(e) => InterfaceResult::from_error(e),
        },
    }
}

#[tauri::command]
pub async fn send_email(email: SendEmailPackage, bcc: Vec<EmailUser>) -> InterfaceResult<InterfaceAuthResult<SendEmailResponseAuthed>> {
    debug!("send_email");

    let hash = email.pow_hash();
    let mut solved_pows = Vec::new();

    for to_solve_for in email.to().iter().chain(email.cc().iter()).chain(bcc.iter()) {
        let requirement = match check_is_whitelisted(&IsWhitelistedRequest::new(to_solve_for.email().clone()))
            .await
        {
            Ok(v) => {
                match v {
                    IsWhitelistedResponseAuthed::Whitelisted(_c) => {
                        solved_pows.push(SolvedPowFor::new(to_solve_for.email().clone(), None));
                        continue;
                    }
                    IsWhitelistedResponseAuthed::NotWhitelisted(p) => {
                        *p.minimum()
                    }
                    IsWhitelistedResponseAuthed::RequestFailed => return InterfaceResult::Err(format!("Request failed to {}", to_solve_for.email())),
                    IsWhitelistedResponseAuthed::BadRequest => return InterfaceResult::Err("Bad request".to_string())
                }
            },
            Err(e) => return match e {
                AuthError::RequireReauth => InterfaceResult::Ok(InterfaceAuthResult::Unauthorized),
                AuthError::Other(e) => InterfaceResult::from_error(e),
            },
        };

        let pow_token = match get_pow_token()

        solved_pows.push(SolvedPowFor::new(
            to_solve_for.email().clone(),

        ))
    }

    todo!()
}