use crate::servers::start_servers;
use h_mail_client::communication::{get_emails, get_emails_s, get_pow_token, get_user_pow_policy, ping_server};
use h_mail_client::interface::routes::native::get_emails::GetEmailsRequest;
use h_mail_client::{
    AuthCredentials, AuthError, reauthenticate, reauthenticate_s, set_server_address,
};
use h_mail_client::interface::routes::foreign::get_pow_token::GetPowTokenRequest;
use h_mail_client::interface::routes::foreign::get_user_pow_policy::GetUserPowPolicyRequest;

mod servers;

#[tokio::test]
async fn test() {
    let mut server = start_servers(2, true).await;
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
    let pow_policy = get_user_pow_policy(&sb, &GetUserPowPolicyRequest::new("test".to_string())).await.unwrap();
    let pow_policy = pow_policy.get().unwrap();
    
    println!("Getting POW token");
    let pow_token = get_pow_token(&sb, &GetPowTokenRequest::new()).await.unwrap();
    let pow_token = pow_token.decode().unwrap();
    
    println!("Cracking POW token");
    
}
