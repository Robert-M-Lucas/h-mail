use crate::servers::start_servers;
use h_mail_client::communication::{
    get_emails_s, get_pow_token, get_user_pow_policy, send_email_s,
};
use h_mail_client::interface::routes::foreign::get_user_pow_policy::GetUserPowPolicyRequest;
use h_mail_client::interface::routes::native::get_emails::GetEmailsRequest;
use h_mail_client::solve_pow;
use h_mail_client::{AuthCredentials, reauthenticate_s};
use h_mail_interface::interface::email::{Email, EmailPackage};
use h_mail_interface::interface::fields::big_uint::BigUintField;
use h_mail_interface::interface::pow::{PowClassification, PowHash};
use h_mail_interface::interface::routes::foreign::deliver_email::DeliverEmailResponse;
use h_mail_interface::interface::routes::native::send_email::{
    SendEmailRequest, SendEmailResponseAuthed,
};

mod servers;

#[tokio::test]
async fn test() {
    let server = start_servers(2, true).await;
    let sb = server[0].address();
    let sa = server[1].address();

    println!("Authenticating for both servers");
    reauthenticate_s(
        &sa,
        AuthCredentials::new("test".to_string(), "test".to_string()),
    )
    .await
    .unwrap();
    reauthenticate_s(
        &sb,
        AuthCredentials::new("test".to_string(), "test".to_string()),
    )
    .await
    .unwrap();

    println!("Asserting B's emails are empty");
    let emails_b = get_emails_s(&sb, &GetEmailsRequest::new(-1)).await.unwrap();
    assert!(emails_b.0.is_empty());

    println!("Getting B's POW policy");
    let pow_policy = get_user_pow_policy(&sb, &GetUserPowPolicyRequest::new("test".to_string()))
        .await
        .unwrap();
    let pow_policy = pow_policy.get().unwrap();

    println!("Getting B's POW token");
    let pow_token = get_pow_token(&sb).await.unwrap();
    let pow_token = pow_token.decode().unwrap();

    let email_package = EmailPackage::new("test".to_string(), "testing".to_string());

    println!("Cracking POW token");
    let iters = *pow_policy.minimum();
    let pow_result = solve_pow(&email_package.pow_hash(), pow_token.token(), iters);

    println!("Sending email from A to B");
    let r = send_email_s(
        &sa,
        &SendEmailRequest::new(
            Email::new(
                email_package,
                iters,
                BigUintField::new(pow_token.token()),
                BigUintField::new(&pow_result),
            ),
            sb.clone(),
        ),
    )
    .await
    .unwrap();

    match r {
        SendEmailResponseAuthed::DeliverResponse(DeliverEmailResponse::Success) => {}
        _ => panic!("{r:?}"),
    }

    println!("Re-getting B's emails");
    let emails_b = get_emails_s(&sb, &GetEmailsRequest::new(-1)).await.unwrap();

    assert_eq!(emails_b.0.len(), 1);
    let email = emails_b.0.into_iter().next().unwrap();
    assert_eq!(email.source().split('@').next().unwrap(), "test");
    assert_eq!(email.email(), "testing");
    assert_eq!(*email.pow_classification(), PowClassification::Minimum);
}
