use anyhow::Context;
use h_mail_interface::error::HResult;
use reqwest::IntoUrl;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::any::type_name;

pub async fn send_post<U: IntoUrl, T: Serialize, R: DeserializeOwned>(
    destination: U,
    data: &T,
) -> HResult<R> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    match client.post(destination).json(data).send().await {
        Ok(r) => match r.json::<R>().await {
            Ok(r) => Ok(r),
            Err(e) => Err(e).context(format!(
                "Failed to deserialize json response into {}",
                type_name::<R>()
            )),
        },
        Err(e) => Err(e).context("Failed to send post request"),
    }
}
