use crate::communication::InterfaceResult;
use crate::pow_manager::{queue_solve_pow, PowSolveRequest};
use h_mail_client::communication::create_account as c_create_account;
use h_mail_client::communication::{get_create_account_pow_policy, get_pow_token_our_server};
use h_mail_client::interface::fields::big_uint::BigUintField;
use h_mail_client::interface::pow::{PowHash, PowIters, PowResult};
use h_mail_client::interface::routes::native::create_account::{
    CreateAccountPackage, CreateAccountRequest, CreateAccountResponse,
};
use h_mail_client::reexports::anyhow::bail;
use h_mail_client::{reauthenticate, AuthCredentials, HResult};
use tracing::debug;

async fn create_account_inner(username: String, password: String) -> HResult<String> {
    let create_account_request = CreateAccountPackage::new(username.clone(), password.clone());
    let pow_token = get_pow_token_our_server().await?;
    let (pow_token, _) = pow_token.decode()?.dissolve();
    let pow_policy = get_create_account_pow_policy().await?;
    let iters = *pow_policy.requirement();
    let Some(pow_result) = queue_solve_pow(PowSolveRequest::new(
        pow_token.clone(),
        *pow_policy.requirement(),
        create_account_request.pow_hash(),
    ))
    .await else {
        bail!("Proof-of-work cancelled")
    };

    let cr = c_create_account(&CreateAccountRequest::new(
        create_account_request,
        Some(PowResult::new(
            iters,
            BigUintField::new(&pow_token),
            BigUintField::new(&pow_result),
        )),
    ))
    .await?;
    match cr {
        CreateAccountResponse::Success => {}
        CreateAccountResponse::BadUsername => {
            bail!("Bad username");
        }
        CreateAccountResponse::UsernameInUse => {
            bail!("Username in use");
        }
        CreateAccountResponse::BadPassword => {
            bail!("Bad password");
        }
        CreateAccountResponse::DoesNotMeetPolicy(_) => {
            bail!("Doesn't meet policy");
        }
        CreateAccountResponse::PowFailure(_) => {
            bail!("Pow failure");
        }
    };
    reauthenticate(AuthCredentials::new(username.clone(), password)).await?;
    Ok(username)
}

#[tauri::command]
pub async fn create_account(username: String, password: String) -> InterfaceResult<String> {
    debug!("create_account");
    create_account_inner(username, password).await.into()
}

#[tauri::command]
pub async fn create_account_requirement() -> InterfaceResult<PowIters> {
    debug!("create_account_requirement");
    get_create_account_pow_policy().await.map(|r| *r.requirement()).into()
}
