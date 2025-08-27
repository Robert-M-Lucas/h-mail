use crate::config::config_file::CONFIG;
use crate::database::Db;
use crate::shared_resources::POW_PROVIDER;
use axum::Json;
use axum::http::StatusCode;
use h_mail_interface::interface::pow::PowFailureReason;
use h_mail_interface::interface::routes::native::create_account::{
    CreateAccountRequest, CreateAccountResponse,
};

pub async fn create_account(
    Json(create_account): Json<CreateAccountRequest>,
) -> (StatusCode, Json<CreateAccountResponse>) {
    let Ok(create_account) = create_account.decode() else {
        return (
            StatusCode::OK,
            CreateAccountResponse::PowFailure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };

    // Check POW token and retrieve associated IP
    let create_account = match POW_PROVIDER
        .write()
        .await
        .check_pow(create_account, *CONFIG.create_account_pow_burden())
        .await
    {
        Ok(create_account) => create_account,
        Err(e) => {
            return (StatusCode::OK, CreateAccountResponse::PowFailure(e).into());
        }
    };

    match Db::create_user(create_account.username(), create_account.password()).await {
        Ok(username) => (
            StatusCode::OK,
            CreateAccountResponse::Success(username).into(),
        ),
        Err(e) => (StatusCode::OK, e.into_create_account_response().into()),
    }
}
