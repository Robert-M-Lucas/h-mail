use crate::config::args::ARGS;
use crate::config::config_file::CONFIG;
use crate::database::Db;
use crate::sending::send_post::send_post;
use crate::shared_resources::POW_PROVIDER;
use axum::Json;
use axum::extract::ConnectInfo;
use axum::http::StatusCode;
use h_mail_interface::interface::fields::auth_token::AuthTokenField;
use h_mail_interface::interface::pow::{PowFailureReason, PowHash};
use h_mail_interface::interface::routes::foreign::deliver_email::{
    DeliverEmailRequest, DeliverEmailResponse,
};
use h_mail_interface::interface::routes::foreign::verify_ip::{
    FOREIGN_VERIFY_IP_PATH, VerifyIpRequest, VerifyIpResponse,
};
use h_mail_interface::shared::get_url_for_path;
use mail_auth::spf::verify::SpfParameters;
use mail_auth::{MessageAuthenticator, SpfResult};
use std::net::{IpAddr, SocketAddr};
use tracing::warn;

pub async fn deliver_email(
    ConnectInfo(connect_info): ConnectInfo<SocketAddr>,
    Json(deliver_email): Json<DeliverEmailRequest>,
) -> (StatusCode, Json<DeliverEmailResponse>) {
    let (
        email_package,
        source_user,
        source_domain,
        destination_user,
        destination_domain,
        verify_ip,
        verify_ip_port,
    ) = deliver_email.dissolve();

    if &destination_domain != CONFIG.domain() {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::WrongDomain.into(),
        );
    }

    let Ok(email_package) = email_package.decode() else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::PowFailure(PowFailureReason::BadRequestCanRetry).into(),
        );
    };

    let hash = email_package.pow_hash();

    let Ok(verify_ip_token) = verify_ip.token().decode() else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::BadRequest.into(),
        );
    };

    let Some(policy) = Db::get_user_pow_policy(&destination_user) else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::UserNotFound.into(),
        );
    };

    // Check against policy
    let Some(classification) = policy.classify(*email_package.pow_result().iters()) else {
        return (
            StatusCode::BAD_REQUEST,
            DeliverEmailResponse::DoesNotMeetPolicy(policy).into(),
        );
    };

    // Check POW token
    let email_package = match POW_PROVIDER
        .write()
        .await
        .check_pow(email_package, *policy.minimum())
        .await
    {
        Ok(email_package) => {
            let Ok(email_package) = email_package.decode() else {
                return (
                    StatusCode::BAD_REQUEST,
                    DeliverEmailResponse::BadRequest.into(),
                );
            };
            email_package
        }
        Err(e) => {
            return (
                StatusCode::EXPECTATION_FAILED,
                DeliverEmailResponse::PowFailure(e).into(),
            );
        }
    };

    // Check that IP is not spoofed
    match send_post::<_, _, VerifyIpResponse>(
        get_url_for_path(
            format!("{}:{}", connect_info.ip(), verify_ip_port),
            FOREIGN_VERIFY_IP_PATH,
        ),
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
    let is_ip = source_domain
        .split(':')
        .next()
        .unwrap()
        .parse::<IpAddr>()
        .is_ok();
    if is_ip {
        warn!(
            "Skipping SPF check as {} is an IP, not domain",
            source_domain
        );
    }
    if !ARGS.no_spf() && !is_ip {
        let authenticator = MessageAuthenticator::new_google().unwrap();
        let sender = format!("{}@{}", &source_user, &source_domain);
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
    if Db::deliver_email(
        &destination_user,
        &source_user,
        &source_domain,
        email_package,
        &hash,
        classification,
    )
    .is_err()
    {
        return (
            StatusCode::EXPECTATION_FAILED,
            DeliverEmailResponse::UserNotFound.into(),
        );
    }

    (StatusCode::OK, DeliverEmailResponse::Success.into())
}
