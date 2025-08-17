use crate::config::config_file::CONFIG;
use crate::database::Db;
use crate::receiving::auth_util::auth_header::AuthorizationHeader;
use crate::sending::send_post::send_post;
use crate::shared_resources::VERIFY_IP_TOKEN_PROVIDER;
use axum::Json;
use axum::http::StatusCode;
use futures::future::join_all;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::hmail::{Hmail, SendHmailPackage};
use h_mail_interface::interface::fields::auth_token::AuthTokenDataField;
use h_mail_interface::interface::pow::PowResultDecoded;
use h_mail_interface::interface::routes::foreign::deliver_hmail::{DeliverHmailRequest, DeliverHmailResponse};
use h_mail_interface::interface::routes::native::send_hmail::{
    SendHmailRequest, SendHmailResponse, SendHmailResponseAuthed, SendHmailResult,
    SendHmailResultPerDestination,
};
use itertools::Itertools;
use std::collections::HashMap;
use tracing::error;
use h_mail_interface::interface::fields::hmail_address::HmailAddress;

pub async fn send_hmail(
    auth_header: AuthorizationHeader,
    Json(send_hmail): Json<SendHmailRequest>,
) -> (StatusCode, Json<SendHmailResponse>) {
    let Some(user_id) = auth_header.check_access_token().await else {
        return (StatusCode::UNAUTHORIZED, Authorized::Unauthorized.into());
    };

    let username = Db::get_username_from_id(user_id).unwrap();

    let (hmail, bccs, solved_pow_for) = send_hmail.dissolve();

    // Process solved POWs
    let mut solved_pow_for_decoded = HashMap::with_capacity(solved_pow_for.len());
    for target in solved_pow_for {
        let (solved_for, solved_pow) = target.dissolve();
        let solved_pow = if let Some(solved_pow) = solved_pow {
            let Ok(solved_pow) = solved_pow.decode() else {
                return (
                    StatusCode::EXPECTATION_FAILED,
                    Authorized::Success(SendHmailResponseAuthed::BadRequest).into(),
                );
            };
            Some(solved_pow)
        } else {
            None
        };

        solved_pow_for_decoded.insert(solved_for, solved_pow);
    }
    let mut solved_pow_for = solved_pow_for_decoded;

    // Map destinations to POWs
    let mut delivering_to: Vec<(HmailAddress, Option<PowResultDecoded>)> =
        Vec::with_capacity(hmail.recipients().len() + hmail.ccs().len() + bccs.len());
    for recipient in hmail
        .recipients()
        .iter()
        .chain(hmail.ccs().iter())
        .map(|to| to.address())
        .chain(&bccs)
    {
        // Check for duplicate destination
        if delivering_to.iter().any(|(address, _)| address == recipient) {
            return (
                StatusCode::EXPECTATION_FAILED,
                Authorized::Success(SendHmailResponseAuthed::DuplicateDestination).into(),
            );
        }

        // Ensure POW exists for destination
        let Some(pow_result) = solved_pow_for.remove(recipient) else {
            return (
                StatusCode::EXPECTATION_FAILED,
                Authorized::Success(SendHmailResponseAuthed::MissingPowFor(recipient.clone())).into(),
            );
        };

        delivering_to.push((recipient.clone(), pow_result));
    }

    let results = join_all(delivering_to.iter().map(|(recipient, pow_result)| {
        send_hmail_to(&username, recipient, pow_result, &hmail)
    }))
    .await
    .into_iter()
    .zip(delivering_to.iter().map(|(recipient, _)| recipient));

    (
        StatusCode::OK,
        Authorized::Success(SendHmailResponseAuthed::DeliverResponse(
            results
                .map(|(result, hmail)| {
                    let r = if let Ok(result) = result {
                        SendHmailResult::DeliveryResult(result)
                    } else {
                        SendHmailResult::Failed
                    };

                    SendHmailResultPerDestination::new(hmail.clone(), r)
                })
                .collect_vec(),
        ))
        .into(),
    )
}

async fn send_hmail_to(
    sender_username: &str,
    recipient: &HmailAddress,
    pow_result: &Option<PowResultDecoded>,
    hmail: &SendHmailPackage,
) -> Result<DeliverHmailResponse, ()> {
    // ! Do not lock resource
    let verify_ip_token = VERIFY_IP_TOKEN_PROVIDER.write().await.get_token(());

    let Ok(sender_address) = HmailAddress::from_username_domain(sender_username, CONFIG.domain()) else {
        return Err(())
    };

    match send_post::<_, _, DeliverHmailResponse>(
        format!("https://{}/foreign/deliver_hmail", &recipient.domain()),
        &DeliverHmailRequest::new(
            Hmail::new(hmail.clone(), pow_result.as_ref().map(|p| p.encode())),
            sender_address,
            recipient.clone(),
            AuthTokenDataField::new(&verify_ip_token),
            CONFIG.port(),
        ),
    )
    .await
    {
        Ok(r) => Ok(r),
        Err(e) => {
            error!("Failed to send hmail: {e:?}");
            Err(())
        }
    }
}
