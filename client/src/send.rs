use crate::auth::get_access_token;
use h_mail_interface::interface::fields::auth_token::AuthTokenField;
use reqwest::{IntoUrl, RequestBuilder};
use serde::Serialize;
use serde::de::DeserializeOwned;

async fn send<R: DeserializeOwned>(request_builder: RequestBuilder) -> Result<R, ()> {
    match request_builder.send().await {
        Ok(r) => match r.json::<R>().await {
            Ok(r) => Ok(r),
            Err(_) => Err(()),
        },
        Err(e) => {
            println!("{e:?}");
            Err(())
        }
    }
}

pub async fn send_post<U: IntoUrl, T: Serialize, R: DeserializeOwned>(
    destination: U,
    data: &T,
) -> Result<R, ()> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    send(client.post(destination).json(data)).await
}

pub async fn send_get<U: IntoUrl, T: Serialize, R: DeserializeOwned>(
    destination: U,
    data: &T,
) -> Result<R, ()> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    send(client.get(destination).query(data)).await
}

pub async fn send_get_auth<U: IntoUrl, T: Serialize, R: DeserializeOwned>(
    destination: U,
    data: &T,
) -> Result<R, ()> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let token_str = AuthTokenField::new(
        &get_access_token()
            .await
            .expect("Failed to get access token"),
    )
    .0;
    send(client.get(destination).query(data).bearer_auth(token_str)).await
}

pub async fn send_post_auth<U: IntoUrl, T: Serialize, R: DeserializeOwned>(
    destination: U,
    data: &T,
) -> Result<R, ()> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let token_str = AuthTokenField::new(
        &get_access_token()
            .await
            .expect("Failed to get access token"),
    )
    .0;
    send(client.post(destination).json(data).bearer_auth(token_str)).await
}
