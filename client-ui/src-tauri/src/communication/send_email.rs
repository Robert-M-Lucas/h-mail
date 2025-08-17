use std::collections::HashMap;
use tracing::debug;
use h_mail_client::interface::fields::system_time::SystemTimeField;
use h_mail_client::interface::routes::native::get_hmails::GetHmailsRequest;
use h_mail_client::{AuthError, AuthResult};
use h_mail_client::communication::{check_is_whitelisted, get_pow_token, get_user_pow_policy};
use h_mail_client::interface::hmail::{HmailUser, SendHmailPackage};
use h_mail_client::interface::pow::PowHash;
use h_mail_client::interface::routes::native::get_user_pow_policy::{GetUserPowPolicyRequest, GetUserPowPolicyResponseAuthed};
use h_mail_client::interface::routes::native::is_whitelisted::{IsWhitelistedRequest, IsWhitelistedResponseAuthed};
use h_mail_client::interface::routes::native::send_hmail::{SendHmailRequest, SendHmailResponseAuthed, SolvedPowFor};
use h_mail_client::reexports::anyhow::bail;
use crate::communication::{InterfaceAuthResult, InterfaceResult};

#[tauri::command]
pub async fn get_pow_req(address: String) -> InterfaceResult<InterfaceAuthResult<GetUserPowPolicyResponseAuthed>> {
    debug!("get_pow_req");
    match get_user_pow_policy(&GetUserPowPolicyRequest::new(address))
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
pub async fn send_hmail(hmail: SendHmailPackage, bcc: Vec<HmailUser>) -> InterfaceResult<InterfaceAuthResult<SendHmailResponseAuthed>> {
    debug!("send_hmail");

    let hash = hmail.pow_hash();
    let mut solved_pows = Vec::new();

    for to_solve_for in hmail.to().iter().chain(hmail.ccs().iter()).chain(bcc.iter()) {
        let requirement = match get_user_pow_policy(&GetUserPowPolicyRequest::new(to_solve_for.hmail().clone()))
            .await
        {
            Ok(v) => {
                match v {
                    GetUserPowPolicyResponseAuthed::Whitelisted(_c) => {
                        solved_pows.push(SolvedPowFor::new(to_solve_for.hmail().clone(), None));
                        continue;
                    }
                    GetUserPowPolicyResponseAuthed::NotWhitelisted(p) => {
                        *p.minimum()
                    }
                    GetUserPowPolicyResponseAuthed::RequestFailed => return InterfaceResult::Err(format!("Request failed to {}", to_solve_for.hmail())),
                    GetUserPowPolicyResponseAuthed::BadRequest => return InterfaceResult::Err("Bad request".to_string())
                }
            },
            Err(e) => return match e {
                AuthError::RequireReauth => InterfaceResult::Ok(InterfaceAuthResult::Unauthorized),
                AuthError::Other(e) => InterfaceResult::from_error(e),
            },
        };

        todo!()
        // let pow_token = match get_pow_token()
        //
        // solved_pows.push(SolvedPowFor::new(
        //     to_solve_for.hmail().clone(),
        //
        // ))
    }

    todo!()
}