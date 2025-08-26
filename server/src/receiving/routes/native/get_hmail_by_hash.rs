use crate::database::Db;
use crate::receiving::auth_util::auth_header::AuthorizationHeader;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::routes::native::get_hmail_by_hash::{
    GetHmailByHashRequest, GetHmailByHashResponse, GetHmailByHashResponseAuthed,
};

pub async fn get_hmail_by_hash(
    auth_header: AuthorizationHeader,
    Query(get_hmail_by_hash): Query<GetHmailByHashRequest>,
) -> (StatusCode, Json<GetHmailByHashResponse>) {
    let Some(user_id) = auth_header.check_access_token().await else {
        return (StatusCode::OK, Authorized::Unauthorized.into());
    };

    let hmail = Db::get_hmail_by_hash(user_id, get_hmail_by_hash.hash()).await;

    (
        StatusCode::OK,
        Authorized::Success(GetHmailByHashResponseAuthed::new(hmail)).into(),
    )
}
