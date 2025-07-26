use axum::Extension;
use axum::extract::FromRequestParts;
use axum::http::HeaderValue;
use axum::http::request::Parts;

#[derive(Clone)]
pub struct AuthorizationHeader(Option<String>);

impl AuthorizationHeader {
    pub fn from_auth_header(header: Option<&HeaderValue>) -> AuthorizationHeader {
        fn parse_header_str(header_str: &str) -> Option<String> {
            let mut split = header_str.split(" ");
            let bearer = split.next()?;
            if bearer != "Bearer" {
                return None;
            }
            let token = split.next()?;
            Some(token.to_string())
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
