use crate::gen_schemas;
use h_mail_interface::interface::email::Email;
use h_mail_interface::interface::email::EmailPackage;
use h_mail_interface::interface::fields::auth_token::AuthTokenDataField;
use h_mail_interface::interface::fields::auth_token::AuthTokenField;
use h_mail_interface::interface::fields::big_uint::BigUintField;
use h_mail_interface::interface::fields::system_time::SystemTimeField;
use h_mail_interface::interface::pow::PowClassification;
use h_mail_interface::interface::pow::PowFailureReason;
use h_mail_interface::interface::pow::PowPolicy;
use h_mail_interface::interface::routes::auth::authenticate::AUTH_AUTHENTICATE_METHOD;
use h_mail_interface::interface::routes::auth::authenticate::AUTH_AUTHENTICATE_PATH;
use h_mail_interface::interface::routes::auth::authenticate::AUTH_AUTHENTICATE_REQUIRES_AUTH;
use h_mail_interface::interface::routes::auth::authenticate::AuthenticateRequest;
use h_mail_interface::interface::routes::auth::authenticate::AuthenticateResponse;
use h_mail_interface::interface::routes::auth::check_auth::AUTH_CHECK_AUTH_METHOD;
use h_mail_interface::interface::routes::auth::check_auth::AUTH_CHECK_AUTH_PATH;
use h_mail_interface::interface::routes::auth::check_auth::AUTH_CHECK_AUTH_REQUIRES_AUTH;
use h_mail_interface::interface::routes::auth::check_auth::CheckAuthRequest;
use h_mail_interface::interface::routes::auth::check_auth::CheckAuthResponse;
use h_mail_interface::interface::routes::auth::check_auth::CheckAuthResponseAuthed;
use h_mail_interface::interface::routes::auth::refresh_access::AUTH_REFRESH_ACCESS_METHOD;
use h_mail_interface::interface::routes::auth::refresh_access::AUTH_REFRESH_ACCESS_PATH;
use h_mail_interface::interface::routes::auth::refresh_access::AUTH_REFRESH_ACCESS_REQUIRES_AUTH;
use h_mail_interface::interface::routes::auth::refresh_access::RefreshAccessRequest;
use h_mail_interface::interface::routes::auth::refresh_access::RefreshAccessResponse;
use h_mail_interface::interface::routes::check_pow::CHECK_POW_METHOD;
use h_mail_interface::interface::routes::check_pow::CHECK_POW_PATH;
use h_mail_interface::interface::routes::check_pow::CHECK_POW_REQUIRES_AUTH;
use h_mail_interface::interface::routes::check_pow::CheckPowPackage;
use h_mail_interface::interface::routes::check_pow::CheckPowRequest;
use h_mail_interface::interface::routes::check_pow::CheckPowResponse;
use h_mail_interface::interface::routes::foreign::deliver_email::DeliverEmailRequest;
use h_mail_interface::interface::routes::foreign::deliver_email::DeliverEmailResponse;
use h_mail_interface::interface::routes::foreign::deliver_email::FOREIGN_DELIVER_EMAIL_METHOD;
use h_mail_interface::interface::routes::foreign::deliver_email::FOREIGN_DELIVER_EMAIL_PATH;
use h_mail_interface::interface::routes::foreign::deliver_email::FOREIGN_DELIVER_EMAIL_REQUIRES_AUTH;
use h_mail_interface::interface::routes::foreign::get_user_pow_policy::FOREIGN_GET_USER_POW_POLICY_METHOD;
use h_mail_interface::interface::routes::foreign::get_user_pow_policy::FOREIGN_GET_USER_POW_POLICY_PATH;
use h_mail_interface::interface::routes::foreign::get_user_pow_policy::FOREIGN_GET_USER_POW_POLICY_REQUIRES_AUTH;
use h_mail_interface::interface::routes::foreign::get_user_pow_policy::GetUserPowPolicyRequest;
use h_mail_interface::interface::routes::foreign::get_user_pow_policy::GetUserPowPolicyResponse;
use h_mail_interface::interface::routes::foreign::verify_ip::FOREIGN_VERIFY_IP_METHOD;
use h_mail_interface::interface::routes::foreign::verify_ip::FOREIGN_VERIFY_IP_PATH;
use h_mail_interface::interface::routes::foreign::verify_ip::FOREIGN_VERIFY_IP_REQUIRES_AUTH;
use h_mail_interface::interface::routes::foreign::verify_ip::VerifyIpRequest;
use h_mail_interface::interface::routes::foreign::verify_ip::VerifyIpResponse;
use h_mail_interface::interface::routes::get_pow_token::GET_POW_TOKEN_METHOD;
use h_mail_interface::interface::routes::get_pow_token::GET_POW_TOKEN_PATH;
use h_mail_interface::interface::routes::get_pow_token::GET_POW_TOKEN_REQUIRES_AUTH;
use h_mail_interface::interface::routes::get_pow_token::GetPowTokenRequest;
use h_mail_interface::interface::routes::get_pow_token::GetPowTokenResponse;
use h_mail_interface::interface::routes::native::create_account::CreateAccountPackage;
use h_mail_interface::interface::routes::native::create_account::CreateAccountRequest;
use h_mail_interface::interface::routes::native::create_account::CreateAccountResponse;
use h_mail_interface::interface::routes::native::create_account::NATIVE_CREATE_ACCOUNT_METHOD;
use h_mail_interface::interface::routes::native::create_account::NATIVE_CREATE_ACCOUNT_PATH;
use h_mail_interface::interface::routes::native::create_account::NATIVE_CREATE_ACCOUNT_REQUIRES_AUTH;
use h_mail_interface::interface::routes::native::get_create_account_pow_policy::GetCreateAccountPowPolicyRequest;
use h_mail_interface::interface::routes::native::get_create_account_pow_policy::GetCreateAccountPowPolicyResponse;
use h_mail_interface::interface::routes::native::get_create_account_pow_policy::NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_METHOD;
use h_mail_interface::interface::routes::native::get_create_account_pow_policy::NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_PATH;
use h_mail_interface::interface::routes::native::get_create_account_pow_policy::NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_REQUIRES_AUTH;
use h_mail_interface::interface::routes::native::get_emails::GetEmailsEmail;
use h_mail_interface::interface::routes::native::get_emails::GetEmailsRequest;
use h_mail_interface::interface::routes::native::get_emails::GetEmailsResponse;
use h_mail_interface::interface::routes::native::get_emails::GetEmailsResponseAuthed;
use h_mail_interface::interface::routes::native::get_emails::NATIVE_GET_EMAILS_METHOD;
use h_mail_interface::interface::routes::native::get_emails::NATIVE_GET_EMAILS_PATH;
use h_mail_interface::interface::routes::native::get_emails::NATIVE_GET_EMAILS_REQUIRES_AUTH;
use h_mail_interface::interface::routes::native::send_email::NATIVE_SEND_EMAIL_METHOD;
use h_mail_interface::interface::routes::native::send_email::NATIVE_SEND_EMAIL_PATH;
use h_mail_interface::interface::routes::native::send_email::NATIVE_SEND_EMAIL_REQUIRES_AUTH;
use h_mail_interface::interface::routes::native::send_email::SendEmailRequest;
use h_mail_interface::interface::routes::native::send_email::SendEmailResponse;
use h_mail_interface::interface::routes::native::send_email::SendEmailResponseAuthed;
use h_mail_interface::shared::RequestMethod;
use schemars::Schema;

