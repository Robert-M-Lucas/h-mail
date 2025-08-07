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
use h_mail_interface::interface::routes::auth::authenticate::AuthenticateRequest;
use h_mail_interface::interface::routes::auth::authenticate::AuthenticateResponse;
use h_mail_interface::interface::routes::auth::check_auth::CheckAuthRequest;
use h_mail_interface::interface::routes::auth::check_auth::CheckAuthResponse;
use h_mail_interface::interface::routes::auth::check_auth::CheckAuthResponseAuthed;
use h_mail_interface::interface::routes::auth::refresh_access::RefreshAccessRequest;
use h_mail_interface::interface::routes::auth::refresh_access::RefreshAccessResponse;
use h_mail_interface::interface::routes::check_pow::CheckPowPackage;
use h_mail_interface::interface::routes::check_pow::CheckPowRequest;
use h_mail_interface::interface::routes::check_pow::CheckPowResponse;
use h_mail_interface::interface::routes::foreign::deliver_email::DeliverEmailRequest;
use h_mail_interface::interface::routes::foreign::deliver_email::DeliverEmailResponse;
use h_mail_interface::interface::routes::foreign::get_pow_token::GetPowTokenRequest;
use h_mail_interface::interface::routes::foreign::get_pow_token::GetPowTokenResponse;
use h_mail_interface::interface::routes::foreign::get_user_pow_policy::GetUserPowPolicyRequest;
use h_mail_interface::interface::routes::foreign::get_user_pow_policy::GetUserPowPolicyResponse;
use h_mail_interface::interface::routes::foreign::verify_ip::VerifyIpRequest;
use h_mail_interface::interface::routes::foreign::verify_ip::VerifyIpResponse;
use h_mail_interface::interface::routes::native::create_account::CreateAccountPackage;
use h_mail_interface::interface::routes::native::create_account::CreateAccountRequest;
use h_mail_interface::interface::routes::native::create_account::CreateAccountResponse;
use h_mail_interface::interface::routes::native::get_create_account_pow_policy::GetCreateAccountPowPolicyRequest;
use h_mail_interface::interface::routes::native::get_create_account_pow_policy::GetCreateAccountPowPolicyResponse;
use h_mail_interface::interface::routes::native::get_emails::GetEmailsEmail;
use h_mail_interface::interface::routes::native::get_emails::GetEmailsRequest;
use h_mail_interface::interface::routes::native::get_emails::GetEmailsResponse;
use h_mail_interface::interface::routes::native::get_emails::GetEmailsResponseAuthed;
use h_mail_interface::interface::routes::native::send_email::SendEmailRequest;
use h_mail_interface::interface::routes::native::send_email::SendEmailResponse;
use h_mail_interface::interface::routes::native::send_email::SendEmailResponseAuthed;
use schemars::Schema;

pub fn all() -> Vec<(Schema, &'static str, &'static str)> {
    gen_schemas![
        (PowFailureReason, "pow"),
        (PowPolicy, "pow"),
        (PowClassification, "pow"),
        (BigUintField, "fields/big_uint"),
        (SystemTimeField, "fields/system_time"),
        (AuthTokenField, "fields/auth_token"),
        (AuthTokenDataField, "fields/auth_token"),
        (CheckPowPackage, "routes/check_pow"),
        (CheckPowResponse, "routes/check_pow"),
        (CheckPowRequest, "routes/check_pow"),
        (RefreshAccessRequest, "routes/auth/refresh_access"),
        (RefreshAccessResponse, "routes/auth/refresh_access"),
        (CheckAuthRequest, "routes/auth/check_auth"),
        (CheckAuthResponseAuthed, "routes/auth/check_auth"),
        (CheckAuthResponse, "routes/auth/check_auth"),
        (AuthenticateRequest, "routes/auth/authenticate"),
        (AuthenticateResponse, "routes/auth/authenticate"),
        (DeliverEmailRequest, "routes/foreign/deliver_email"),
        (DeliverEmailResponse, "routes/foreign/deliver_email"),
        (
            GetUserPowPolicyRequest,
            "routes/foreign/get_user_pow_policy"
        ),
        (
            GetUserPowPolicyResponse,
            "routes/foreign/get_user_pow_policy"
        ),
        (GetPowTokenRequest, "routes/foreign/get_pow_token"),
        (GetPowTokenResponse, "routes/foreign/get_pow_token"),
        (VerifyIpRequest, "routes/foreign/verify_ip"),
        (VerifyIpResponse, "routes/foreign/verify_ip"),
        (GetEmailsRequest, "routes/native/get_emails"),
        (GetEmailsEmail, "routes/native/get_emails"),
        (GetEmailsResponseAuthed, "routes/native/get_emails"),
        (GetEmailsResponse, "routes/native/get_emails"),
        (CreateAccountPackage, "routes/native/create_account"),
        (CreateAccountResponse, "routes/native/create_account"),
        (CreateAccountRequest, "routes/native/create_account"),
        (SendEmailRequest, "routes/native/send_email"),
        (SendEmailResponseAuthed, "routes/native/send_email"),
        (SendEmailResponse, "routes/native/send_email"),
        (
            GetCreateAccountPowPolicyRequest,
            "routes/native/get_create_account_pow_policy"
        ),
        (
            GetCreateAccountPowPolicyResponse,
            "routes/native/get_create_account_pow_policy"
        ),
        (EmailPackage, "email"),
        (Email, "email")
    ]
}
