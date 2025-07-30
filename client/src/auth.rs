use crate::auth::AuthError::Other;
use crate::send::send_post;
use crate::state::get_url_for_path;
use anyhow::{Context, anyhow, bail};
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use h_mail_interface::error::HResult;
use h_mail_interface::interface::auth::AuthToken;
use h_mail_interface::interface::fields::auth_token::AuthTokenField;
use h_mail_interface::interface::routes::auth::authenticate::{
    AUTH_AUTHENTICATE_PATH, AuthenticateRequest, AuthenticateResponse,
};
use h_mail_interface::interface::routes::auth::refresh_access::{
    AUTH_REFRESH_ACCESS_PATH, RefreshAccessRequest, RefreshAccessResponse,
};
use once_cell::sync::Lazy;
use tokio::fs;
use tokio::sync::RwLock;

static ACCESS_TOKEN: Lazy<RwLock<Option<AuthToken>>> = Lazy::new(|| RwLock::new(None));

#[derive(Debug)]
pub enum AuthError {
    RequireReauth,
    Other(anyhow::Error),
}

impl From<anyhow::Error> for AuthError {
    fn from(err: anyhow::Error) -> Self {
        Other(err)
    }
}

#[derive(Debug, new, Getters, Dissolve)]
pub struct AuthCredentials {
    username: String,
    password: String,
}

pub type AuthResult<T> = Result<T, AuthError>;

// Get access token (short-lived e.g. 10 minutes, not stored to disk)
pub async fn get_access_token() -> Option<AuthToken> {
    ACCESS_TOKEN.read().await.clone()
}

// Generate access token from refresh token
pub async fn refresh_access_token() -> AuthResult<AuthToken> {
    let refresh_token = match get_refresh_token_disk().await {
        Some(token) => token, // Refresh token saved
        None => {
            return Err(AuthError::RequireReauth);
        }
    };

    let r = send_post::<_, _, RefreshAccessResponse>(
        get_url_for_path(AUTH_REFRESH_ACCESS_PATH).await,
        &RefreshAccessRequest::new(AuthTokenField::new(&refresh_token)),
    )
    .await?;
    match r {
        RefreshAccessResponse::Failure => Err(AuthError::RequireReauth),
        RefreshAccessResponse::BadRequest => Err(anyhow!("Bad refresh token request").into()),
        RefreshAccessResponse::Success(at) => {
            let at = at.token().decode()?;
            ACCESS_TOKEN.write().await.replace(at.clone());
            Ok(at)
        }
    }
}

// Regenerate refresh token (long-lived e.g. 1 month, stored securely on disk out of memory) from credentials
pub async fn reauthenticate(auth_credentials: AuthCredentials) -> HResult<()> {
    let (username, password) = auth_credentials.dissolve();

    let r = send_post::<_, _, AuthenticateResponse>(
        get_url_for_path(AUTH_AUTHENTICATE_PATH).await,
        &AuthenticateRequest::new(username, password),
    )
    .await?;

    match r {
        AuthenticateResponse::Failure => {
            bail!("Username or password is invalid")
        }
        AuthenticateResponse::Success(rt) => {
            let rt = rt.token().decode()?;
            write_refresh_token_disk(&rt).await?;
            Ok(())
        }
    }
}

async fn get_refresh_token_disk() -> Option<AuthToken> {
    AuthTokenField(fs::read_to_string("refresh_token").await.ok()?)
        .decode()
        .ok()
}

async fn write_refresh_token_disk(token: &AuthToken) -> HResult<()> {
    fs::write("refresh_token", AuthTokenField::new(token).0)
        .await
        .context("Failed to write refresh token to disk")
}
