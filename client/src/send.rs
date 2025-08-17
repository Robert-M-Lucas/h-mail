use crate::auth::{AuthError, AuthResult, get_access_token, refresh_access_token};
use h_mail_interface::error::HResult;
use h_mail_interface::interface::RequestMethod;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::fields::auth_token::AuthTokenField;
use h_mail_interface::reexports::anyhow::{Context, anyhow};
use h_mail_interface::utility::get_url_for_path;
use reqwest::RequestBuilder;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::any::type_name;

async fn send_internal<R: DeserializeOwned>(request_builder: RequestBuilder) -> HResult<R> {
    match request_builder.send().await {
        Ok(r) => {
            // let text = r.text().await?;
            // warn!("!! {}", text);
            match r.json::<R>().await {
                // match serde_json::from_str(&text) {
                Ok(r) => Ok(r),
                Err(e) => Err(anyhow!(
                    "Failed to deserialise json to {} - {e:?}",
                    type_name::<R>()
                )),
            }
        }
        Err(e) => Err(e).context("Failed to send request to server"),
    }
}

pub async fn send_post<S: Serialize, R: DeserializeOwned, T1: AsRef<str>, T2: AsRef<str>>(
    server: T1,
    path: T2,
    data: &S,
) -> HResult<R> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    send_internal(client.post(get_url_for_path(server, path)).json(data)).await
}

pub async fn send_get<S: Serialize, R: DeserializeOwned, T1: AsRef<str>, T2: AsRef<str>>(
    server: T1,
    path: T2,
    data: &S,
) -> HResult<R> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    send_internal(client.get(get_url_for_path(server, path)).query(data)).await
}

pub async fn send_delete<S: Serialize, R: DeserializeOwned, T1: AsRef<str>, T2: AsRef<str>>(
    server: T1,
    path: T2,
    data: &S,
) -> HResult<R> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    send_internal(client.delete(get_url_for_path(server, path)).query(data)).await
}

pub async fn send<S: Serialize, R: DeserializeOwned, T1: AsRef<str>, T2: AsRef<str>>(
    server: T1,
    path: T2,
    data: &S,
    request_method: RequestMethod,
) -> HResult<R> {
    match request_method {
        RequestMethod::Post => send_post(server, path, data).await,
        RequestMethod::Get => send_get(server, path, data).await,
        RequestMethod::Delete => send_delete(server, path, data).await,
    }
}

pub async fn send_post_auth<S: Serialize, R: DeserializeOwned, T1: AsRef<str>, T2: AsRef<str>>(
    server: T1,
    path: T2,
    data: &S,
) -> AuthResult<R> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let destination = get_url_for_path(server.as_ref(), path);

    let mut refreshed = false; // Have we tried refreshing the access token
    let mut token = match get_access_token().await {
        Some(t) => t,
        None => {
            refreshed = true;
            refresh_access_token(server.as_ref()).await?
        }
    };

    loop {
        let token_str = AuthTokenField::new(&token).0;

        let result: Authorized<R> = send_internal(
            client
                .post(destination.as_str())
                .json(data)
                .bearer_auth(token_str),
        )
        .await?;

        match result {
            Authorized::Success(r) => {
                return Ok(r);
            }
            Authorized::Unauthorized => {
                if refreshed {
                    return Err(AuthError::RequireReauth);
                }

                refreshed = true;
                token = refresh_access_token(server.as_ref()).await?;
            }
        }
    }
}

pub async fn send_get_auth<S: Serialize, R: DeserializeOwned, T1: AsRef<str>, T2: AsRef<str>>(
    server: T1,
    path: T2,
    data: &S,
) -> AuthResult<R> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let destination = get_url_for_path(server.as_ref(), path);

    let mut refreshed = false; // Have we tried refreshing the access token
    let mut token = match get_access_token().await {
        Some(t) => t,
        None => {
            refreshed = true;
            refresh_access_token(server.as_ref()).await?
        }
    };

    loop {
        let token_str = AuthTokenField::new(&token).0;

        let result: Authorized<R> = send_internal(
            client
                .get(destination.as_str())
                .query(data)
                .bearer_auth(token_str),
        )
        .await?;

        match result {
            Authorized::Success(r) => {
                return Ok(r);
            }
            Authorized::Unauthorized => {
                if refreshed {
                    return Err(AuthError::RequireReauth);
                }

                refreshed = true;
                token = refresh_access_token(server.as_ref()).await?;
            }
        }
    }
}
pub async fn send_delete_auth<S: Serialize, R: DeserializeOwned, T1: AsRef<str>, T2: AsRef<str>>(
    server: T1,
    path: T2,
    data: &S,
) -> AuthResult<R> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let destination = get_url_for_path(server.as_ref(), path);

    let mut refreshed = false; // Have we tried refreshing the access token
    let mut token = match get_access_token().await {
        Some(t) => t,
        None => {
            refreshed = true;
            refresh_access_token(server.as_ref()).await?
        }
    };

    loop {
        let token_str = AuthTokenField::new(&token).0;

        let result: Authorized<R> = send_internal(
            client
                .delete(destination.as_str())
                .query(data)
                .bearer_auth(token_str),
        )
        .await?;

        match result {
            Authorized::Success(r) => {
                return Ok(r);
            }
            Authorized::Unauthorized => {
                if refreshed {
                    return Err(AuthError::RequireReauth);
                }

                refreshed = true;
                token = refresh_access_token(server.as_ref()).await?;
            }
        }
    }
}

pub async fn send_auth<S: Serialize, R: DeserializeOwned, T1: AsRef<str>, T2: AsRef<str>>(
    server: T1,
    path: T2,
    data: &S,
    request_method: RequestMethod,
) -> AuthResult<R> {
    match request_method {
        RequestMethod::Post => send_post_auth(server, path, data).await,
        RequestMethod::Get => send_get_auth(server, path, data).await,
        RequestMethod::Delete => send_delete_auth(server, path, data).await,
    }
}
