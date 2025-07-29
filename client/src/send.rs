use crate::auth::{AuthError, AuthResult, get_access_token, refresh_access_token};
use anyhow::{Context, anyhow};
use h_mail_interface::error::HResult;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::fields::auth_token::AuthTokenField;
use reqwest::{IntoUrl, RequestBuilder};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::any::type_name;
use std::borrow::Borrow;

async fn send<R: DeserializeOwned>(request_builder: RequestBuilder) -> HResult<R> {
    match request_builder.send().await {
        Ok(r) => match r.json::<R>().await {
            Ok(r) => Ok(r),
            Err(_) => Err(anyhow!(
                "Failed to deserialise json to {}",
                type_name::<R>()
            )),
        },
        Err(e) => Err(e).context("Failed to send request to server"),
    }
}

pub async fn send_post<U: IntoUrl, T: AsRef<S>, S: Serialize, R: DeserializeOwned>(
    destination: U,
    data: T,
) -> HResult<R> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    send(client.post(destination).json(data.as_ref())).await
}

pub async fn send_get<U: IntoUrl, T: AsRef<S>, S: Serialize, R: DeserializeOwned>(
    destination: U,
    data: T,
) -> HResult<R> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    send(client.get(destination).query(data.as_ref())).await
}

pub async fn send_get_auth<U: IntoUrl, T: AsRef<S>, S: Serialize, R: DeserializeOwned>(
    destination: U,
    data: T,
) -> AuthResult<R> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let mut refreshed = false; // Have we tried refreshing the access token
    let mut token = match get_access_token().await {
        Some(t) => t,
        None => {
            refreshed = true;
            refresh_access_token().await?
        }
    };

    loop {
        let token_str = AuthTokenField::new(&token).0;

        let result: Authorized<R> = send(
            client
                .get(destination.as_str())
                .query(data.as_ref())
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
                token = refresh_access_token().await?;
            }
        }
    }
}

pub async fn send_post_auth<U: IntoUrl, T: AsRef<S>, S: Serialize, R: DeserializeOwned>(
    destination: U,
    data: T,
) -> AuthResult<R> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let mut refreshed = false; // Have we tried refreshing the access token
    let mut token = match get_access_token().await {
        Some(t) => t,
        None => {
            refreshed = true;
            refresh_access_token().await?
        }
    };

    loop {
        let token_str = AuthTokenField::new(&token).0;

        let result: Authorized<R> = send(
            client
                .post(destination.as_str())
                .json(data.as_ref())
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
                token = refresh_access_token().await?;
            }
        }
    }
}
