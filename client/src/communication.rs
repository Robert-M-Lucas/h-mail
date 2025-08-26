use crate::auth::AuthResult;
use crate::send::{send, send_auth};
use crate::state::get_server_address;
use h_mail_interface::error::HResult;
use h_mail_interface::interface::routes::auth::check_auth::{
    AUTH_CHECK_AUTH_METHOD, AUTH_CHECK_AUTH_PATH, CheckAuthRequest, CheckAuthResponseAuthed,
};
use h_mail_interface::interface::routes::check_pow::{
    CHECK_POW_METHOD, CHECK_POW_PATH, CheckPowRequest, CheckPowResponse,
};
use h_mail_interface::interface::routes::foreign::get_anonymous_user_pow_policy::{
    FOREIGN_GET_ANONYMOUS_USER_POW_POLICY_METHOD, FOREIGN_GET_ANONYMOUS_USER_POW_POLICY_PATH,
    GetAnonymousUserPowPolicyRequest, GetAnonymousUserPowPolicyResponse,
};
use h_mail_interface::interface::routes::get_pow_token::{
    GET_POW_TOKEN_METHOD, GET_POW_TOKEN_PATH, GetPowTokenRequest, GetPowTokenResponse,
};
use h_mail_interface::interface::routes::native::add_whitelist::{
    AddWhitelistRequest, AddWhitelistResponseAuthed, NATIVE_ADD_WHITELIST_METHOD,
    NATIVE_ADD_WHITELIST_PATH,
};
use h_mail_interface::interface::routes::native::create_account::{
    CreateAccountRequest, CreateAccountResponse, NATIVE_CREATE_ACCOUNT_METHOD,
    NATIVE_CREATE_ACCOUNT_PATH,
};
use h_mail_interface::interface::routes::native::get_create_account_pow_policy::{
    GetCreateAccountPowPolicyRequest, GetCreateAccountPowPolicyResponse,
    NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_METHOD, NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_PATH,
};
use h_mail_interface::interface::routes::native::get_foreign_pow_policy::{
    GetForeignPowPolicyRequest, GetForeignPowPolicyResponseAuthed,
    NATIVE_GET_FOREIGN_POW_POLICY_METHOD, NATIVE_GET_FOREIGN_POW_POLICY_PATH,
};
use h_mail_interface::interface::routes::native::get_hmail_by_hash::{
    GetHmailByHashRequest, GetHmailByHashResponseAuthed, NATIVE_GET_HMAIL_BY_HASH_METHOD,
    NATIVE_GET_HMAIL_BY_HASH_PATH,
};
use h_mail_interface::interface::routes::native::get_hmails::{
    GetHmailsRequest, GetHmailsResponseAuthed, NATIVE_GET_HMAILS_METHOD, NATIVE_GET_HMAILS_PATH,
};
use h_mail_interface::interface::routes::native::get_pow_policy::{
    GetPowPolicyRequest, GetPowPolicyResponseAuthed, NATIVE_GET_POW_POLICY_METHOD,
    NATIVE_GET_POW_POLICY_PATH,
};
use h_mail_interface::interface::routes::native::get_whitelist::{
    GetWhitelistRequest, GetWhitelistResponseAuthed, NATIVE_GET_WHITELIST_METHOD,
    NATIVE_GET_WHITELIST_PATH,
};
use h_mail_interface::interface::routes::native::remove_whitelist::{
    NATIVE_REMOVE_WHITELIST_METHOD, NATIVE_REMOVE_WHITELIST_PATH, RemoveWhitelistRequest,
    RemoveWhitelistResponseAuthed,
};
use h_mail_interface::interface::routes::native::send_hmail::{
    NATIVE_SEND_HMAIL_METHOD, NATIVE_SEND_HMAIL_PATH, SendHmailRequest, SendHmailResponseAuthed,
};
use h_mail_interface::interface::routes::native::set_pow_policy::{
    NATIVE_SET_POW_POLICY_METHOD, NATIVE_SET_POW_POLICY_PATH, SetPowPolicyRequest,
    SetPowPolicyResponseAuthed,
};
use h_mail_interface::interface::routes::{CHECK_ALIVE_PATH, CHECK_ALIVE_RESPONSE};
use h_mail_interface::reexports::anyhow::bail;
use h_mail_interface::utility::get_url_for_path;

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
    check_alive_s(get_server_address().await?).await
}

