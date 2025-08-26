use crate::config::config_file::CONFIG;
use crate::database::{Db, UserId};
use crate::receiving::auth_util::auth_header::AuthorizationHeader;
use crate::sending::send_post::send_post;
use crate::shared_resources::VERIFY_IP_TOKEN_PROVIDER;
use axum::Json;
use axum::http::StatusCode;
use futures::future::join_all;
use h_mail_interface::interface::auth::Authorized;
use h_mail_interface::interface::fields::auth_token::AuthTokenDataField;
use h_mail_interface::interface::fields::big_uint::BigUintField;
use h_mail_interface::interface::fields::hmail_address::HmailAddress;
use h_mail_interface::interface::hmail::{Hmail, SendHmailPackage};
use h_mail_interface::interface::pow::{PowClassification, PowHash, PowResultDecoded};
use h_mail_interface::interface::routes::foreign::deliver_hmail::{
    DeliverHmailRequest, DeliverHmailResponse,
};
use h_mail_interface::interface::routes::native::send_hmail::{
    SendHmailRequest, SendHmailResponse, SendHmailResponseAuthed, SendHmailResult,
    SendHmailResultPerDestination,
};
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use tracing::error;

pub async fn send_hmail(
    auth_header: AuthorizationHeader,
    Json(send_hmail): Json<SendHmailRequest>,
) -> (StatusCode, Json<SendHmailResponse>) {
    let Some(user_id) = auth_header.check_access_token().await else {
        return (StatusCode::OK, Authorized::Unauthorized.into());
    };

    let (hmail, bccs, solved_pow_for) = send_hmail.dissolve();

    let hmail_hash = hmail.pow_hash();
    let Ok(hmail_decoded) = hmail.clone().decode() else {
        return (
            StatusCode::OK,
            Authorized::Success(SendHmailResponseAuthed::BadRequest).into(),
        );
    };

    Db::deliver_hmail_to_id(
        user_id,
        hmail_decoded,
        &hmail_hash,
        PowClassification::Personal,
        vec![],
        true,
    )
    .await
    .unwrap();

    // Process solved POWs
    let mut solved_pow_for_decoded = HashMap::with_capacity(solved_pow_for.len());
    for target in solved_pow_for {
        let (solved_for, solved_pow) = target.dissolve();
        let solved_pow = if let Some(solved_pow) = solved_pow {
            let Ok(solved_pow) = solved_pow.decode() else {
                return (
                    StatusCode::OK,
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
        if delivering_to
            .iter()
            .any(|(address, _)| address == recipient)
        {
            return (
                StatusCode::OK,
                Authorized::Success(SendHmailResponseAuthed::DuplicateDestination).into(),
            );
        }

        // Ensure POW exists for destination
        let Some(pow_result) = solved_pow_for.remove(recipient) else {
            return (
                StatusCode::OK,
                Authorized::Success(SendHmailResponseAuthed::MissingPowFor(recipient.clone()))
                    .into(),
            );
        };

        delivering_to.push((recipient.clone(), pow_result));
    }

    let mut parent_hmails = HashMap::new();

    let mut requests = Vec::new();

    for (recipient, pow_result) in &delivering_to {
        requests.push(
            generate_request(user_id, recipient, pow_result, &hmail, &mut parent_hmails).await,
        )
    }

    let results = join_all(
        requests
            .into_iter()
            .map(|(hmail, recipient, context)| send_hmail_to(hmail, recipient, context)),
    )
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
    hmail: Hmail,
    recipient: HmailAddress,
    context: Vec<SendHmailPackage>,
) -> Result<DeliverHmailResponse, ()> {
    // ! Do not lock resource
    let verify_ip_token = VERIFY_IP_TOKEN_PROVIDER
        .write()
        .await
        .get_token(recipient.clone());

    match send_post::<_, _, DeliverHmailResponse>(
        format!("https://{}/foreign/deliver_hmail", &recipient.domain()),
        &DeliverHmailRequest::new(
            hmail,
            recipient,
            AuthTokenDataField::new(&verify_ip_token),
            CONFIG.port(),
            context,
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

async fn get_hmail_by_hash<'a>(
    authed_user: UserId,
    hash: &BigUintField,
    parent_hmails: &'a mut HashMap<String, SendHmailPackage>,
) -> Option<&'a SendHmailPackage> {
    match parent_hmails.entry(hash.as_str().to_string()) {
        Entry::Occupied(entry) => Some(entry.into_mut()), // gives mutable ref
        Entry::Vacant(entry) => {
            let hmail = Db::get_hmail_by_hash(authed_user, hash).await?;
            let (
                _,
                _,
                sender,
                recipients,
                subject,
                sent_at,
                _,
                random_id,
                reply_to,
                ccs,
                parent,
                body,
                _,
                _,
            ) = hmail.dissolve();
            Some(entry.insert(SendHmailPackage::new(
                sender, recipients, subject, sent_at, random_id, reply_to, ccs, parent, body,
            )))
        }
    }
}

async fn generate_request(
    authed_user: UserId,
    recipient: &HmailAddress,
    pow_result: &Option<PowResultDecoded>,
    hmail: &SendHmailPackage,
    parent_hmails: &mut HashMap<String, SendHmailPackage>,
) -> (Hmail, HmailAddress, Vec<SendHmailPackage>) {
    let context = if let Some(parent) = hmail.parent() {
        let mut context = Vec::new();
        let mut parent = parent.clone();

        while let Some(hmail) = get_hmail_by_hash(authed_user, &parent, parent_hmails).await {
            if hmail
                .recipients()
                .iter()
                .map(|u| u.address())
                .contains(recipient)
                || hmail.ccs().iter().map(|u| u.address()).contains(recipient)
                || hmail.sender().address() == recipient
            {
                break;
            }
            context.push(hmail.clone());
            if let Some(new_parent) = hmail.parent() {
                parent = new_parent.clone()
            } else {
                break;
            }
        }

        context
    } else {
        Vec::new()
    };

    (
        Hmail::new(hmail.clone(), pow_result.as_ref().map(|p| p.encode())),
        recipient.clone(),
        context,
    )
}
