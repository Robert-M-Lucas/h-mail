use crate::communication::{InterfaceAuthResult, InterfaceResult};
use h_mail_client::communication::add_whitelist as c_add_whitelist;
use h_mail_client::communication::get_whitelist as c_get_whitelist;
use h_mail_client::communication::remove_whitelist as c_remove_whitelist;
use h_mail_client::interface::fields::hmail_address::HmailAddress;
use h_mail_client::interface::pow::PowClassification;
use h_mail_client::interface::routes::native::add_whitelist::{
    AddWhitelistRequest, AddWhitelistResponseAuthed,
};
use h_mail_client::interface::routes::native::remove_whitelist::{
    RemoveWhitelistRequest, RemoveWhitelistResponseAuthed,
};
use h_mail_client::AuthError;
use itertools::Itertools;
use tracing::debug;

#[tauri::command]
pub async fn get_whitelist() -> InterfaceResult<InterfaceAuthResult<Vec<(String, String)>>> {
    debug!("get_whitelist");
    match c_get_whitelist().await {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(
            v.dissolve()
                .into_iter()
                .map(|e| {
                    let (address, place_in) = e.dissolve();
                    (
                        address.as_str().to_string(),
                        place_in.to_ident().to_string(),
                    )
                })
                .collect_vec(),
        )),
        Err(e) => match e {
            AuthError::RequireReauth => InterfaceResult::Ok(InterfaceAuthResult::Unauthorized),
            AuthError::Other(e) => InterfaceResult::from_error(e),
        },
    }
}

#[tauri::command]
pub async fn remove_whitelist(address: String) -> InterfaceResult<InterfaceAuthResult<bool>> {
    debug!("remove_whitelist");
    match c_remove_whitelist(&RemoveWhitelistRequest::new(address)).await {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(match v {
            RemoveWhitelistResponseAuthed::Success => true,
            RemoveWhitelistResponseAuthed::Failure => false,
        })),
        Err(e) => e.into(),
    }
}

#[tauri::command]
pub async fn add_whitelist(
    address: String,
    classification: String,
) -> InterfaceResult<InterfaceAuthResult<bool>> {
    debug!("add_whitelist");
    let Some(classification) = PowClassification::from_ident(&classification) else {
        return InterfaceResult::Err(format!("Classification {} not found", classification));
    };

    let Ok(address) = HmailAddress::new(&address) else {
        return InterfaceResult::Err("Invalid address".to_string());
    };

    match c_add_whitelist(&AddWhitelistRequest::new(address, classification)).await {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(match v {
            AddWhitelistResponseAuthed::Success => true,
        })),
        Err(e) => e.into(),
    }
}