pub fn all() -> Vec<(
    Schema,
    &'static str,
    Option<&'static str>,
    Option<(&'static str, RequestMethod, bool)>,
)> {
    gen_schemas![
        (PowFailureReason, Some("pow"), None),
        (PowPolicy, Some("pow"), None),
        (PowClassification, Some("pow"), None),
        (BigUintField, Some("fields/big_uint"), None),
        (SystemTimeField, Some("fields/system_time"), None),
        (AuthTokenField, Some("fields/auth_token"), None),
        (AuthTokenDataField, Some("fields/auth_token"), None),
        (EmailPackage, Some("email"), None),
        (Email, Some("email"), None),
        (CheckPowPackage, Some("routes/check_pow"), None),
        (CheckPowResponse, Some("routes/check_pow"), None),
        (
            CheckPowRequest,
            Some("routes/check_pow"),
            Some((CHECK_POW_PATH, CHECK_POW_METHOD, CHECK_POW_REQUIRES_AUTH))
        ),
        (
            GetUserPowPolicyRequest,
            Some("routes/foreign/get_user_pow_policy"),
            Some((
                FOREIGN_GET_USER_POW_POLICY_PATH,
                FOREIGN_GET_USER_POW_POLICY_METHOD,
                FOREIGN_GET_USER_POW_POLICY_REQUIRES_AUTH
            ))
        ),
        (
            GetUserPowPolicyResponse,
            Some("routes/foreign/get_user_pow_policy"),
            None
        ),
        (
            VerifyIpRequest,
            Some("routes/foreign/verify_ip"),
            Some((
                FOREIGN_VERIFY_IP_PATH,
                FOREIGN_VERIFY_IP_METHOD,
                FOREIGN_VERIFY_IP_REQUIRES_AUTH
            ))
        ),
        (VerifyIpResponse, Some("routes/foreign/verify_ip"), None),
        (
            DeliverEmailRequest,
            Some("routes/foreign/deliver_email"),
            Some((
                FOREIGN_DELIVER_EMAIL_PATH,
                FOREIGN_DELIVER_EMAIL_METHOD,
                FOREIGN_DELIVER_EMAIL_REQUIRES_AUTH
            ))
        ),
        (
            DeliverEmailResponse,
            Some("routes/foreign/deliver_email"),
            None
        ),
        (
            GetPowTokenRequest,
            Some("routes/get_pow_token"),
            Some((
                GET_POW_TOKEN_PATH,
                GET_POW_TOKEN_METHOD,
                GET_POW_TOKEN_REQUIRES_AUTH
            ))
        ),
        (GetPowTokenResponse, Some("routes/get_pow_token"), None),
        (
            CreateAccountPackage,
            Some("routes/native/create_account"),
            None
        ),
        (
            CreateAccountResponse,
            Some("routes/native/create_account"),
            None
        ),
        (
            CreateAccountRequest,
            Some("routes/native/create_account"),
            Some((
                NATIVE_CREATE_ACCOUNT_PATH,
                NATIVE_CREATE_ACCOUNT_METHOD,
                NATIVE_CREATE_ACCOUNT_REQUIRES_AUTH
            ))
        ),
        (
            SendEmailRequest,
            Some("routes/native/send_email"),
            Some((
                NATIVE_SEND_EMAIL_PATH,
                NATIVE_SEND_EMAIL_METHOD,
                NATIVE_SEND_EMAIL_REQUIRES_AUTH
            ))
        ),
        (
            SendEmailResponseAuthed,
            Some("routes/native/send_email"),
            None
        ),
        (SendEmailResponse, Some("routes/native/send_email"), None),
        (
            GetCreateAccountPowPolicyRequest,
            Some("routes/native/get_create_account_pow_policy"),
            Some((
                NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_PATH,
                NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_METHOD,
                NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_REQUIRES_AUTH
            ))
        ),
        (
            GetCreateAccountPowPolicyResponse,
            Some("routes/native/get_create_account_pow_policy"),
            None
        ),
        (
            GetEmailsRequest,
            Some("routes/native/get_emails"),
            Some((
                NATIVE_GET_EMAILS_PATH,
                NATIVE_GET_EMAILS_METHOD,
                NATIVE_GET_EMAILS_REQUIRES_AUTH
            ))
        ),
        (GetEmailsEmail, Some("routes/native/get_emails"), None),
        (
            GetEmailsResponseAuthed,
            Some("routes/native/get_emails"),
            None
        ),
        (GetEmailsResponse, Some("routes/native/get_emails"), None),
        (
            AuthenticateRequest,
            Some("routes/auth/authenticate"),
            Some((
                AUTH_AUTHENTICATE_PATH,
                AUTH_AUTHENTICATE_METHOD,
                AUTH_AUTHENTICATE_REQUIRES_AUTH
            ))
        ),
        (AuthenticateResponse, Some("routes/auth/authenticate"), None),
        (
            RefreshAccessRequest,
            Some("routes/auth/refresh_access"),
            Some((
                AUTH_REFRESH_ACCESS_PATH,
                AUTH_REFRESH_ACCESS_METHOD,
                AUTH_REFRESH_ACCESS_REQUIRES_AUTH
            ))
        ),
        (
            RefreshAccessResponse,
            Some("routes/auth/refresh_access"),
            None
        ),
        (
            CheckAuthRequest,
            Some("routes/auth/check_auth"),
            Some((
                AUTH_CHECK_AUTH_PATH,
                AUTH_CHECK_AUTH_METHOD,
                AUTH_CHECK_AUTH_REQUIRES_AUTH
            ))
        ),
        (
            CheckAuthResponseAuthed,
            Some("routes/auth/check_auth"),
            None
        ),
        (CheckAuthResponse, Some("routes/auth/check_auth"), None)
    ]
}
