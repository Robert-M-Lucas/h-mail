use crate::config::CREATE_ACCOUNT_POW_BURDEN;
use crate::database::Db;
use crate::shared_resources::POW_PROVIDER;
use axum::Json;
use axum::http::StatusCode;
use h_mail_interface::interface::pow::PowFailureReason;
use h_mail_interface::interface::routes::native::create_account::{
    CreateAccountRequest, CreateAccountResponse,
};
use h_mail_interface::shared::hash_str;

pub async fn create_account(
    Json(create_account): Json<CreateAccountRequest>,
) -> (StatusCode, Json<CreateAccountResponse>) {
    let Ok(token) = create_account.token().decode() else {
        return (
            StatusCode::BAD_REQUEST,
            CreateAccountResponse::PowFailure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };
    let Ok(pow_result) = create_account.pow_result().decode() else {
        return (
            StatusCode::BAD_REQUEST,
            CreateAccountResponse::PowFailure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };

    if create_account.iters() < CREATE_ACCOUNT_POW_BURDEN {
        return (
            StatusCode::BAD_REQUEST,
            CreateAccountResponse::DoesNotMeetPolicy(CREATE_ACCOUNT_POW_BURDEN).into(),
        );
    }

    // Check POW token and retrieve associated IP
    let hash = hash_str(create_account.username());
    match POW_PROVIDER
        .write()
        .await
        .check_pow(token, create_account.iters(), hash, pow_result)
        .await
    {
        Ok(ip_addr) => ip_addr,
        Err(e) => {
            return (
                StatusCode::EXPECTATION_FAILED,
                CreateAccountResponse::PowFailure(e).into(),
            );
        }
    };

    // Try deliver email (database)
    if Db::create_user(create_account.username(), create_account.password()).is_ok() {
        return (
            StatusCode::EXPECTATION_FAILED,
            CreateAccountResponse::UsernameInUse.into(),
        );
    }

    (StatusCode::OK, CreateAccountResponse::Success.into())
}