pub async fn check_pow_s<S: AsRef<str>>(
    server: S,
    check_pow_request: &CheckPowRequest,
) -> HResult<CheckPowResponse> {
    send(server, CHECK_POW_PATH, check_pow_request, CHECK_POW_METHOD).await
}

pub async fn check_pow(check_pow_request: &CheckPowRequest) -> HResult<CheckPowResponse> {
    check_pow_s(get_server_address().await?, check_pow_request).await
}

pub async fn get_pow_token<S: AsRef<str>>(server: S) -> HResult<GetPowTokenResponse> {
    send(
        server,
        GET_POW_TOKEN_PATH,
        &GetPowTokenRequest::new(),
        GET_POW_TOKEN_METHOD,
    )
    .await
}

pub async fn get_pow_token_our_server() -> HResult<GetPowTokenResponse> {
    get_pow_token(get_server_address().await?).await
}

pub async fn get_anonymous_user_pow_policy<S: AsRef<str>>(
    server: S,
    get_user_pow_policy_request: &GetAnonymousUserPowPolicyRequest,
) -> HResult<GetAnonymousUserPowPolicyResponse> {
    send(
        server,
        FOREIGN_GET_ANONYMOUS_USER_POW_POLICY_PATH,
        get_user_pow_policy_request,
        FOREIGN_GET_ANONYMOUS_USER_POW_POLICY_METHOD,
    )
    .await
}

pub async fn get_create_account_pow_policy_s<S: AsRef<str>>(
    server: S,
) -> HResult<GetCreateAccountPowPolicyResponse> {
    send(
        server,
        NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_PATH,
        &GetCreateAccountPowPolicyRequest::new(),
        NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_METHOD,
    )
    .await
}

pub async fn get_create_account_pow_policy() -> HResult<GetCreateAccountPowPolicyResponse> {
    get_create_account_pow_policy_s(get_server_address().await?).await
}

pub async fn create_account_s<S: AsRef<str>>(
    server: S,
    create_account_request: &CreateAccountRequest,
) -> HResult<CreateAccountResponse> {
    send(
        server,
        NATIVE_CREATE_ACCOUNT_PATH,
        create_account_request,
        NATIVE_CREATE_ACCOUNT_METHOD,
    )
    .await
}

pub async fn create_account(
    create_account_request: &CreateAccountRequest,
) -> HResult<CreateAccountResponse> {
    create_account_s(get_server_address().await?, create_account_request).await
}

pub async fn get_hmails_s<S: AsRef<str>>(
    server: S,
    get_hmails_request: &GetHmailsRequest,
) -> AuthResult<GetHmailsResponseAuthed> {
    send_auth::<_, GetHmailsResponseAuthed, _, _>(
        server,
        NATIVE_GET_HMAILS_PATH,
        get_hmails_request,
        NATIVE_GET_HMAILS_METHOD,
    )
    .await
}

pub async fn get_hmails(
    get_hmails_request: &GetHmailsRequest,
) -> AuthResult<GetHmailsResponseAuthed> {
    get_hmails_s(get_server_address().await?, get_hmails_request).await
}

pub async fn get_hmail_by_hash_s<S: AsRef<str>>(
    server: S,
    get_hmail_by_hash_request: &GetHmailByHashRequest,
) -> AuthResult<GetHmailByHashResponseAuthed> {
    send_auth::<_, GetHmailByHashResponseAuthed, _, _>(
        server,
        NATIVE_GET_HMAIL_BY_HASH_PATH,
        get_hmail_by_hash_request,
        NATIVE_GET_HMAIL_BY_HASH_METHOD,
    )
    .await
}

pub async fn get_hmail_by_hash(
    get_hmail_by_hash_request: &GetHmailByHashRequest,
) -> AuthResult<GetHmailByHashResponseAuthed> {
    get_hmail_by_hash_s(get_server_address().await?, get_hmail_by_hash_request).await
}

pub async fn send_hmail_s<S: AsRef<str>>(
    server: S,
    send_hmail_request: &SendHmailRequest,
) -> AuthResult<SendHmailResponseAuthed> {
    send_auth(
        server,
        NATIVE_SEND_HMAIL_PATH,
        send_hmail_request,
        NATIVE_SEND_HMAIL_METHOD,
    )
    .await
}

pub async fn send_hmail(
    send_hmail_request: &SendHmailRequest,
) -> AuthResult<SendHmailResponseAuthed> {
    send_hmail_s(get_server_address().await?, send_hmail_request).await
}

