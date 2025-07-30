use crate::auth::AuthResult;
use crate::send::{send_get, send_get_auth};
use crate::state::get_url_for_path;
use h_mail_interface::error::HResult;
use h_mail_interface::interface::routes::native::get_create_account_pow_policy::{GetCreateAccountPowPolicyRequest, GetCreateAccountPowPolicyResponse, NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_PATH};
use h_mail_interface::interface::routes::native::get_emails::{
    GetEmailsRequest, GetEmailsResponseAuthed, NATIVE_GET_EMAILS_PATH,
};
use std::borrow::Borrow;
use anyhow::bail;
use h_mail_interface::interface::routes::{CHECK_ALIVE_PATH, CHECK_ALIVE_RESPONSE};

pub async fn ping_server() -> HResult<()> {
    let r = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get(get_url_for_path(CHECK_ALIVE_PATH).await).send().await?.text().await?;

    if r != CHECK_ALIVE_RESPONSE {
        bail!("Server did not respond to alive check with expected response.");
    }

    Ok(())
}

pub async fn get_create_account_pow_policy() -> HResult<GetCreateAccountPowPolicyResponse> {
    send_get(get_url_for_path(NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_PATH).await, &GetCreateAccountPowPolicyRequest::new()).await
}

pub async fn get_emails(get_emails_request: &GetEmailsRequest) -> AuthResult<GetEmailsResponseAuthed> {
    send_get_auth(
        get_url_for_path(NATIVE_GET_EMAILS_PATH).await,
        get_emails_request,
    )
    .await
}
