use reqwest::IntoUrl;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub async fn send_post<U: IntoUrl, T: Serialize, R: DeserializeOwned>(
    destination: U,
    data: &T,
) -> Result<R, ()> {
    let client = reqwest::Client::new();
    match client.post(destination).json(data).send().await {
        Ok(r) => match r.json::<R>().await {
            Ok(r) => Ok(r),
            Err(_) => Err(()),
        },
        Err(_) => Err(()),
    }
}
