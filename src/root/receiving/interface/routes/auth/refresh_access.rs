use crate::root::receiving::interface::fields::auth_token::{AuthTokenDataField, AuthTokenField};
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Getters, new)]
pub struct RefreshAccessRequest {
    refresh_token: AuthTokenField,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RefreshAccessResponse {
    Failure,
    BadRequest,
    Success(AuthTokenDataField),
}
