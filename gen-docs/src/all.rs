use crate::gen_schemas;
use schemars::Schema;
use h_mail_interface::interface::pow::PowFailureReason;
use h_mail_interface::interface::pow::PowPolicy;
use h_mail_interface::interface::pow::PowClassification;
use h_mail_interface::interface::fields::big_uint::BigUintField;
use h_mail_interface::interface::fields::system_time::SystemTimeField;
use h_mail_interface::interface::fields::auth_token::AuthTokenField;
use h_mail_interface::interface::fields::auth_token::AuthTokenDataField;
use h_mail_interface::interface::email::EmailPackage;
use h_mail_interface::interface::email::Email;
use h_mail_interface::interface::routes::check_pow::CheckPowPackage;
use h_mail_interface::interface::routes::check_pow::CheckPowResponse;
use h_mail_interface::interface::routes::check_pow::CheckPowRequest;
use h_mail_interface::interface::routes::foreign::get_user_pow_policy::GetUserPowPolicyRequest;
use h_mail_interface::interface::routes::foreign::get_user_pow_policy::GetUserPowPolicyResponse;
use h_mail_interface::interface::routes::foreign::verify_ip::VerifyIpRequest;
use h_mail_interface::interface::routes::foreign::verify_ip::VerifyIpResponse;
use h_mail_interface::interface::routes::foreign::deliver_email::DeliverEmailRequest;
use h_mail_interface::interface::routes::foreign::deliver_email::DeliverEmailResponse;
use h_mail_interface::interface::routes::get_pow_token::GetPowTokenRequest;
use h_mail_interface::interface::routes::get_pow_token::GetPowTokenResponse;
use h_mail_interface::interface::routes::native::create_account::CreateAccountPackage;
use h_mail_interface::interface::routes::native::create_account::CreateAccountResponse;
use h_mail_interface::interface::routes::native::create_account::CreateAccountRequest;
use h_mail_interface::interface::routes::native::send_email::SendEmailRequest;
use h_mail_interface::interface::routes::native::send_email::SendEmailResponseAuthed;
use h_mail_interface::interface::routes::native::send_email::SendEmailResponse;
use h_mail_interface::interface::routes::native::get_create_account_pow_policy::GetCreateAccountPowPolicyRequest;
use h_mail_interface::interface::routes::native::get_create_account_pow_policy::GetCreateAccountPowPolicyResponse;
use h_mail_interface::interface::routes::native::get_emails::GetEmailsRequest;
use h_mail_interface::interface::routes::native::get_emails::GetEmailsEmail;
use h_mail_interface::interface::routes::native::get_emails::GetEmailsResponseAuthed;
use h_mail_interface::interface::routes::native::get_emails::GetEmailsResponse;
use h_mail_interface::interface::routes::auth::authenticate::AuthenticateRequest;
use h_mail_interface::interface::routes::auth::authenticate::AuthenticateResponse;
use h_mail_interface::interface::routes::auth::refresh_access::RefreshAccessRequest;
use h_mail_interface::interface::routes::auth::refresh_access::RefreshAccessResponse;
use h_mail_interface::interface::routes::auth::check_auth::CheckAuthRequest;
use h_mail_interface::interface::routes::auth::check_auth::CheckAuthResponseAuthed;
use h_mail_interface::interface::routes::auth::check_auth::CheckAuthResponse;


pub fn all() -> Vec<(Schema, &'static str, Option<&'static str>, Option<&'static str>)> {
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
        (CheckPowRequest, Some("routes/check_pow"), None),
        (GetUserPowPolicyRequest, Some("routes/foreign/get_user_pow_policy"), None),
        (GetUserPowPolicyResponse, Some("routes/foreign/get_user_pow_policy"), None),
        (VerifyIpRequest, Some("routes/foreign/verify_ip"), None),
        (VerifyIpResponse, Some("routes/foreign/verify_ip"), None),
        (DeliverEmailRequest, Some("routes/foreign/deliver_email"), None),
        (DeliverEmailResponse, Some("routes/foreign/deliver_email"), None),
        (GetPowTokenRequest, Some("routes/get_pow_token"), None),
        (GetPowTokenResponse, Some("routes/get_pow_token"), None),
        (CreateAccountPackage, Some("routes/native/create_account"), None),
        (CreateAccountResponse, Some("routes/native/create_account"), None),
        (CreateAccountRequest, Some("routes/native/create_account"), None),
        (SendEmailRequest, Some("routes/native/send_email"), None),
        (SendEmailResponseAuthed, Some("routes/native/send_email"), None),
        (SendEmailResponse, Some("routes/native/send_email"), None),
        (GetCreateAccountPowPolicyRequest, Some("routes/native/get_create_account_pow_policy"), None),
        (GetCreateAccountPowPolicyResponse, Some("routes/native/get_create_account_pow_policy"), None),
        (GetEmailsRequest, Some("routes/native/get_emails"), None),
        (GetEmailsEmail, Some("routes/native/get_emails"), None),
        (GetEmailsResponseAuthed, Some("routes/native/get_emails"), None),
        (GetEmailsResponse, Some("routes/native/get_emails"), None),
        (AuthenticateRequest, Some("routes/auth/authenticate"), None),
        (AuthenticateResponse, Some("routes/auth/authenticate"), None),
        (RefreshAccessRequest, Some("routes/auth/refresh_access"), None),
        (RefreshAccessResponse, Some("routes/auth/refresh_access"), None),
        (CheckAuthRequest, Some("routes/auth/check_auth"), None),
        (CheckAuthResponseAuthed, Some("routes/auth/check_auth"), None),
        (CheckAuthResponse, Some("routes/auth/check_auth"), None)
    ]
}