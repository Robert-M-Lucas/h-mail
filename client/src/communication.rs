use crate::auth::AuthResult;
use crate::send::{send_get, send_get_auth, send_post, send_post_auth};
use crate::state::get_server_address;
use anyhow::bail;
use h_mail_interface::error::HResult;
use h_mail_interface::interface::routes::auth::check_auth::{
    AUTH_CHECK_AUTH_PATH, CheckAuthRequest, CheckAuthResponseAuthed,
};
use h_mail_interface::interface::routes::check_pow::{
    CHECK_POW_PATH, CheckPowRequest, CheckPowResponse,
};
use h_mail_interface::interface::routes::foreign::get_pow_token::{
    FOREIGN_GET_POW_TOKEN_PATH, GetPowTokenRequest, GetPowTokenResponse,
};
use h_mail_interface::interface::routes::foreign::get_user_pow_policy::{
    FOREIGN_GET_USER_POW_POLICY_PATH, GetUserPowPolicyRequest, GetUserPowPolicyResponse,
};
use h_mail_interface::interface::routes::native::create_account::{
    CreateAccountRequest, CreateAccountResponse, NATIVE_CREATE_ACCOUNT_PATH,
};
use h_mail_interface::interface::routes::native::get_create_account_pow_policy::{
    GetCreateAccountPowPolicyRequest, GetCreateAccountPowPolicyResponse,
    NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_PATH,
};
use h_mail_interface::interface::routes::native::get_emails::{
    GetEmailsRequest, GetEmailsResponseAuthed, NATIVE_GET_EMAILS_PATH,
};
use h_mail_interface::interface::routes::native::send_email::{
    NATIVE_SEND_EMAIL_PATH, SendEmailRequest, SendEmailResponseAuthed,
};
use h_mail_interface::interface::routes::{CHECK_ALIVE_PATH, CHECK_ALIVE_RESPONSE};
use h_mail_interface::shared::get_url_for_path;

pub async fn check_alive_s<S: AsRef<str>>(server: S) -> HResult<()> {
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

pub async fn check_alive() -> HResult<()> {
    check_alive_s(get_server_address().await).await
}

pub async fn check_pow_s<S: AsRef<str>>(
    server: S,
    check_pow_request: &CheckPowRequest,
) -> HResult<CheckPowResponse> {
    send_post(server, CHECK_POW_PATH, check_pow_request).await
}

pub async fn check_pow(check_pow_request: &CheckPowRequest) -> HResult<CheckPowResponse> {
    check_pow_s(get_server_address().await, check_pow_request).await
}

pub async fn get_pow_token<S: AsRef<str>>(
    server: S,
    get_pow_token_request: &GetPowTokenRequest,
) -> HResult<GetPowTokenResponse> {
    send_get(server, FOREIGN_GET_POW_TOKEN_PATH, get_pow_token_request).await
}

pub async fn get_user_pow_policy<S: AsRef<str>>(
    server: S,
    get_user_pow_policy_request: &GetUserPowPolicyRequest,
) -> HResult<GetUserPowPolicyResponse> {
    send_get(
        server,
        FOREIGN_GET_USER_POW_POLICY_PATH,
        get_user_pow_policy_request,
    )
    .await
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

pub async fn create_account_s<S: AsRef<str>>(
    server: S,
    create_account_request: &CreateAccountRequest,
) -> HResult<CreateAccountResponse> {
    send_post(server, NATIVE_CREATE_ACCOUNT_PATH, create_account_request).await
}

pub async fn create_account(
    create_account_request: &CreateAccountRequest,
) -> HResult<CreateAccountResponse> {
    create_account_s(get_server_address().await, create_account_request).await
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

pub async fn send_email_s<S: AsRef<str>>(
    server: S,
    send_email_request: &SendEmailRequest,
) -> AuthResult<SendEmailResponseAuthed> {
    send_post_auth(server, NATIVE_SEND_EMAIL_PATH, send_email_request).await
}

pub async fn send_email(
    send_email_request: &SendEmailRequest,
) -> AuthResult<SendEmailResponseAuthed> {
    send_email_s(get_server_address().await, send_email_request).await
}

pub async fn check_auth_s<S: AsRef<str>>(server: S) -> AuthResult<CheckAuthResponseAuthed> {
    send_get_auth::<_, CheckAuthResponseAuthed, _, _>(
        server,
        AUTH_CHECK_AUTH_PATH,
        &CheckAuthRequest::new(),
    )
    .await
}

pub async fn check_auth() -> AuthResult<CheckAuthResponseAuthed> {
    check_auth_s(get_server_address().await).await
}
