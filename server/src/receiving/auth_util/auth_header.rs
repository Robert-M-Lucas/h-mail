use crate::database::UserId;
use crate::shared_resources::ACCESS_TOKEN_PROVIDER;
use axum::Extension;
use axum::extract::FromRequestParts;
use axum::http::HeaderValue;
use axum::http::request::Parts;
use h_mail_interface::interface::auth::AuthToken;

#[derive(Clone)]
pub struct AuthorizationHeader(Option<AuthToken>);

impl AuthorizationHeader {
    pub fn from_auth_header(header: Option<&HeaderValue>) -> AuthorizationHeader {
        fn parse_header_str(header_str: &str) -> Option<AuthToken> {
            let mut split = header_str.split(" ");
            let bearer = split.next()?;
            if bearer != "Bearer" {
                return None;
            }
            let token = split.next()?;
            AuthToken::from_string(token).ok()
        }

        let inner = match header {
            Some(header_value) => match header_value.to_str() {
                Ok(header_str) => parse_header_str(header_str),
                Err(_) => None,
            },
            None => None,
        };

        AuthorizationHeader(inner)
    }

    pub async fn check_access_token(&self) -> Option<UserId> {
        let token = self.0.clone()?;

        ACCESS_TOKEN_PROVIDER.write().await.validate_token(&token)
    }
}

impl<S> FromRequestParts<S> for AuthorizationHeader
where
    S: Send + Sync,
{
    type Rejection = <Extension<Self> as FromRequestParts<S>>::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Extension::<Self>::from_request_parts(parts, state).await {
            Ok(Extension(auth_header)) => Ok(auth_header),
            Err(err) => Err(err),
        }
    }
}
