use crate::shared_resources::{ACCESS_TOKEN_PROVIDER, REFRESH_TOKEN_PROVIDER};
use axum::Json;
use axum::http::StatusCode;
use h_mail_interface::interface::fields::auth_token::AuthTokenDataField;
use h_mail_interface::interface::routes::auth::refresh_access::{
    RefreshAccessRequest, RefreshAccessResponse,
};

pub async fn refresh_access(
    Json(refresh_access_request): Json<RefreshAccessRequest>,
) -> (StatusCode, Json<RefreshAccessResponse>) {
    let Ok(token) = refresh_access_request.refresh_token().decode() else {
        return (
            StatusCode::OK,
            RefreshAccessResponse::BadRequest.into(),
        );
    };
    let user_id = REFRESH_TOKEN_PROVIDER.write().await.validate_token(&token);

    match user_id {
        Some(user_id) => {
            let access_token = ACCESS_TOKEN_PROVIDER.write().await.get_token(user_id);
            (
                StatusCode::OK,
                RefreshAccessResponse::Success(AuthTokenDataField::new(&access_token)).into(),
            )
        }
        None => (
            StatusCode::OK,
            RefreshAccessResponse::Failure.into(),
        ),
    }
}
