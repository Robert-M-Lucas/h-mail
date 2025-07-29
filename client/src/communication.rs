use crate::auth::AuthResult;
use crate::send::send_get_auth;
use crate::state::get_url_for_path;
use h_mail_interface::interface::routes::native::get_emails::{
    GetEmailsRequest, GetEmailsResponseAuthed, NATIVE_GET_EMAILS_PATH,
};
use std::borrow::Borrow;

pub async fn get_emails<G: Borrow<GetEmailsRequest>>(
    get_emails_request: G,
) -> AuthResult<GetEmailsResponseAuthed> {
    send_get_auth(
        get_url_for_path(NATIVE_GET_EMAILS_PATH).await,
        get_emails_request.borrow(),
    )
    .await
}
