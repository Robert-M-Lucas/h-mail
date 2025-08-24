use crate::communication::{InterfaceAuthResult, InterfaceResult};
use crate::pow_manager::queue_solve_pow_result;
use h_mail_client::communication::get_pow_token;
use h_mail_client::communication::{get_foreign_pow_policy, send_hmail as c_send_hmail};
use h_mail_client::interface::fields::hmail_address::HmailAddress;
use h_mail_client::interface::hmail::SendHmailPackage;
use h_mail_client::interface::pow::{PowClassification, PowHash};
use h_mail_client::interface::routes::native::get_foreign_pow_policy::{
    GetForeignPowPolicyRequest, GetForeignPowPolicyResponseAuthed,
};
use h_mail_client::interface::routes::native::send_hmail::{
    SendHmailRequest, SendHmailResponseAuthed, SolvedPowFor,
};
use h_mail_client::AuthError;
use tracing::debug;

#[tauri::command]
pub async fn get_pow_req(
    address: String,
) -> InterfaceResult<InterfaceAuthResult<GetForeignPowPolicyResponseAuthed>> {
    debug!("get_pow_req");
    let Ok(address) = HmailAddress::new(&address) else {
        return InterfaceResult::Err("Invalid address".to_string());
    };
    match get_foreign_pow_policy(&GetForeignPowPolicyRequest::new(address)).await {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(v)),
        Err(e) => match e {
            AuthError::RequireReauth => InterfaceResult::Ok(InterfaceAuthResult::Unauthorized),
            AuthError::Other(e) => InterfaceResult::from_error(e),
        },
    }
}

#[tauri::command]
pub async fn send_hmail(
    hmail: SendHmailPackage,
    bccs: Vec<HmailAddress>,
    classifications: Vec<(HmailAddress, PowClassification)>,
) -> InterfaceResult<InterfaceAuthResult<SendHmailResponseAuthed>> {
    debug!("send_hmail");

    let hash = hmail.pow_hash();
    let mut solved_pows = Vec::new();

    for to_solve_for in hmail
        .recipients()
        .iter()
        .map(|r| r.address())
        .chain(hmail.ccs().iter().map(|c| c.address()))
        .chain(bccs.iter())
    {
        let target_classification = classifications
            .iter()
            .find(|(a, _c)| a == to_solve_for)
            .unwrap()
            .1;
        let requirement =
            match get_foreign_pow_policy(&GetForeignPowPolicyRequest::new(to_solve_for.clone()))
                .await
            {
                Ok(v) => match v {
                    GetForeignPowPolicyResponseAuthed::Whitelisted(r) => {
                        if *r.classification() == target_classification {
                            solved_pows.push(SolvedPowFor::new(to_solve_for.clone(), None));
                            continue;
                        } else {
                            r.policy().iters_from_classification(target_classification)
                        }
                    }
                    GetForeignPowPolicyResponseAuthed::NotWhitelisted(p) => {
                        p.iters_from_classification(target_classification)
                    }
                    GetForeignPowPolicyResponseAuthed::RequestFailed => {
                        return InterfaceResult::Err(format!("Request failed to {}", to_solve_for))
                    }
                    GetForeignPowPolicyResponseAuthed::BadRequest => {
                        return InterfaceResult::Err("Bad request".to_string())
                    }
                    GetForeignPowPolicyResponseAuthed::UserDoesNotExist => {
                        return InterfaceResult::Err(format!("User {to_solve_for} does not exist"))
                    }
                },
                Err(e) => return e.into(),
            };

        let pow_token_response = match get_pow_token(to_solve_for.domain()).await {
            Ok(pt) => match pt.decode() {
                Ok(pt) => pt,
                Err(_) => {
                    return InterfaceResult::Err(format!(
                        "Failed to decode response from {}",
                        to_solve_for
                    ))
                }
            },
            Err(_e) => return InterfaceResult::Err(format!("Request failed to {}", to_solve_for)),
        };

        let Some(solved) =
            queue_solve_pow_result(pow_token_response.token(), requirement, &hash).await
        else {
            return InterfaceResult::Err("Proof-of-work cancelled".to_string());
        };

        solved_pows.push(SolvedPowFor::new(to_solve_for.clone(), Some(solved)))
    }

    c_send_hmail(&SendHmailRequest::new(hmail, bccs, solved_pows))
        .await
        .into()
}
