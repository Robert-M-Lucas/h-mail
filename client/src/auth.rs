use crate::get_server_path;
use crate::send::send_post;
use crate::util::read_line;
use h_mail_interface::interface::auth::AuthToken;
use h_mail_interface::interface::fields::auth_token::AuthTokenField;
use h_mail_interface::interface::routes::auth::authenticate::{
    AuthenticateRequest, AuthenticateResponse,
};
use h_mail_interface::interface::routes::auth::refresh_access::{
    RefreshAccessRequest, RefreshAccessResponse,
};
use once_cell::sync::Lazy;
use tokio::fs;
use tokio::sync::RwLock;

static ACCESS_TOKEN: Lazy<RwLock<Option<AuthToken>>> = Lazy::new(|| RwLock::new(None));

// Get access token (short-lived e.g. 10 minutes, not stored to disk)
pub async fn get_access_token() -> Option<AuthToken> {
    ACCESS_TOKEN.read().await.clone()
}

// Generate access token from refresh token
pub async fn refresh_access_token() -> Result<AuthToken, ()> {
    let mut reauthed = false; // Have we tried reauthenticating and getting a new refresh toke? 

    let mut refresh_token = match get_refresh_token_disk().await {
        Some(token) => token, // Refresh token saved
        None => {
            reauthed = true;
            let rt = reauthenticate_refresh_token().await;
            if let Ok(rt) = rt.as_ref() {
                write_refresh_token_disk(rt).await.ok();
            }
            rt?
        }
    };

    loop {
        match send_post::<_, _, RefreshAccessResponse>(
            get_server_path("auth/refresh_access").await,
            &RefreshAccessRequest::new(AuthTokenField::new(&refresh_token)),
        )
        .await
        {
            Ok(r) => {
                match r {
                    RefreshAccessResponse::Failure => {
                        if reauthed { // Don't reauth twice
                            break Err(());
                        }

                        // Need to reauthenticate
                        let rt = reauthenticate_refresh_token().await?;
                        write_refresh_token_disk(&rt).await.ok();
                        refresh_token = rt;
                        reauthed = true;
                    }
                    RefreshAccessResponse::BadRequest => break Err(()),
                    RefreshAccessResponse::Success(at) => {
                        let at = at.token().decode()?;
                        ACCESS_TOKEN.write().await.replace(at.clone());
                        break Ok(at);
                    }
                }
            }
            Err(_) => {
                break Err(());
            }
        }
    }
}

// Regenerate refresh token (long-lived e.g. 1 month, stored securely on disk out of memory) from credentials
async fn reauthenticate_refresh_token() -> Result<AuthToken, ()> {
    loop {
        println!("Enter username: ");
        let username = read_line();
        println!("Enter password: ");
        let password = read_line();

        match send_post::<_, _, AuthenticateResponse>(
            get_server_path("auth/authenticate").await,
            &AuthenticateRequest::new(username, password),
        )
        .await
        {
            Ok(r) => match r {
                AuthenticateResponse::Failure => {
                    println!("Username or password is invalid\n");
                }
                AuthenticateResponse::Success(rt) => break rt.token().decode(),
            },
            Err(_) => {
                break Err(());
            }
        }
    }
}

async fn get_refresh_token_disk() -> Option<AuthToken> {
    AuthTokenField(fs::read_to_string("refresh_token").await.ok()?)
        .decode()
        .ok()
}

async fn write_refresh_token_disk(token: &AuthToken) -> Result<(), ()> {
    fs::write("refresh_token", AuthTokenField::new(token).0)
        .await
        .map_err(|_| ())
}
