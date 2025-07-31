use crate::interface::auth::Authorized;
use crate::interface::pow::PowClassification;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

pub const AUTH_CHECK_AUTH_PATH: &str = "/auth/check_auth";

#[derive(Serialize, Deserialize, new, Debug)]
pub struct CheckAuthRequest;

#[derive(Serialize, Deserialize, new, Debug)]
pub struct CheckAuthResponseAuthed;

pub type CheckAuthResponse = Authorized<CheckAuthResponseAuthed>;
