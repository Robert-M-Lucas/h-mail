use crate::auth::AuthError::Other;
use crate::get_data_location;
use crate::send::send_post;
use crate::state::{get_server_address, wipe_old_tokens};
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
use h_mail_interface::reexports::AnyhowError;
use h_mail_interface::reexports::anyhow::{Context, anyhow, bail};
use h_mail_interface::utility::bytes_to_base64;
use itertools::Itertools;
use once_cell::sync::Lazy;
use tokio::fs;
use tokio::sync::RwLock;

static ACCESS_TOKEN: Lazy<RwLock<Option<AuthToken>>> = Lazy::new(|| RwLock::new(None));

#[derive(Debug)]
pub enum AuthError {
    RequireReauth,
    Other(AnyhowError),
}

impl From<AnyhowError> for AuthError {
    fn from(err: AnyhowError) -> Self {
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
pub async fn refresh_access_token<T: AsRef<str>>(server: T) -> AuthResult<AuthToken> {
    let refresh_token = match get_refresh_token_disk(server.as_ref()).await {
        Some(token) => token, // Refresh token saved
        None => {
            return Err(AuthError::RequireReauth);
        }
    };

    let r = send_post::<_, RefreshAccessResponse, _, _>(
        server,
        AUTH_REFRESH_ACCESS_PATH,
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
    reauthenticate_s(get_server_address().await?, auth_credentials).await
}

pub async fn reauthenticate_s<T: AsRef<str>>(
    server: T,
    auth_credentials: AuthCredentials,
) -> HResult<()> {
    let (username, password) = auth_credentials.dissolve();

    let r = send_post::<_, AuthenticateResponse, _, _>(
        server.as_ref(),
        AUTH_AUTHENTICATE_PATH,
        &AuthenticateRequest::new(username, password),
    )
    .await?;

    match r {
        AuthenticateResponse::Failure => {
            bail!("Username or password is invalid")
        }
        AuthenticateResponse::Success(rt) => {
            let rt = rt.token().decode()?;
            write_refresh_token_disk(server.as_ref(), &rt).await?;
            Ok(())
        }
    }
}

async fn get_refresh_token_disk<T: AsRef<str>>(server: T) -> Option<AuthToken> {
    let b = server.as_ref().bytes().collect_vec();
    let path = bytes_to_base64(&b);

    AuthTokenField(
        fs::read_to_string(
            get_data_location()
                .ok()?
                .join(format!("refresh_token-{path}")),
        )
        .await
        .ok()?,
    )
    .decode()
    .ok()
}

async fn remove_all_refresh_tokens_disk() -> HResult<()> {
    let mut read_dir = fs::read_dir(get_data_location()?)
        .await
        .context("Failed to read token directory")?;
    while let Some(entry) = read_dir.next_entry().await? {
        if entry.file_type().await?.is_dir()
            || !entry
                .file_name()
                .to_string_lossy()
                .starts_with("refresh_token")
        {
            continue;
        }
        fs::remove_file(entry.path()).await?;
    }
    Ok(())
}

async fn write_refresh_token_disk<T: AsRef<str>>(server: T, token: &AuthToken) -> HResult<()> {
    if wipe_old_tokens().await {
        remove_all_refresh_tokens_disk().await?;
    }

    let b = server.as_ref().bytes().collect_vec();
    let path = bytes_to_base64(&b);

    fs::write(
        get_data_location()?.join(format!("refresh_token-{path}")),
        AuthTokenField::new(token).0,
    )
    .await
    .context("Failed to write refresh token to disk")
}

pub async fn logout() -> HResult<()> {
    remove_all_refresh_tokens_disk().await?;
    *ACCESS_TOKEN.write().await = None;

    Ok(())
}