pub async fn check_auth_s<S: AsRef<str>>(server: S) -> AuthResult<CheckAuthResponseAuthed> {
    send_auth::<_, CheckAuthResponseAuthed, _, _>(
        server,
        AUTH_CHECK_AUTH_PATH,
        &CheckAuthRequest::new(),
        AUTH_CHECK_AUTH_METHOD,
    )
    .await
}

pub async fn check_auth() -> AuthResult<CheckAuthResponseAuthed> {
    check_auth_s(get_server_address().await?).await
}

pub async fn get_foreign_pow_policy_s<S: AsRef<str>>(
    server: S,
    is_whitelisted_request: &GetForeignPowPolicyRequest,
) -> AuthResult<GetForeignPowPolicyResponseAuthed> {
    send_auth::<_, GetForeignPowPolicyResponseAuthed, _, _>(
        server,
        NATIVE_GET_FOREIGN_POW_POLICY_PATH,
        is_whitelisted_request,
        NATIVE_GET_FOREIGN_POW_POLICY_METHOD,
    )
    .await
}

pub async fn get_foreign_pow_policy(
    is_whitelisted_request: &GetForeignPowPolicyRequest,
) -> AuthResult<GetForeignPowPolicyResponseAuthed> {
    get_foreign_pow_policy_s(get_server_address().await?, is_whitelisted_request).await
}

pub async fn add_whitelist_s<S: AsRef<str>>(
    server: S,
    add_whitelist_request: &AddWhitelistRequest,
) -> AuthResult<AddWhitelistResponseAuthed> {
    send_auth::<_, AddWhitelistResponseAuthed, _, _>(
        server,
        NATIVE_ADD_WHITELIST_PATH,
        add_whitelist_request,
        NATIVE_ADD_WHITELIST_METHOD,
    )
    .await
}

pub async fn add_whitelist(
    add_whitelist_request: &AddWhitelistRequest,
) -> AuthResult<AddWhitelistResponseAuthed> {
    add_whitelist_s(get_server_address().await?, add_whitelist_request).await
}

pub async fn remove_whitelist_s<S: AsRef<str>>(
    server: S,
    remove_whitelist_request: &RemoveWhitelistRequest,
) -> AuthResult<RemoveWhitelistResponseAuthed> {
    send_auth::<_, RemoveWhitelistResponseAuthed, _, _>(
        server,
        NATIVE_REMOVE_WHITELIST_PATH,
        remove_whitelist_request,
        NATIVE_REMOVE_WHITELIST_METHOD,
    )
    .await
}

pub async fn remove_whitelist(
    remove_whitelist_request: &RemoveWhitelistRequest,
) -> AuthResult<RemoveWhitelistResponseAuthed> {
    remove_whitelist_s(get_server_address().await?, remove_whitelist_request).await
}

pub async fn get_whitelist_s<S: AsRef<str>>(server: S) -> AuthResult<GetWhitelistResponseAuthed> {
    send_auth::<_, GetWhitelistResponseAuthed, _, _>(
        server,
        NATIVE_GET_WHITELIST_PATH,
        &GetWhitelistRequest::new(),
        NATIVE_GET_WHITELIST_METHOD,
    )
    .await
}

pub async fn get_whitelist() -> AuthResult<GetWhitelistResponseAuthed> {
    get_whitelist_s(get_server_address().await?).await
}

pub async fn get_pow_policy_s<S: AsRef<str>>(server: S) -> AuthResult<GetPowPolicyResponseAuthed> {
    send_auth::<_, GetPowPolicyResponseAuthed, _, _>(
        server,
        NATIVE_GET_POW_POLICY_PATH,
        &GetPowPolicyRequest::new(),
        NATIVE_GET_POW_POLICY_METHOD,
    )
    .await
}

pub async fn get_pow_policy() -> AuthResult<GetPowPolicyResponseAuthed> {
    get_pow_policy_s(get_server_address().await?).await
}

pub async fn set_pow_policy_s<S: AsRef<str>>(
    server: S,
    set_pow_policy_request: &SetPowPolicyRequest,
) -> AuthResult<SetPowPolicyResponseAuthed> {
    send_auth::<_, SetPowPolicyResponseAuthed, _, _>(
        server,
        NATIVE_SET_POW_POLICY_PATH,
        set_pow_policy_request,
        NATIVE_SET_POW_POLICY_METHOD,
    )
    .await
}

pub async fn set_pow_policy(
    set_pow_policy_request: &SetPowPolicyRequest,
) -> AuthResult<SetPowPolicyResponseAuthed> {
    set_pow_policy_s(get_server_address().await?, set_pow_policy_request).await
}
