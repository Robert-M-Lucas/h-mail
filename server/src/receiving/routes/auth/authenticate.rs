use crate::database::Db;
use crate::shared_resources::REFRESH_TOKEN_PROVIDER;
use axum::Json;
use axum::http::StatusCode;
use h_mail_interface::interface::fields::auth_token::AuthTokenDataField;
use h_mail_interface::interface::routes::auth::authenticate::{
    AuthenticateRequest, AuthenticateResponse,
};

pub async fn authenticate(
    Json(authentication_request): Json<AuthenticateRequest>,
) -> (StatusCode, Json<AuthenticateResponse>) {
    let user_id = Db::authenticate(
        authentication_request.username(),
        authentication_request.password(),
    )
    .await;
    match user_id {
        Ok(user_id) => {
            let refresh_token = REFRESH_TOKEN_PROVIDER.write().await.get_token(user_id);
            (
                StatusCode::OK,
                AuthenticateResponse::Success(AuthTokenDataField::new(&refresh_token)).into(),
            )
        }
        Err(_) => (StatusCode::OK, AuthenticateResponse::Failure.into()),
    }
}
