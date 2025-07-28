use crate::shared_resources::{DB, REFRESH_TOKEN_PROVIDER};
use axum::Json;
use axum::http::StatusCode;
use h_mail_interface::interface::fields::auth_token::AuthTokenDataField;
use h_mail_interface::interface::routes::auth::authenticate::{
    AuthenticateRequest, AuthenticateResponse,
};
use rusqlite::fallible_iterator::FallibleIterator;

pub async fn authenticate(
    Json(authentication_request): Json<AuthenticateRequest>,
) -> (StatusCode, Json<AuthenticateResponse>) {
    let user_id = DB.lock().await.as_ref().unwrap().authenticate(
        authentication_request.username(),
        authentication_request.password(),
    );
    match user_id {
        Ok(user_id) => {
            let refresh_token = REFRESH_TOKEN_PROVIDER.write().await.get_token(user_id);
            (
                StatusCode::OK,
                AuthenticateResponse::Success(AuthTokenDataField::new(&refresh_token)).into(),
            )
        }
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            AuthenticateResponse::Failure.into(),
        ),
    }
}
