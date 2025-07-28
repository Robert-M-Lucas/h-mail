use crate::root::receiving::interface::fields::auth_token::AuthTokenDataField;
use crate::root::receiving::interface::routes::auth::authenticate::{
    AuthenticateRequest, AuthenticateResponse,
};
use crate::root::shared_resources::{DB, REFRESH_TOKEN_PROVIDER};
use axum::Json;
use axum::http::StatusCode;
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
