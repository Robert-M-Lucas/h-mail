use crate::sending::send_post::send_post;
use crate::shared_resources::{DB, POW_PROVIDER};
use axum::Json;
use axum::extract::ConnectInfo;
use axum::http::StatusCode;
use h_mail_interface::interface::fields::auth_token::AuthTokenField;
use h_mail_interface::interface::pow::PowFailureReason;
use h_mail_interface::interface::routes::foreign::deliver_email::{
    DeliverEmailRequest, DeliverEmailResponse,
};
use h_mail_interface::interface::routes::foreign::verify_ip::{VerifyIpRequest, VerifyIpResponse, FOREIGN_VERIFY_IP_PATH};
#[cfg(not(feature = "no_spf"))]
use mail_auth::spf::verify::SpfParameters;
#[cfg(not(feature = "no_spf"))]
use mail_auth::{MessageAuthenticator, SpfResult};
use std::net::SocketAddr;
use h_mail_interface::shared::get_url_for_path;

pub async fn deliver_email(
    ConnectInfo(connect_info): ConnectInfo<SocketAddr>,
    Json(deliver_email): Json<DeliverEmailRequest>,
) -> (StatusCode, Json<DeliverEmailResponse>) {
    let email_package = deliver_email.package();
    let Ok(token) = email_package.token().decode() else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::PowFailure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };
    let Ok(pow_result) = email_package.pow_result().decode() else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::PowFailure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };
    let Ok(verify_ip_token) = deliver_email.verify_ip().token().decode() else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::BadRequest.into(),
        );
    };

    let Some(policy) = DB
        .lock()
        .await
        .as_ref()
        .unwrap()
        .get_user_pow_policy(email_package.destination_user())
    else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::UserNotFound.into(),
        );
    };

    // Check against policy
    let Some(classification) = policy.classify(email_package.iters()) else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::DoesNotMeetPolicy(policy).into(),
        );
    };

    // Check POW token
    let hash = email_package.email().hash();
    if let Err(e) = POW_PROVIDER
        .write()
        .await
        .check_pow(token, email_package.iters(), hash, pow_result)
        .await
    {
        return (
            StatusCode::EXPECTATION_FAILED,
            DeliverEmailResponse::PowFailure(e).into(),
        );
    };

    // Check that IP is not spoofed
    match send_post::<_, _, VerifyIpResponse>(
        get_url_for_path(format!("{}:{}", connect_info.ip(), connect_info.port()), FOREIGN_VERIFY_IP_PATH),
        &VerifyIpRequest::new(AuthTokenField::new(&verify_ip_token)),
    )
    .await
    {
        Ok(VerifyIpResponse::Success) => {}
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                DeliverEmailResponse::SenderIpNotAuthed.into(),
            );
        }
    }

    // Check IP against DNS
    #[cfg(not(feature = "no_spf"))]
    {
        let authenticator = MessageAuthenticator::new_google().unwrap();
        let sender = format!(
            "{}@{}",
            email_package.source_user(),
            email_package.source_domain()
        );
        let result = authenticator
            .verify_spf(SpfParameters::verify_mail_from(
                connect_info.ip(),
                "",
                "",
                &sender,
            ))
            .await;

        match result.result() {
            SpfResult::Pass => {}
            _ => {
                return (
                    StatusCode::UNAUTHORIZED,
                    DeliverEmailResponse::SenderIpNotAuthed.into(),
                );
            }
        }
    }

    // Try deliver email (database)
    if DB
        .lock()
        .await
        .as_ref()
        .unwrap()
        .deliver_email(
            email_package.destination_user(),
            deliver_email.source_user(),
            deliver_email.source_domain(),
            email_package.email(),
            classification,
        )
        .is_ok()
    {
        return (
            StatusCode::EXPECTATION_FAILED,
            DeliverEmailResponse::UserNotFound.into(),
        );
    }

    (StatusCode::OK, DeliverEmailResponse::Success.into())
}
