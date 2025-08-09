use std::collections::HashMap;
use crate::config::config_file::CONFIG;
use crate::database::Db;
use crate::receiving::auth_util::auth_header::AuthorizationHeader;
use crate::sending::send_post::send_post;
use crate::shared_resources::VERIFY_IP_TOKEN_PROVIDER;
use axum::Json;
use axum::http::StatusCode;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::fields::auth_token::AuthTokenDataField;
use h_mail_interface::interface::routes::foreign::deliver_email::{
    DeliverEmailRequest, DeliverEmailResponse,
};
use h_mail_interface::interface::routes::native::send_email::{
    SendEmailRequest, SendEmailResponse, SendEmailResponseAuthed,
};
use std::io::{Write, stdout};
use futures::future::join_all;
use itertools::Itertools;
use tracing::error;
use h_mail_interface::interface::email::{Email, EmailUser, SendEmailPackage};
use h_mail_interface::interface::pow::PowResultDecoded;

pub async fn send_email(
    auth_header: AuthorizationHeader,
    Json(send_email): Json<SendEmailRequest>,
) -> (StatusCode, Json<SendEmailResponse>) {
    let Some(user_id) = auth_header.check_access_token().await else {
        return (StatusCode::UNAUTHORIZED, Authorized::Unauthorized.into());
    };

    let username = Db::get_username_from_id(user_id).unwrap();

    let (email, solved_pow_for) = send_email.dissolve();
    let mut solved_pow_for_decoded = HashMap::with_capacity(solved_pow_for.len());
    for target in solved_pow_for {
        let (solved_for, solved_pow) = target.dissolve();
        let Ok(solved_pow) = solved_pow.decode() else {
            return (StatusCode::EXPECTATION_FAILED, Authorized::Success(SendEmailResponseAuthed::BadRequest).into())
        };
        solved_pow_for_decoded.insert(solved_for, solved_pow);
    }
    let mut solved_pow_for = solved_pow_for_decoded;

    let mut delivering_to: Vec<((&str, &str), PowResultDecoded)> = Vec::with_capacity(email.to().len() + email.cc().len());
    for to in email.to().iter().chain(email.cc().iter()) {
        if delivering_to.iter().any(|(t, _)| t.email() == to.email()) {
            return (StatusCode::EXPECTATION_FAILED, Authorized::Success(SendEmailResponseAuthed::DuplicateDestination).into())
        }
        let Some(pow_result) = solved_pow_for.remove(to.email()) else {
            return (StatusCode::EXPECTATION_FAILED, Authorized::Success(SendEmailResponseAuthed::MissingPowFor(to.email().to_string())).into())
        };
        let mut split = to.email().split('@');
        let Some(user) = split.next() else {
            return (StatusCode::EXPECTATION_FAILED, Authorized::Success(SendEmailResponseAuthed::BadRequest).into())
        };
        let Some(domain) = split.next() else {
            return (StatusCode::EXPECTATION_FAILED, Authorized::Success(SendEmailResponseAuthed::BadRequest).into())
        };
        if split.next().is_some() {
            return (StatusCode::EXPECTATION_FAILED, Authorized::Success(SendEmailResponseAuthed::BadRequest).into())
        }

        delivering_to.push(((user, domain), pow_result));
    }

    let results: Vec<Result<DeliverEmailResponse, ()>> = join_all(delivering_to.into_iter().map(|((user, domain), pow_result)|
        send_email_to(&username, user, domain, pow_result, &email)
    )).await.collect_vec();


}

async fn send_email_to(source_user: &str, destination_user: &str, destination_domain: &str, pow_result: PowResultDecoded, email: &SendEmailPackage) -> Result<DeliverEmailResponse, ()> {
    // ! Do not lock resource
    let verify_ip_token = VERIFY_IP_TOKEN_PROVIDER.write().await.get_token(());

    stdout().flush().ok();
    match send_post::<_, _, DeliverEmailResponse>(
        format!("https://{}/foreign/deliver_email", &destination_domain),
        &DeliverEmailRequest::new(
            Email::new(email.clone(), pow_result.encode()),
            source_user.to_string(),
            CONFIG.domain().to_string(),
            destination_user.to_string(),
            destination_domain.to_string(),
            AuthTokenDataField::new(&verify_ip_token),
            CONFIG.port(),
        ),
    )
        .await
    {
        Ok(r) => Ok(r),
        Err(e) => {
            error!("Failed to send email: {e:?}");
            Err(())
        }
    }
}
