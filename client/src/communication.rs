use crate::auth::AuthResult;
use crate::send::{send_get, send_get_auth};
use crate::state::get_server_address;
use anyhow::bail;
use h_mail_interface::error::HResult;
use h_mail_interface::interface::routes::foreign::get_user_pow_policy::{
    FOREIGN_GET_USER_POW_POLICY_PATH, GetUserPowPolicyRequest, GetUserPowPolicyResponse,
};
use h_mail_interface::interface::routes::native::get_create_account_pow_policy::{
    GetCreateAccountPowPolicyRequest, GetCreateAccountPowPolicyResponse,
    NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_PATH,
};
use h_mail_interface::interface::routes::native::get_emails::{
    GetEmailsRequest, GetEmailsResponseAuthed, NATIVE_GET_EMAILS_PATH,
};
use h_mail_interface::interface::routes::{CHECK_ALIVE_PATH, CHECK_ALIVE_RESPONSE};
use h_mail_interface::interface::routes::foreign::get_pow_token::{GetPowTokenRequest, GetPowTokenResponse, FOREIGN_GET_POW_TOKEN_PATH};
use h_mail_interface::shared::get_url_for_path;

pub async fn ping_server_s<S: AsRef<str>>(server: S) -> HResult<()> {
    let r = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get(get_url_for_path(server, CHECK_ALIVE_PATH))
        .send()
        .await?
        .text()
        .await?;

    if r != CHECK_ALIVE_RESPONSE {
        bail!("Server did not respond to alive check with expected response.");
    }

    Ok(())
}

pub async fn ping_server() -> HResult<()> {
    ping_server_s(get_server_address().await).await
}

pub async fn get_create_account_pow_policy_s<S: AsRef<str>>(
    server: S,
) -> HResult<GetCreateAccountPowPolicyResponse> {
    send_get(
        server,
        NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_PATH,
        &GetCreateAccountPowPolicyRequest::new(),
    )
    .await
}

pub async fn get_create_account_pow_policy() -> HResult<GetCreateAccountPowPolicyResponse> {
    get_create_account_pow_policy_s(get_server_address().await).await
}

pub async fn get_user_pow_policy<S: AsRef<str>>(
    server: S,
    get_user_pow_policy: &GetUserPowPolicyRequest,
) -> HResult<GetUserPowPolicyResponse> {
    send_get(server, FOREIGN_GET_USER_POW_POLICY_PATH, get_user_pow_policy).await
}

pub async fn get_pow_token<S: AsRef<str>>(
    server: S,
    get_pow_token: &GetPowTokenRequest,
) -> HResult<GetPowTokenResponse> {
    send_get(server, FOREIGN_GET_POW_TOKEN_PATH, get_pow_token).await
}

pub async fn get_emails_s<S: AsRef<str>>(
    server: S,
    get_emails_request: &GetEmailsRequest,
) -> AuthResult<GetEmailsResponseAuthed> {
    send_get_auth(server, NATIVE_GET_EMAILS_PATH, get_emails_request).await
}

pub async fn get_emails(
    get_emails_request: &GetEmailsRequest,
) -> AuthResult<GetEmailsResponseAuthed> {
    get_emails_s(get_server_address().await, get_emails_request).await
}
