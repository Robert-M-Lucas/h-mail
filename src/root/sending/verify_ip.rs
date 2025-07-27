use crate::root::auth_token_provider::AuthToken;
use crate::root::receiving::interface::fields::auth_token::AuthTokenField;
use crate::root::receiving::interface::routes::foreign::verify_ip::{
    VerifyIpRequest, VerifyIpResponse,
};
use std::net::IpAddr;

pub async fn verify_ip(ip: IpAddr, verify_token: &AuthToken) -> bool {
    let client = reqwest::Client::new();
    let response = match client
        .post(format!("https://{ip}:8081/foreign/verify_ip"))
        .json(&VerifyIpRequest::new(AuthTokenField::new(verify_token)))
        .send()
        .await
    {
        Ok(r) => match r.json::<VerifyIpResponse>().await {
            Ok(r) => r,
            Err(_) => return false,
        },
        Err(_) => {
            return false;
        }
    };

    match response {
        VerifyIpResponse::Success => true,
        VerifyIpResponse::BadRequest => false,
        VerifyIpResponse::Failure => false,
    }
}
